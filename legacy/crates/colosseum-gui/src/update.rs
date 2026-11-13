//! "Check for updates" against GitHub Releases.
//!
//! One anonymous GET to the release list, filtered to stable `vX.Y.Z` tags
//! and compared version-wise against `CARGO_PKG_VERSION`. The request runs on a
//! detached thread; the About dialog polls the receiver each frame. Every
//! failure path degrades gracefully: a 404 (nothing published yet) counts as
//! up to date, network errors surface as a gentle "couldn't check" line, and
//! nothing panics.

use std::sync::mpsc::{Receiver, channel};

/// The human-facing releases page (fallback link target).
pub const RELEASES_URL: &str = "https://github.com/maelic13/colosseum/releases";
/// Known limit, deliberately not fixed: one unpaginated page holds the newest
/// hundred releases. The repository holds the desktop application alone and
/// adds a handful of releases a year, so the newest cannot fall off the page
/// for many years; add pagination when the count passes fifty.
const API_URL: &str = "https://api.github.com/repos/maelic13/colosseum/releases?per_page=100";

/// Outcome of one update check.
pub enum UpdateStatus {
    UpToDate,
    UpdateAvailable {
        /// Version string without the `v` prefix, e.g. `1.2.0`.
        version: String,
        /// The release page to open in the browser.
        url: String,
    },
    /// The check itself failed (offline, GitHub unreachable…).
    Failed,
}

/// A running background check; poll it each frame until it yields.
pub struct UpdateCheck {
    rx: Receiver<UpdateStatus>,
}

impl UpdateCheck {
    /// Spawn the check on a detached thread and return the handle to poll.
    #[must_use]
    pub fn start() -> Self {
        let (tx, rx) = channel();
        std::thread::spawn(move || {
            let _ = tx.send(check());
        });
        Self { rx }
    }

    /// The result, once the background thread has one.
    #[must_use]
    pub fn poll(&self) -> Option<UpdateStatus> {
        self.rx.try_recv().ok()
    }
}

fn check() -> UpdateStatus {
    match fetch_latest() {
        Ok(Some((tag, url))) => {
            let latest = release_version(&tag);
            let current = parse_version(env!("CARGO_PKG_VERSION"));
            match (latest, current) {
                (Some(l), Some(c)) if l > c => UpdateStatus::UpdateAvailable {
                    version: tag.strip_prefix('v').unwrap_or(&tag).to_string(),
                    url,
                },
                // Equal, older (dev build ahead of the release), or an
                // unparseable tag all read as "nothing newer out there".
                _ => UpdateStatus::UpToDate,
            }
        }
        // No release published yet — the current build is all there is.
        Ok(None) => UpdateStatus::UpToDate,
        Err(_) => UpdateStatus::Failed,
    }
}

/// `Ok(Some((tag_name, html_url)))` for the latest release, `Ok(None)` when
/// the repository has no releases (404).
fn fetch_latest() -> Result<Option<(String, String)>, Box<dyn std::error::Error>> {
    let config = ureq::Agent::config_builder()
        .timeout_global(Some(std::time::Duration::from_secs(10)))
        .build();
    let agent: ureq::Agent = config.into();
    let response = agent
        .get(API_URL)
        .header(
            "User-Agent",
            concat!("colosseum/", env!("CARGO_PKG_VERSION")),
        )
        .header("Accept", "application/vnd.github+json")
        .call();
    let mut response = match response {
        Ok(r) => r,
        Err(ureq::Error::StatusCode(404)) => return Ok(None),
        Err(e) => return Err(e.into()),
    };
    let body = response.body_mut().read_to_string()?;
    let releases: serde_json::Value = serde_json::from_str(&body)?;
    let releases = releases
        .as_array()
        .ok_or("release API did not return an array")?;
    Ok(select_latest_release(releases))
}

fn select_latest_release(releases: &[serde_json::Value]) -> Option<(String, String)> {
    releases
        .iter()
        // A draft is not published, and a prerelease is not offered to
        // everyone: the tag shape alone cannot tell, because a release marked
        // prerelease on GitHub may still carry a clean `v1.3.0` tag.
        .filter(|release| !flag(release, "draft") && !flag(release, "prerelease"))
        .filter_map(|release| {
            let tag = release.get("tag_name")?.as_str()?;
            let version = release_version(tag)?;
            let url = release
                .get("html_url")
                .and_then(|value| value.as_str())
                .unwrap_or(RELEASES_URL);
            Some((version, tag.to_owned(), url.to_owned()))
        })
        .max_by_key(|(version, _, _)| *version)
        .map(|(_, tag, url)| (tag, url))
}

/// One boolean field of a release object, absent or malformed counting as
/// false — the API always sends both, and a missing flag must not hide a
/// release that is really there.
fn flag(release: &serde_json::Value, name: &str) -> bool {
    release
        .get(name)
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
}

/// The version of a stable release tag, `vX.Y.Z` and nothing else.
///
/// From 1.2.0 every release of this repository, 2.0.0 included, is tagged
/// `vX.Y.Z` (ADR-0001). The `gui-v` and `cli-v` tags of the shared repository
/// the application came from are never offered, and neither is a tag with a
/// prerelease or build suffix.
fn release_version(tag: &str) -> Option<(u64, u64, u64)> {
    let version = tag.strip_prefix('v')?;
    if version.contains('-') || version.contains('+') || version.split('.').count() != 3 {
        return None;
    }
    parse_version(version)
}

/// Parse `v1.2.3` / `1.2.3` (pre-release/build suffixes ignored) into a
/// comparable triple.
fn parse_version(s: &str) -> Option<(u64, u64, u64)> {
    let core = s.trim().trim_start_matches('v').split(['-', '+']).next()?;
    let mut parts = core.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next().unwrap_or("0").parse().ok()?;
    Some((major, minor, patch))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_tags_with_and_without_prefix() {
        assert_eq!(parse_version("v1.2.3"), Some((1, 2, 3)));
        assert_eq!(parse_version("1.0.0"), Some((1, 0, 0)));
        assert_eq!(parse_version("v2.0.0-rc.1"), Some((2, 0, 0)));
        assert_eq!(parse_version("v1.2"), Some((1, 2, 0)));
        assert_eq!(parse_version("nightly"), None);
    }

    #[test]
    fn ordering_matches_semver() {
        assert!(parse_version("v1.0.1") > parse_version("v1.0.0"));
        assert!(parse_version("v1.10.0") > parse_version("v1.9.9"));
        assert!(parse_version("v1.0.0") == parse_version("1.0.0"));
    }

    #[test]
    fn selects_only_stable_plain_v_tags() {
        let releases = serde_json::json!([
            {"tag_name":"cli-v9.0.0","html_url":"cli"},
            {"tag_name":"gui-v8.0.0","html_url":"old-lane"},
            {"tag_name":"v1.3.0-rc.1","html_url":"rc"},
            {"tag_name":"v1.2.0","html_url":"new"},
            {"tag_name":"v1.1.9","html_url":"older"},
            {"tag_name":"v2.0.0","html_url":"draft","draft":true}
        ]);
        assert_eq!(
            select_latest_release(releases.as_array().unwrap()),
            Some(("v1.2.0".into(), "new".into()))
        );
    }

    /// 2.0.0 is offered to 1.x by the same rule: a higher plain `v` tag.
    #[test]
    fn a_new_major_version_is_offered() {
        let releases = serde_json::json!([
            {"tag_name":"v1.2.0","html_url":"one"},
            {"tag_name":"v2.0.0","html_url":"two"}
        ]);
        assert_eq!(
            select_latest_release(releases.as_array().unwrap()),
            Some(("v2.0.0".into(), "two".into()))
        );
    }

    #[test]
    fn only_a_complete_stable_tag_has_a_version() {
        assert_eq!(release_version("v1.2.0"), Some((1, 2, 0)));
        assert_eq!(release_version("v10.0.1"), Some((10, 0, 1)));
        for tag in [
            "gui-v1.2.0",
            "cli-v1.2.0",
            "1.2.0",
            "v1.2",
            "v1.2.0-rc.1",
            "v1.2.0+build",
            "vnext",
        ] {
            assert_eq!(release_version(tag), None, "{tag}");
        }
    }

    /// The tag shape cannot tell a prerelease from a stable release: a
    /// release marked prerelease on GitHub may carry a perfectly clean `v`
    /// tag. Only the flag says so, and someone running the stable build must
    /// not be sent to it.
    #[test]
    fn a_release_marked_prerelease_is_not_offered_however_its_tag_reads() {
        let releases = serde_json::json!([
            {"tag_name":"v1.2.0","html_url":"stable","draft":false,"prerelease":false},
            {"tag_name":"v1.3.0","html_url":"beta","draft":false,"prerelease":true}
        ]);
        assert_eq!(
            select_latest_release(releases.as_array().unwrap()),
            Some(("v1.2.0".into(), "stable".into()))
        );

        // Nothing but prereleases is the same as nothing: up to date.
        let only_beta = serde_json::json!([
            {"tag_name":"v1.3.0","html_url":"beta","prerelease":true}
        ]);
        assert_eq!(select_latest_release(only_beta.as_array().unwrap()), None);

        // A release object that omits the flags is still a release.
        let bare = serde_json::json!([{"tag_name":"v1.2.0","html_url":"stable"}]);
        assert_eq!(
            select_latest_release(bare.as_array().unwrap()),
            Some(("v1.2.0".into(), "stable".into()))
        );
    }
}
