# ADR-0001: 1.x maintained here, and the swap at the split

- **Status:** Accepted. Revised the same day, before any of it was acted
  on: plain `v` tags from the first release here instead of `gui-v` tags and
  a bridge release, and squash pull requests into `master` as Rarog
  delivers (decisions 4, 6 and 8)
- **Date:** 2026-11-13
- **Supersedes:** in the CLI repository's
  [ADR-0012](https://github.com/maelic13/colosseum-cli/blob/dev/docs/architecture/adr/0012-names-and-repositories-after-the-split.md),
  the timing of the swap (*Branches and order*), the bridge release, *No 1.x
  patch after the swap*, and the 1.x half of *The 1.x releases and tags are
  deleted*. Its names are kept unchanged
- **Relates to:** PLAN §G8, §G9 (Phase 0, M8, M9); GUIDE 0.1–0.3

## Context

CLI ADR-0012 leaves the 1.x application (egui, Rust) on the CLI
repository's `main` until the end of the programme. That has three costs.
The CLI repository carries a desktop product, its release lane and its
packaging for as long as 2.0 takes. The CLI's `main` and `dev` have
diverged: `dev` retired the egui application at CLI 11.2.1 on 2026-09-29,
while the CLI releases since 0.2.0 shipped from `main`, which still holds
it. And the two products are split in the plans but not in the
repositories.

ADR-0012 rejected an early swap for one reason. After the swap, 1.x's
updater reads `maelic13/colosseum` for `gui-v` tags, so a 1.x patch
published from the CLI repository would be invisible to it. That reason
holds only while 1.x is released from the CLI repository.

On 2026-11-13 the maintainer chose to finish the split now: the 1.x
application comes to this repository, the CLI repository becomes the CLI
alone, and the 2.0 plan is unchanged.

## Decision

1. **1.x moves here, in `legacy/`.** A plain copy of the CLI repository's
   `main` at `8fa63cc` (CLI 0.5.0): the `colosseum-gui` crate (GUI 1.1.0 and
   a three-line adapter change for CLI 0.5.0) and the crates it builds
   against, `colosseum-application`, `-core`, `-engine` and `-uci`,
   duplicated as they are; its release lane, packaging, release tooling and
   changelog. `legacy/` is its own Cargo workspace. Its history stays in the
   CLI repository, whose commits are never deleted.
2. **The duplicates are frozen.** `legacy/` receives fixes that 1.x users
   need and nothing else. No feature is added to 1.x, and nothing is synced
   from the CLI crates unless a 1.x fix requires it.
3. **The swap happens at the split**, not at CLI 14.1. The maintainer renames
   `maelic13/colosseum` to `maelic13/colosseum-cli`, then at once
   `maelic13/colosseum-gui` to `maelic13/colosseum`, with the same-day
   checklist of ADR-0012 (clones' `origin`, Rarog's CLI download address,
   both repositories' links).
4. **Releases come from here, tagged `vX.Y.Z`**, for 1.x and for 2.0 alike.
   The first is 1.x's next version, 1.2.0, whose update check reads plain
   `v` tags. A GitHub release has exactly one tag, so serving 1.1.0's check,
   which accepts only `gui-v` tags, would take a second release of the same
   assets; the maintainer declined it. Installed 1.1.0 copies are therefore
   never offered an update and their users download 1.2.0 once by hand.
   Every version from 1.2.0 on sees every later release, 2.0.0 included,
   with no bridge.
5. **The CLI repository loses 1.x.** Its `dev` is brought up to date with
   its `main` and merged into it, which removes the egui application
   (retired from its `dev` at CLI 11.2.1) and the GUI release lane; that is
   the CLI repository's work. Its published desktop releases and tags
   (`v1.0.0-rc.1` to `v1.0.2`, `gui-v1.1.0`) are deleted by the maintainer
   once 1.2.0 is out here, so the 1.x downloads are never absent, and it
   keeps only CLI releases.
6. **2.0 is unchanged** but for its tag: still a client of the CLI, copying
   nothing from `legacy/`, released as `v2.0.0` with no bridge. `legacy/`
   and its 1.x release lane are removed in the change that releases 2.0.0
   (5.1).
7. **The inventory reads `legacy/`.** The 1.x source no longer needs a CLI
   checkout. `legacy/` differs from `gui-v1.1.0` (`ba30829`), which the
   inventory draft was made at, only by the adapter change and the CLI 0.5.0
   crates; the draft stands.

8. **Delivery as Rarog's.** `master` is the trunk. Each larger piece of work
   lives on its own short-lived branch with its step commits and reaches
   `master` as one squash commit through a pull request, which runs CI and
   the release candidate; the maintainer squash-merges it when both are
   green. A release is a pull request that dates the changelog and sets the
   version, then a `vX.Y.Z` tag pushed on the merged `master`, which the
   release workflow validates, builds and publishes. `dev` becomes the first
   such branch.

## Consequences

- Each repository holds one product. The CLI's `main` and `dev` no longer
  differ by a desktop application; reconciling them is the CLI repository's
  business.
- From 1.2.0 on, 1.x fixes and then 2.0.0 reach installed copies through
  their own update check. 1.1.0 copies are left to a manual download; that
  is the accepted cost of one tag scheme without a bridge.
- Between the swap and 1.2.0 here, nothing new is offered anywhere; 1.2.0
  should follow the swap closely.
- This repository has a Rust workspace beside the TypeScript application
  until 2.0.0. They share nothing: CI runs them as separate jobs, and the 2.1
  verification baseline covers the TypeScript application; `legacy/` keeps
  the checks it had in the CLI repository.
- The engine-playing code exists twice until 2.0.0: the frozen copy in
  `legacy/` and the CLI's. That is accepted; only 1.x uses the copy.

## Alternatives considered

### Keep 1.x in the CLI repository until 2.0 (ADR-0012 as it was)

Not taken. It leaves the products split on paper only and keeps the CLI's
`main` carrying the desktop application.

### Move 1.x but keep the swap at the end

Rejected. Patches from `maelic13/colosseum-gui` would be invisible to the
1.x updater, which reads `maelic13/colosseum`, until the swap.

### Build 1.x against the CLI crates as a dependency

Rejected. A git dependency on the CLI repository would tie 1.x to the CLI's
API as it changes for 2.0, which is what the frozen copy avoids.

### `gui-v` tags, or both tags on each release

Not taken. A GitHub release carries one tag, so both schemes means two
releases of the same assets each time. The `gui-v` prefix only ever served
1.1.0's check, and the maintainer accepted losing it for one tag scheme.

### Import with history

Not taken. A history-preserving import brings the shared crates' history
too, and the full history stays readable in the CLI repository.
