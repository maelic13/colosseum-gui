use std::fs;
use std::path::Path;

use semver::Version;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReleaseMetadata {
    pub validation: &'static str,
    pub product: &'static str,
    pub package: &'static str,
    pub version: String,
    pub tag: String,
    pub changelog: &'static str,
    pub artifact_stem: String,
    pub prerelease: bool,
}

#[derive(Debug, Clone, Copy)]
struct Product {
    name: &'static str,
    package: &'static str,
    manifest: &'static str,
    changelog: &'static str,
    artifact_name: &'static str,
    tag_prefix: &'static str,
}

#[derive(Debug, Error)]
pub enum MetadataError {
    #[error("tag must be vX.Y.Z; the CLI is released from maelic13/colosseum-cli")]
    TagPrefix,
    #[error("invalid release semantic version: {0}")]
    Semver(#[from] semver::Error),
    #[error("release versions must not contain build metadata")]
    BuildMetadata,
    #[error("could not read {path}: {source}")]
    Read {
        path: String,
        source: std::io::Error,
    },
    #[error("invalid package manifest {path}: {source}")]
    Manifest {
        path: String,
        source: toml::de::Error,
    },
    #[error("package {package} has version {actual}, tag requests {requested}")]
    VersionMismatch {
        package: &'static str,
        actual: String,
        requested: Version,
    },
    #[error("{changelog} has no release heading for {version}")]
    MissingNotes {
        changelog: &'static str,
        version: Version,
    },
    #[error(
        "{changelog}'s heading for {version} is not dated: write `## [{version}] — YYYY-MM-DD`"
    )]
    UndatedNotes {
        changelog: &'static str,
        version: Version,
    },
    #[error("product must be gui; the CLI is released from maelic13/colosseum-cli")]
    Product,
    #[error("unsupported target triple: {0}")]
    Target(String),
    #[error("could not copy {path}: {source}")]
    Copy {
        path: String,
        source: std::io::Error,
    },
}

pub fn validate(root: &Path, tag: &str) -> Result<ReleaseMetadata, MetadataError> {
    let Some(raw_version) = tag.strip_prefix('v') else {
        return Err(MetadataError::TagPrefix);
    };
    let product = product("gui")?;

    let version = Version::parse(raw_version)?;
    if !version.build.is_empty() {
        return Err(MetadataError::BuildMetadata);
    }
    let actual = package_version(root, product)?;
    if actual != version.to_string() {
        return Err(MetadataError::VersionMismatch {
            package: product.package,
            actual,
            requested: version,
        });
    }

    let heading = format!("## [{}]", version);
    let notes = read(root, product.changelog)?;
    let Some(line) = notes
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with(&heading))
    else {
        return Err(MetadataError::MissingNotes {
            changelog: product.changelog,
            version,
        });
    };
    // A release's section is dated in the commit that is tagged, so the
    // notes a release publishes say when it was cut.
    if !is_dated(&line[heading.len()..]) {
        return Err(MetadataError::UndatedNotes {
            changelog: product.changelog,
            version,
        });
    }

    Ok(metadata(product, version, tag.to_owned(), "release"))
}

/// Whether what follows a release heading's `## [X.Y.Z]` is ` — YYYY-MM-DD`
/// (an em dash or a hyphen).
fn is_dated(rest: &str) -> bool {
    let rest = rest.trim_start();
    let Some(date) = rest
        .strip_prefix('—')
        .or_else(|| rest.strip_prefix('-'))
        .map(str::trim)
    else {
        return false;
    };
    let bytes = date.as_bytes();
    bytes.len() == 10
        && bytes.iter().enumerate().all(|(index, byte)| match index {
            4 | 7 => *byte == b'-',
            _ => byte.is_ascii_digit(),
        })
}

/// Validate the package-owned version for an immutable, unpublished candidate.
///
/// Candidates deliberately do not require final changelog headings or tags.
/// They are workflow artifacts tied to a commit SHA, never GitHub Releases.
/// A development version between releases (`X.Y.Z-dev`) is a candidate like
/// any other.
pub fn candidate(root: &Path, name: &str) -> Result<ReleaseMetadata, MetadataError> {
    let product = product(name)?;
    let version = Version::parse(&package_version(root, product)?)?;
    if !version.build.is_empty() {
        return Err(MetadataError::BuildMetadata);
    }
    let proposed_tag = format!("{}{}", product.tag_prefix, version);
    Ok(metadata(product, version, proposed_tag, "candidate"))
}

/// Extract only the selected product/version release-note section.
pub fn release_notes(root: &Path, tag: &str) -> Result<String, MetadataError> {
    let metadata = validate(root, tag)?;
    let text = read(root, metadata.changelog)?;
    let heading = format!("## [{}]", metadata.version);
    let mut collecting = false;
    let mut lines = Vec::new();
    for line in text.lines() {
        if line.trim().starts_with("## [") {
            if collecting {
                break;
            }
            collecting = line.trim().starts_with(&heading);
            continue;
        }
        if collecting {
            lines.push(line);
        }
    }
    // A changelog may separate versions with a horizontal rule; it belongs
    // between sections, not at the end of one release's notes.
    while lines
        .last()
        .is_some_and(|line| line.trim().is_empty() || line.trim() == "---")
    {
        lines.pop();
    }
    Ok(lines.join("\n").trim().to_owned() + "\n")
}

/// Map a Rust target triple to the artifact's platform and architecture
/// tokens.
///
/// One mapping for every caller — the build entry point, the
/// release workflows and the archive smoke — so an artifact's name can never
/// disagree with the triple it was built for.
pub fn target_platform(target: &str) -> Result<(&'static str, &'static str), MetadataError> {
    let platform = if target.contains("windows") {
        "windows"
    } else if target.contains("linux") {
        "linux"
    } else if target.contains("darwin") {
        "macos"
    } else {
        return Err(MetadataError::Target(target.to_owned()));
    };
    let arch = if target.starts_with("x86_64") {
        "x64"
    } else if target.starts_with("aarch64") {
        "arm64"
    } else {
        return Err(MetadataError::Target(target.to_owned()));
    };
    Ok((platform, arch))
}

fn metadata(
    product: Product,
    version: Version,
    tag: String,
    validation: &'static str,
) -> ReleaseMetadata {
    ReleaseMetadata {
        validation,
        product: product.name,
        package: product.package,
        version: version.to_string(),
        tag,
        changelog: product.changelog,
        artifact_stem: format!("{}-{version}", product.artifact_name),
        prerelease: !version.pre.is_empty(),
    }
}

fn product(name: &str) -> Result<Product, MetadataError> {
    match name {
        "gui" => Ok(Product {
            name: "gui",
            package: "colosseum-gui",
            manifest: "crates/colosseum-gui/Cargo.toml",
            changelog: "CHANGELOG.md",
            artifact_name: "colosseum-gui",
            tag_prefix: "v",
        }),
        _ => Err(MetadataError::Product),
    }
}

fn package_version(root: &Path, product: Product) -> Result<String, MetadataError> {
    let manifest_text = read(root, product.manifest)?;
    let document: toml::Value =
        toml::from_str(&manifest_text).map_err(|source| MetadataError::Manifest {
            path: product.manifest.into(),
            source,
        })?;
    Ok(document["package"]["version"]
        .as_str()
        .unwrap_or_default()
        .to_owned())
}

fn read(root: &Path, relative: &str) -> Result<String, MetadataError> {
    fs::read_to_string(root.join(relative)).map_err(|source| MetadataError::Read {
        path: relative.into(),
        source,
    })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn fixture() -> tempfile::TempDir {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("crates/colosseum-gui")).unwrap();
        fs::write(
            root.path().join("crates/colosseum-gui/Cargo.toml"),
            "[package]\nname = \"colosseum-gui\"\nversion = \"1.2.3\"\n",
        )
        .unwrap();
        fs::write(
            root.path().join("CHANGELOG.md"),
            "## [1.2.3] — 2026-01-02\n",
        )
        .unwrap();
        root
    }

    /// Releases here are plain `vX.Y.Z` (ADR-0001). The CLI's lane left with
    /// the CLI, and the `gui-v` lane ended at 1.1.0: both are refused rather
    /// than validated against a missing package or the wrong history.
    #[test]
    fn routes_the_gui_tag_and_refuses_the_cli_lane() {
        let root = fixture();
        let gui = validate(root.path(), "v1.2.3").unwrap();
        assert_eq!(gui.package, "colosseum-gui");
        assert_eq!(gui.artifact_stem, "colosseum-gui-1.2.3");
        assert_eq!(gui.validation, "release");
        for tag in ["cli-v1.2.3", "gui-v1.2.3"] {
            assert!(
                matches!(validate(root.path(), tag), Err(MetadataError::TagPrefix)),
                "{tag}"
            );
        }
        assert!(matches!(
            candidate(root.path(), "cli"),
            Err(MetadataError::Product)
        ));
    }

    /// One naming scheme. The tokens are the artifact's, not the triple's: a
    /// download called `…-windows-x64.zip` says what it runs on, where
    /// `…-x86_64-pc-windows-msvc` says how it was compiled.
    #[test]
    fn every_released_triple_maps_to_the_one_naming_scheme() {
        for (target, expected) in [
            ("x86_64-pc-windows-msvc", ("windows", "x64")),
            ("aarch64-pc-windows-msvc", ("windows", "arm64")),
            ("x86_64-unknown-linux-gnu", ("linux", "x64")),
            ("aarch64-unknown-linux-gnu", ("linux", "arm64")),
            ("x86_64-apple-darwin", ("macos", "x64")),
            ("aarch64-apple-darwin", ("macos", "arm64")),
        ] {
            assert_eq!(target_platform(target).unwrap(), expected, "{target}");
        }
        for unsupported in ["i686-pc-windows-msvc", "x86_64-unknown-freebsd", "nonsense"] {
            assert!(matches!(
                target_platform(unsupported),
                Err(MetadataError::Target(_))
            ));
        }
    }

    #[test]
    fn rejects_wrong_version_and_build_metadata() {
        let root = fixture();
        assert!(matches!(
            validate(root.path(), "v1.2.4"),
            Err(MetadataError::VersionMismatch { .. })
        ));
        assert!(matches!(
            validate(root.path(), "v1.2.3+local"),
            Err(MetadataError::BuildMetadata)
        ));
    }

    #[test]
    fn a_tag_needs_its_section() {
        let root = fixture();
        fs::write(root.path().join("CHANGELOG.md"), "## [Unreleased]\n").unwrap();
        assert!(matches!(
            validate(root.path(), "v1.2.3"),
            Err(MetadataError::MissingNotes { .. })
        ));
    }

    #[test]
    fn a_tag_needs_its_section_dated() {
        let root = fixture();
        for heading in ["## [1.2.3]", "## [1.2.3] — soon", "## [1.2.3] — 2026-1-2"] {
            fs::write(root.path().join("CHANGELOG.md"), format!("{heading}\n")).unwrap();
            assert!(
                matches!(
                    validate(root.path(), "v1.2.3"),
                    Err(MetadataError::UndatedNotes { .. })
                ),
                "{heading}"
            );
        }
    }

    #[test]
    fn a_development_version_is_a_candidate() {
        let root = fixture();
        fs::write(
            root.path().join("crates/colosseum-gui/Cargo.toml"),
            "[package]\nname = \"colosseum-gui\"\nversion = \"1.3.0-dev\"\n",
        )
        .unwrap();
        let gui = candidate(root.path(), "gui").unwrap();
        assert_eq!(gui.version, "1.3.0-dev");
        assert!(gui.prerelease);
    }

    #[test]
    fn candidate_uses_manifest_without_claiming_release_readiness() {
        let root = fixture();
        fs::write(root.path().join("CHANGELOG.md"), "## [Unreleased]\n").unwrap();
        let gui = candidate(root.path(), "gui").unwrap();
        assert_eq!(gui.validation, "candidate");
        assert_eq!(gui.tag, "v1.2.3");
        assert!(matches!(
            candidate(root.path(), "other"),
            Err(MetadataError::Product)
        ));
    }

    #[test]
    fn extracts_only_the_matching_version_notes() {
        let root = fixture();
        fs::write(
            root.path().join("CHANGELOG.md"),
            "# GUI\n\n## [Unreleased]\n\nFuture.\n\n## [1.2.3] — 2026-01-02\n\nGUI notes.\n\n## [1.2.2] — 2026-01-01\n\nOld.\n",
        )
        .unwrap();
        assert_eq!(
            release_notes(root.path(), "v1.2.3").unwrap(),
            "GUI notes.\n"
        );
    }

    #[test]
    fn notes_end_before_the_rule_that_separates_versions() {
        let root = fixture();
        fs::write(
            root.path().join("CHANGELOG.md"),
            "# GUI\n\n---\n\n## [1.2.3] - 2026-01-01\n\nGUI notes.\n\n---\n\n## [1.2.2]\n\nOld.\n",
        )
        .unwrap();
        assert_eq!(
            release_notes(root.path(), "v1.2.3").unwrap(),
            "GUI notes.\n"
        );
    }
}
