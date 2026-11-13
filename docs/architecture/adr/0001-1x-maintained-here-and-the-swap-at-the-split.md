# ADR-0001: 1.x maintained here, and the swap at the split

- **Status:** Accepted
- **Date:** 2026-11-13
- **Supersedes:** in the CLI repository's
  [ADR-0012](https://github.com/maelic13/colosseum/blob/dev/docs/architecture/adr/0012-names-and-repositories-after-the-split.md),
  the timing of the swap (*Branches and order*), *No 1.x patch after the
  swap*, and the 1.x half of *The 1.x releases and tags are deleted*. Its
  names, the bridge release and the plain `v` tags are kept unchanged
- **Relates to:** PLAN §G8, §G9 (Phase 0, M8, M9); GUIDE 0.1, 0.2

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
4. **1.x releases come from here** as `gui-v1.x.y` tags, from `master`. 1.x's
   updater reads `maelic13/colosseum` for `gui-v` tags, so after the swap it
   finds them without any change to 1.x. `dev` is merged into `master` at the
   split and again for each release, so `master` holds what is released.
5. **The CLI repository loses 1.x** from `main` once this repository builds
   it (CLI step, recorded there). Its published 1.x releases and tags are
   deleted by the maintainer once the first 1.x release from here is out, so
   the 1.x downloads are never absent.
6. **2.0 is unchanged.** It is still a client of the CLI and copies nothing
   from `legacy/`. Its first release is still `v2.0.0` with the `gui-v2.0.0`
   bridge release, because 1.x's updater accepts only `gui-v` tags.
   `legacy/` and its release lane are removed in the change that releases
   2.0.0 (5.1).
7. **The inventory reads `legacy/`.** The 1.x source no longer needs a CLI
   checkout. `legacy/` differs from `gui-v1.1.0` (`ba30829`), which the
   inventory draft was made at, only by the adapter change and the CLI 0.5.0
   crates; the draft stands.

## Consequences

- Each repository holds one product. The CLI's `main` and `dev` no longer
  differ by a desktop application; reconciling them is the CLI repository's
  business.
- 1.x patches reach installed copies through their own updater for the whole
  programme, which ADR-0012 gave up.
- Between the swap and the first 1.x release here, 1.x's update check finds
  no release and reports "up to date". The first release should follow the
  swap closely.
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

### Import with history

Not taken. A history-preserving import brings the shared crates' history
too, and the full history stays readable in the CLI repository.
