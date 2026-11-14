# Colosseum desktop application — agent instructions

Read this file, [`PLAN.md`](PLAN.md) and [`GUIDE.md`](GUIDE.md) before any
work. `PLAN.md` is the binding specification of the desktop application and
its relationship to the Colosseum CLI; `GUIDE.md` is the ordered tracker.

## What this repository is

The desktop application is a **client of the Colosseum CLI**. It starts the
CLI as a child process and speaks its front-end protocol; the CLI owns
everything that plays or scores games, this repository owns presentation and
the user's own data (PLAN §G3). The CLI repository's `PLAN.md` §S5.15 is the
protocol's principles and its `docs/cli/protocol.md` the contract. The two
repositories advance together on the joint milestones in PLAN §G9.

Both products are called Colosseum: the desktop application **Colosseum**,
the command-line tool **Colosseum CLI** (`colosseum-cli`). A step number
prefixed `CLI` (`CLI 11.4`) is the CLI repository's; a bare number is a step
here.

| Repository | Address | Before the swap of 2026-11-13 (ADR-0001) |
|---|---|---|
| This one | `maelic13/colosseum` | `maelic13/colosseum-gui` |
| The CLI's | `maelic13/colosseum-cli` | `maelic13/colosseum` |

This repository holds both desktop generations until 2.0.0: the 1.x
application (Rust, egui) in `legacy/`, maintained and released from here,
and the 2.0 application being designed. Releases are plain `vX.Y.Z` tags on
`master`. `master` is the trunk: work reaches it as squash pull requests
from short-lived branches (*Pull requests* below). The swap and the release
order are in PLAN §G8 and
[ADR-0001](docs/architecture/adr/0001-1x-maintained-here-and-the-swap-at-the-split.md),
which moved the swap of
[CLI ADR-0012](https://github.com/maelic13/colosseum-cli/blob/dev/docs/architecture/adr/0012-names-and-repositories-after-the-split.md)
to the split.

**What users read is about the desktop application only.** `README.md`,
the changelogs and the release notes cut from them do not mention the CLI,
except the one short paragraph near the top of `README.md` that points to
`maelic13/colosseum-cli`, until the application itself uses the CLI (2.0).
The maintainer-facing documents (`PLAN.md`, `GUIDE.md`, this file,
`docs/`) speak of the CLI freely.

## Scope and architecture

- Implement numbered `GUIDE.md` steps in order. Do not skip a phase exit.
- **Design first.** No screen is implemented before its specification (Phase
  3) is signed off, and no specification is written before the design system
  (1.4) and the protocol (CLI 11.4) exist. A step tempted to build a
  screen early stops instead.
- **Never re-implement the CLI.** No rating, statistic, pairing, clock,
  adjudication, PGN writing or move generation lives here. Positions arrive
  as FEN; results arrive as facts; anything missing is a CLI extension
  requested through the CLI repository's gap analysis, not code here.
- **One component catalogue.** Every screen is composed from the design
  system's components. A new need extends the catalogue and its gallery in
  both themes first.
- **Performance is a requirement with numbers** (PLAN §G5). The UI thread
  only renders; long work runs in the CLI process or off the UI thread;
  every long list is virtualised; live updates coalesce to one render per
  frame; the window is usable before data arrives.
- **Dependencies where they pay.** Every dependency has a line in
  `docs/dependencies.md` saying why; a dependency that overlaps one already
  taken, or adds weight for little, is refused.
- **Tools in Python, platform-neutral** (PLAN §G4, *Tooling*). A new
  repository tool is a Python 3 script, standard library first, that would
  run unchanged on Linux and macOS: `pathlib` paths, processes started
  without a shell, no PowerShell or shell scripts beyond a few lines of CI
  glue. Development runs on Windows for now and only Windows is tested, but
  nothing may make the move to another machine harder.
- **`legacy/` is 1.x, frozen.** It receives the fixes 1.x users need and
  nothing else: no feature, and no sync from the CLI's crates unless a 1.x
  fix requires it. The 2.0 application copies nothing from it except assets
  and the product rules the inventory keeps, and depends on nothing in it.
  The inventory reads it in place; it differs from `gui-v1.1.0` (commit
  `ba308297f8866dc7aa8fd3172730d941e05b3d68`) only as ADR-0001 records.
  `legacy/` is removed in the change that releases 2.0.0.
- Treat engine crashes and protocol quirks as real, supported conditions:
  every error the CLI reports has a place on a screen.

## Step and commit discipline

One numbered `GUIDE.md` item is the normal unit of work.

### Remote operations

An agent may perform remote operations that can be undone: fetch, push its
own work branches, open and update pull requests (title and description as
below), read CI runs, logs, releases and repository settings, and re-run a
failed workflow.

Anything that cannot be undone, or that others see as final, waits for the
maintainer's explicit go-ahead for that one action, asked in chat with what
it will do:

- force-pushing, or otherwise rewriting history that has left this
  machine;
- deleting a remote branch, tag or release (GitHub's automatic deletion of
  a merged pull request's branch excepted);
- merging into `master`, pushing a release tag or publishing a release;
- changing the repository's settings, branch rules or secrets.

`master` takes no direct push; everything reaches it by pull request.

### Pull requests

`master` is the trunk. Each larger piece of work (a phase, or a coherent
group of steps) lives on its own short-lived branch with its step commits,
and reaches `master` as one squash commit through a pull request the
maintainer merges once CI and Release are green. Merge `master` into the
branch first if it moved; never rebase a branch that has been pushed. The
title and description become the squash commit's message, as Rarog writes
them (the repository fills the squash message from them, `GUIDE.md`,
*Repository settings*):

- **Title:** a short imperative saying what changed for the product
  ("Move the 1.x application", "Finish the repository swap"); no step
  number, phase or result. A release is titled `Release X.Y.Z`.
- **Description:** a few plain lines on what and why, wrapped at 72
  columns. Secondary changes follow in one paragraph opening "Also:". Then
  the evidence, one line each: the checks run and their counts
  (`legacy/: fmt, clippy clean; 512 tests passed`), a design sign-off or
  measurement where the step had one. No attribution.

`PLAN.md` §G9 records each open step's capability class (`R3`, `R2`, `I2`,
`I1`, `M`, `V`), mirrored on the numbered `GUIDE.md` item; `GUIDE.md`'s
"Current model mapping" table is the single maintainer-edited source that
maps a class to the current Claude model and thinking mode. Whenever
reporting the next step, name it with its class and recommend the mapped
model and mode as `Claude: <model> — <mode>` with a brief task-specific
reason; do not substitute newer models, and never change the active model
automatically. If an `I1`, `M` or `V` step exposes a material design choice
not settled by the plan, stop and continue it as `R2` rather than inventing
the contract; never silently downgrade a recorded class. Keep classes
synchronized between PLAN and GUIDE.

1. Start from a clean worktree, or identify and preserve pre-existing user
   changes. Never stage unrelated files.
2. Implement the step, its tests, documentation and generated files.
3. Run the verification baseline below and whatever the step names.
4. Demonstrate the exit criterion. A compiling change is not complete.
5. Tick the finished step (`- [x]`, ending `— DONE <date>`) in `GUIDE.md`
   in the same change, with the evidence the exit asks for. An open step is
   `- [ ]` ending in its class; end it `— **IN PROGRESS / <class>**` only
   while genuinely in progress. Sub-steps are indented four spaces under
   their step, one line each.
6. Commit the completed step before starting another numbered step.

Use a short imperative commit subject naming the outcome, preferably with
the step identifier, for example `GUI 2.2: component gallery in both themes`.

Do not:

- combine independently completable steps in one commit;
- mark or commit an incomplete step as done;
- amend, rewrite, reset or discard user commits or changes unless asked;
- add `Co-authored-by`, assistant attribution or other authorship trailers,
  in commits, pull request titles and descriptions or files
  (`.claude/settings.json` turns Claude Code's own attribution off);
- begin the next numbered step until the current step's commit succeeds.

If a step is blocked, leave it unchecked, document the blocker, and do not
create a misleading completion commit.

## Verification baseline

Fixed at step 2.1 when the stack is in place and recorded here then. Until
then: `git diff --check` and a consistency read of the documents a step
touched. From 2.1 on, whatever CI enforces, in this order: formatter check,
linter, type check, unit tests, component screenshot tests in both themes,
end-to-end tests against the CLI's synthetic streams, and the protocol
contract test against the pinned CLI's published fixtures. A formatting or
lint failure is a failure.

A change under `legacy/` runs, in `legacy/`: `cargo fmt --all --check`,
`cargo clippy --locked --workspace --all-targets -- -D warnings` and
`cargo test --locked --workspace --all-targets`, as its CI does.

## Documentation ownership

- `PLAN.md` and `GUIDE.md`: maintainer-facing specification and tracker.
  `PLAN.md` carries the binding specification and only the open work;
  completed phases become one line in `GUIDE.md` and a record under
  `docs/architecture/`.
- `docs/design/`: the design documents (inventory, requirements, concepts,
  wireframes, design system, screens). Signed-off documents change only by a
  dated amendment.
- `docs/architecture/adr/`: decisions, numbered from 0001 in this repository.
- `docs/dependencies.md`: every dependency with one line of reason.
- `README.md` and the changelog (`legacy/CHANGELOG.md` for 1.x,
  `CHANGELOG.md` from 2.0): user-facing; short and plain, no phase numbers,
  internal argumentation or mention of the CLI (*What this repository is*).
