# Colosseum — desktop application development guide

The short operational view: where the work stands and what to do next.
Rationale, specifications and exit criteria live in [`PLAN.md`](PLAN.md)
(§G9 holds the steps); the evidence of completed phases will live under
`docs/architecture/`.

**This file and `PLAN.md` are the maintainer-facing pair.** `README.md` and
`CHANGELOG.md` are user-facing and carry no phase numbers or internal
argumentation.

## Current checkpoint

| | |
|---|---|
| Repository | `maelic13/colosseum-gui`, seeded from the CLI repository at its step 11.2 (`docs/gui-v2/`); takes `maelic13/colosseum` in the swap before both releases (PLAN §G8). Work on `dev`, merged into `master` by one pull request after the swap. No code yet |
| Predecessor | Colosseum GUI 1.1.0 (egui), released 2026-09-22. It stays on the CLI repository's `main`, patchable, until the merge; it left the CLI repository's `dev` at CLI 11.2.1 (2026-09-29), and 1.x is read at commit `ba30829` |
| What is missing | Everything: Phase 1 (design foundations), Phase 2 (repository and technology proof), Phase 3 (screen specifications), Phase 4 (implementation), Phase 5 (release) |
| Depends on Colosseum | 11.4 protocol specification (before Phase 3), 13.2 synthetic streams (before 2.3), 13.3–13.4 (before 4.5; the lifecycle decision with 4.2), 13.5–13.7 (before 4.6), 13.8 release candidate (4.8) — PLAN §G9 joint milestones |
| Next step | **1.1.1** — the engine-library decisions; Delete All, names and versions and rating lists decided in inventory §12; the rating anchor (Q1) waits for the maintainer — `R2` (Claude Opus 5 — High); then 1.1.2–1.1.4 and 1.2, each `R2`. **2.1** — skeleton — `I2` (Claude Opus 5 — High) may start at the same time |
| Confirmed decisions | PLAN §G2 (2026-09-27): design first; a client of the CLI over its protocol; TypeScript + React with shadcn/ui on Radix and Tailwind; the shell a measured choice at 2.3 between Electron and Tauri 2 with Electron the default expectation; placement off, several tournaments at once; 64 participants the design point; lifecycle and close behaviour decided at 4.2 with *stop now on close* the starting preference; amendments and changed engines are CLI mechanisms; run file per tournament, rebuildable index, positions as FEN, no chess logic here; the 1.x library imported, 1.x tournaments not migrated; both themes equal |

## Current model mapping

PLAN §G9 records each step's stable capability class; this table maps the
classes to the current Claude model and thinking mode and is the only place
model names appear. Edit it when model generations change; do not rewrite
the roadmap. These are maintainer judgements, not measured rankings.

| Class | Capability | Claude |
|---|---|---|
| `R3` | Frontier causal/architecture research | Claude Fable 5.1 — High |
| `R2` | Bounded correctness-sensitive reasoning | Claude Opus 5 — High |
| `I2` | Difficult implementation | Claude Opus 5 — High |
| `I1` | Well-specified implementation | Claude Sonnet 5 — Medium |
| `M` | Mechanical/docs/provenance | Claude Sonnet 5 — Medium |
| `V` | Verification/measurement | Claude Sonnet 5 — High |

## Phases and steps

Each phase ends with a verifiable exit criterion. Nothing is "done" because
it compiles or renders; it is done when its criterion is demonstrated. When
reporting the next step, always report its class and the model the
mapping above gives it. Step identifiers are never reused.

Every phase, step and sub-step is one checkbox line here, ending in its
class (`IN PROGRESS / <class>` while under way, `DONE <date>` once ticked);
sub-steps sit indented under their step. PLAN §G9 holds each step's
specification and exit; `(Mn)` marks a joint milestone.

### Phase 1 — Design foundations (documents only; sign-offs in pairs)

- [ ] **1.1** Inventory of 1.x from the CLI repository at `gui-v1.1.0`: every screen, control, setting, stored item, workflow, keyboard path, error surface, background behaviour, defect and lesson, classified keep / change / drop / decide; guidelines split into product rules and egui workarounds ([`docs/design/inventory.md`](docs/design/inventory.md)). Draft 2026-09-29: 163 items (keep 68, change 74, drop 6, decide 15), 21 product rules, 17 egui workarounds, 18 defects, 12 lessons, 19 questions (14 in 1.1.1–1.1.4, 5 for CLI 11.3.1–11.3.5); reviewed with 1.2 — **IN PROGRESS / R2**
    - [ ] **1.1.1** Decide the engine-library questions: Delete All, clone identity, library rating provenance and history (ENG-10, ENG-19, ENG-23); Delete All dropped; engine, version and profile decided; rating lists decided with the anchor open (Q1), inventory §12, 2026-09-29 — **IN PROGRESS / R2**
    - [ ] **1.1.2** Decide the tournament-setup questions: multi-seed gauntlet, an extra PGN file, what presets carry, the default time allowed past the clock (TRN-06, TRN-21, TRN-22, PLY-08) — **R2**
    - [ ] **1.1.3** Decide the results and live-view questions: ratings reverted on delete, evaluation point of view (ARN-05, LIV-09) — **R2**
    - [ ] **1.1.4** Decide the application questions: update checks, portable mode, writeback conflicts, second instance, long-run behaviour (APP-06, APP-08, APP-15, APP-22, APP-23) — **R2**
- [ ] **1.2** Requirements: users, workflows and frequency, scale targets confirmed (64 participants the design point), performance budgets as numbers, keyboard and accessibility, the library import, the open decisions answered as requirements (several tournaments, close expectations, amendments, changed engines) and the answers of 1.1.1–1.1.4; `docs/design/requirements.md` signed off with 1.1; handed to CLI 11.3 (M2) — **R2**
- [ ] **1.3** Layout concepts: two or three as static HTML wireframes in both themes, walked through the workflows; the maintainer chooses; `docs/design/concepts.md` — **R2**
- [ ] **1.4** EXIT — Design system: tokens for both themes and the component catalogue with states, sizes and keyboard behaviour; `docs/design/design-system.md` signed off with 1.3 — **R2**

### Phase 2 — Repository and technology proof (beside Phase 1)

- [ ] **2.1** Skeleton: the §G4 front end behind a shell-agnostic interface in the Electron shell; formatter, linter, type check, unit / component / end-to-end runners, CI on the three required platforms, a packaged empty application per required target, `docs/dependencies.md`, user-facing `README.md`, the verification baseline written into `AGENTS.md` — **I2**
- [ ] **2.2** Component gallery: the 1.4 catalogue on the real stack in both themes with screenshot tests; gaps fed back into 1.4 before Phase 3 — **I1**
- [ ] **2.3** EXIT — Performance proof against the 1.2 budgets at scale on the CLI's synthetic streams (CLI 13.2), the same front end in Electron and in Tauri 2, on macOS arm64, Windows x64 and Linux x64; the shell chosen by ADR (Electron unless Tauri meets every budget and its smaller bundle is judged worth it), the other build removed; the §G4 fallbacks measured the same way only if the front end itself fails (M4) — **V**

### Phase 3 — Screen specifications (after 1.4, 2.2 and CLI 11.4)

- [ ] **3.1** Shell, settings and engine library; guards DEF-02, 04, 05, 06, 08 — **R2**
- [ ] **3.2** Tournament creation, presets, history and results; guards DEF-07, 13, 15, 17, 18 — **R2**
- [ ] **3.3** EXIT — Live view, lifecycle (stop, stop now, close options, resume) and amendments; guards DEF-01, 10, 11, 12, 16; `docs/design/screens.md` signed off; every keep and change item placed or dropped; every defect guarded — **R2**

### Phase 4 — Implementation (a screen only after its specification is signed off)

- [ ] **4.1** Design system in code, gallery finalised, screenshot regression — **I1**
- [ ] **4.2** Shell and CLI process manager: bundled and pinned CLI, handshake and version check, crash handling, Windows Job Object; the lifecycle decision (close, detach, re-attach) analysed and taken with CLI 13.4, recorded by ADR; guards DEF-01, 02, 09 — **R2, then I2**
- [ ] **4.3** Engine library with the 1.x import; guards DEF-02, 03, 04, 05, 06, 08 — **I1**
- [ ] **4.4** Tournament creation and presets: run file writer, dry-run validation, the largest tournament within budget; guards DEF-07, 15, 17 — **I1**
- [ ] **4.5** Live tournament view: facts and state coalesced to one render per frame, gap and snapshot resync, several tournaments at once, following a game; guards DEF-12, 15 (M5) — **I2**
- [ ] **4.6** Results, history and amendments: rebuildable index, standings, crosstable, games browser on paged queries, export, stop and resume, add / remove / length, changed engines, rating writeback; guards DEF-10, 11, 13, 16, 18 (M6) — **I2**
- [ ] **4.7** Packaging, signing, updater and release lane for the required targets and the cheap wanted ones; the CLI bundled and pinned — **I2**
- [ ] **4.8** EXIT — Every keep and change item present or dropped; no inventory defect reproducible; budgets met on a real tournament on the three required platforms (native Rarog and Basilisk on macOS and Linux); usability walkthrough; against the CLI 13.8 release candidate (M7) — **V**

### Phase 5 — Release

- [ ] **5.1** EXIT — First release after the swap and the merge: 2.0.0 with its `gui-v2.0.0` bridge release, `CHANGELOG.md`, the pin set to the released CLI, published right after the CLI release (M9) — **V**

## What to do now

**Decide 1.1.1–1.1.4, then write 1.2; 2.1 beside them.** The inventory
draft (1.1) is written; its fourteen questions for this repository are
steps 1.1.1–1.1.4, each answered, dated, in inventory §12. 1.2 carries the
answers into the requirements and is reviewed with 1.1; when it is signed
off it goes to the CLI repository, whose 11.3 and 11.4 produce the protocol
specification Phase 3 needs. 2.1 stands the repository up on the
provisional stack at any time. 2.3 waits for CLI 13.2. Nothing in Phase 4
starts before its Phase 3 specification is signed off.

```
git diff --check
```

## Working rhythm

```text
Pick the next open step  ->  do it + tests  ->  demonstrate its exit
criterion  ->  tick it here  ->  update PLAN.md when useful  ->  commit before
the next step.
```

Use a focused imperative commit subject, stage only the step's files and
never add co-author or assistant-attribution trailers; `AGENTS.md` is the
binding repository workflow.

## Recurring procedures

### Bumping the pinned CLI

- A numbered step: update the pin, the bundled binary checksums and the
  contract test's schema and fixtures together; run the end-to-end suite on
  the new synthetic streams; note the protocol version in `CHANGELOG.md`.

### Cutting a release (maintainer)

- The CLI release this build pins is published first.
- The first release only: after the swap and the merge, the `gui-v2.0.0`
  bridge release beside `v2.0.0` (PLAN §G8).
- Changelog dated; candidate dispatched and its packages started on the
  three required platforms; tag; watch the release workflow; check the
  release page and the updater from the previous version.

## Decision rules

| Situation | Action |
|---|---|
| A screen is wanted before its specification is signed off | Not allowed: design first (PLAN §G2) |
| A screen needs a component the catalogue lacks | Extend the catalogue and the gallery in both themes first, then use it |
| Something takes longer than a frame on the UI thread | Move it to the CLI or off the UI thread; virtualise the list; coalesce the updates |
| The CLI does not provide something a screen needs | A CLI extension through the protocol, requested via the CLI repository's gap analysis — never game-playing, rating or chess logic here |
| A protocol message is unknown to this build | Ignore it if the handshake's major version matches; refuse to start the CLI if it does not |
| A `gap` fact arrives | Request a snapshot and re-apply facts above it; never guess |
| Tempted to keep a derived statistic | Ask the CLI's query again; the run directory is the record |
| A dependency would save work | Accept only if mature, widely used, maintained, GPL-3.0-compatible and not overlapping one already taken; one line of reason in `docs/dependencies.md` |
| A GUI behaviour would differ from the CLI's for the same mechanism | The CLI's is the specification |
| Front-end code wants a shell API (Electron or Tauri) directly | Not allowed: go through the shell interface, so the 2.3 choice stays a build-level decision |
| Tempted to coordinate CPU placement across tournaments | Placement is a CLI feature; tournaments here run with it off |
