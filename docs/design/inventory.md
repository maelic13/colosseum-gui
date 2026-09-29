# Inventory of Colosseum 1.x

GUI step 1.1. Everything the 1.x desktop application does, each item
classified for the new application. It is reviewed and signed off together
with the requirements (1.2).

| | |
|---|---|
| Status | Draft, 2026-09-29, for review with 1.2 |
| Source | The CLI repository at `gui-v1.1.0`, commit `ba308297f8866dc7aa8fd3172730d941e05b3d68`: `crates/colosseum-gui` (15,000 lines), the parts of `colosseum-engine`, `colosseum-core` and `colosseum-uci` a 1.x tournament runs through, `CHANGELOG-GUI.md` (0.1.0 to 1.1.0), `docs/design/GUIDELINES.md`, `CLAUDE.md` and the architecture records. Only `colosseum-cli` changed between the tag and the CLI repository's `dev` of 2026-09-29 (the CLI 0.2.0 work); the CLI column in §8 reads that `dev` |
| Used by | 1.2 (requirements), 1.4 (design system: §9 and §10), 3.3 and 4.8 (every *keep* item placed on a screen or dropped by the maintainer), CLI 11.3 (the gap analysis: §8 and the CLI column there) |

## 1. How to read it

**Classes.** A class judges the *capability*, never the egui implementation.

- **keep**: the new application has it, as 1.x has it or better.
- **change**: the new application has the capability, but something about it
  is wrong in 1.x; the note says what.
- **drop**: the new application does not have it; the note says why.
- **decide**: the maintainer decides; the question is in §12 under the same
  ID.

The binding requirements (PLAN §G2) and decisions (CLI ADR-0011, ADR-0012)
already settle some classes, and the note names them. The main ones:

- no game playing or chess logic here;
- positions arrive as FEN;
- 1.x tournaments are not migrated, but the 1.x engine library is imported;
- CLI runs are not shown;
- CPU placement is off;
- several tournaments may run at once;
- the theme follows the device.

**IDs** are stable. ENG engine library, TRN tournament setup, ARN tournament
list and results, LIV live view, APP application shell, VIS visual identity,
PKG packaging, PLY game playing and scoring, PR product rules, EW egui
workarounds, DEF defects, LES lessons.

**Where 1.x lives.** 1.x has three tabs: **Tournament** (setup), **Arena**
(tournament list, standings, live board) and **Engines** (library). It
opens on Tournament; starting a tournament switches to Arena. The layout
itself is not inventoried as a *keep*: 1.3 chooses the information
architecture. The GUI v2 research (CLI repository,
`docs/architecture/gui-v2-research.md` §1) records why: setup and watching
live apart, and the Arena stacks four concerns into one screen.

## 2. Engine library (Engines tab)

| ID | Item | 1.x | Class | Note |
|---|---|---|---|---|
| ENG-01 | Library view | Card grid of every engine: logo or monogram, name, subtitle (author, else file name), `ELO` and `VERSION` stats, ⚠ when the executable is missing. Count in the header | keep | Shape is 1.3's call; must hold the §G5 200 engines |
| ENG-02 | Add engine | File picker (no type filter), UCI handshake in the background, identity and options read, new entry saved and selected | change | Add an executable filter per platform; see ENG-18 for detection settings |
| ENG-03 | Scan folder | Non-recursive; launches every `.exe` (Windows) or executable file (Unix) one at a time; 10 s + 10 s timeouts; progress "Scanning n/N" | change | It launches uninstallers and GUI programs blindly, cannot be cancelled, and only the last failure is shown (DEF-06). Show the candidates first, allow cancel, report per file |
| ENG-04 | Duplicate check | Same path (case-insensitive) or same name + version; modal "Engine already in library" for one, a tick list (nothing ticked) for a scan | keep | |
| ENG-05 | Name and version split | Architecture tokens (x64, avx2, bmi2, popcnt, …) dropped; the last digit-bearing token starts the version; leading `v` removed | keep | |
| ENG-06 | Sort | Name, Elo (unrated last), Author (none last); persisted | change | Version order is plain text ("10" before "9"); make it numeric-aware |
| ENG-07 | Filter | Word-wise AND over name, version, author, file name | keep | |
| ENG-08 | Selection | Click edits; Ctrl-click toggles; Shift-click ranges in visible order; right-click selects and opens the menu; click outside clears | change | Ctrl is used on macOS too; use the platform modifier |
| ENG-09 | Context menu | Clone, Re-detect (n), Open containing folder(s), Copy path, Delete (n)… | keep | |
| ENG-10 | Delete All | Inline two-step Confirm/Cancel, removes every engine and every logo | decide | §12 |
| ENG-11 | Delete | Modal naming the engines; "The executable on disk is not touched"; logos removed | keep | |
| ENG-12 | Identity fields | Name, Version, Author, Elo (optional integer) | change | Invalid Elo text is silently stored as unrated while the field still shows it; validate visibly |
| ENG-13 | Logo | Click the slot to choose (png, jpg, webp, bmp, gif, ico); copied into app data; aspect-fitted, never cropped; Remove | keep | The old file is deleted before the copy, so a failed copy leaves no logo; order it the other way |
| ENG-14 | Launch settings | Path (Change…, Open folder), arguments, working directory (Browse…), environment variables (add, remove) in a collapsed section | change | Arguments are split on whitespace, so an argument with a space is impossible: use a list. An empty working directory does not mean the engine's folder as the hint says: the engine inherits the application's directory (DEF-05). Change… does no re-detect and no duplicate check |
| ENG-15 | UCI options editor | One typed editor per declared option (check, spin with range hint, combo, string, button); only changed values stored; per-row reset and Reset all | keep | An emptied string stores an empty override that looks like the default; show it |
| ENG-16 | Options managed elsewhere | Threads, hash, ponder and tablebase options are hidden from the per-engine editor with "n managed elsewhere" | change | Hidden by substring, so Rybka "CPU Usage", Lc0 "CPuct*" and similar cannot be edited per engine (DEF-04); use the same allowlist that maps them |
| ENG-17 | Button options | Armed button sends `setoption` at every game start | keep | |
| ENG-18 | Re-detect | Single or bulk; replaces the detected options, overwrites name, version and author | change | Detection ignores the entry's arguments, environment and working directory; bulk re-detect jumps the selection through every engine, overwrites user-edited names without warning, and shows only the last error. Merge, don't steal the selection, report per engine |
| ENG-19 | Clone | New identity, same name and version, logo copied | decide | §12 |
| ENG-20 | Autosave | 600 ms after the last change, immediately on selection change, tab switch and close; "saved" flash | change | The edit buffer is a snapshot and overwrites the library on the next edit, undoing rating writebacks (DEF-03). Save per field or refresh on external change |
| ENG-21 | Global tablebases | Syzygy, Nalimov, Gaviota folders; Syzygy probe limit (3 to 7, default 7) and 50-move rule (on); Nalimov and Gaviota cache (1 to 1024 MB, default 32); Gaviota compression (cp0 to cp4, default cp4). Collapsed bar with a ● ○ summary under the engine editor | change | Keep every setting; where it lives is 1.3's call (it is not per engine). Typing a path trims it on every keystroke, so a path with spaces can only be pasted (DEF-08) |
| ENG-22 | Library file | `engines.json` next to the settings, written atomically; every edit rewrites the whole file | change | An unreadable file silently gives an empty library that the next save overwrites (DEF-02). The 1.x file is read once, at import (PLAN §G2) |
| ENG-23 | Library Elo | Shown on cards; seeds tournaments (unrated = 1500); overwritten by rating writeback | decide | No provenance or history; a typed Elo is replaced by the next game. §12 |
| ENG-24 | Keyboard | Delete removes a multi-selection only | change | Delete for the single edited engine, select all, arrows, Escape |
| ENG-25 | Empty states | "No engines yet", "No engines match your filter.", "No engine selected" | keep | |
| ENG-26 | Chess960 options | Not refused in the editor | drop | Standard chess only; the CLI refuses them |

## 3. Tournament setup (Tournament tab)

| ID | Item | 1.x | Class | Note |
|---|---|---|---|---|
| TRN-01 | Participant picker | Filterable, sortable list (Name, Elo, Author; persisted) with logo, name, version, Elo, ⚠ missing; whole row toggles; Select all (respects filter), Clear (ignores it); "n selected" | change | Select all appends in storage order, not display order |
| TRN-02 | Seeding | Selection order is the seeding order; the first selected engine is the gauntlet engine; never shown | change | Show any order that matters; the gauntlet engine is chosen explicitly (TRN-06) |
| TRN-03 | Per-tournament UCI overrides | Row "⋯" or right-click opens every option with the value that applies now; overrides beat library and tournament-wide values; ● marks an engine with overrides; Clear tournament overrides | keep | Overrides survive Start into the next setup in the same session: clear them with the rest |
| TRN-04 | Name | Default "Tournament"; blank becomes "Tournament"; duplicates allowed | keep | |
| TRN-05 | Formats | Round robin, gauntlet; "Knockout (planned)" and "SPRT (planned)" shown disabled | change | Show only what exists. SPRT belongs to the CLI and is not shown here (PLAN §G2) |
| TRN-06 | Gauntlet engine | Picks one engine; the core supports several seeds but only one is offered | decide | Multi-seed gauntlet: §12 |
| TRN-07 | Cycles | 1 to 10,000 | keep | |
| TRN-08 | Games per pair | 1 or 2 (both colours) | change | With 1 game per pair and several cycles, the same engine has White in every cycle (DEF-14) |
| TRN-09 | Schedule summary | "n engines · G games · ~duration" in the card and again beside Start | keep | Not pluralised beside Start ("1 games") |
| TRN-10 | Parallel games | 1 to the logical cores | change | Ignores tournaments already running; a warning, not a cap (APP-17) |
| TRN-11 | Time control | Per move, sudden death, base + increment (value + ms/s/min unit), fixed nodes, fixed depth; default 100 ms per move | keep | Moves-per-session controls, per-engine controls and a visible time margin are absent (§11) |
| TRN-12 | Duration estimate | ~60 moves per side; sudden death at its full budget; none for nodes and depth; total ×1.05 over parallel lanes | keep | |
| TRN-13 | Threads | On by default, value 1, capped at the cores; forwarded to every recognised thread option, clamped to its range | keep | Where the mapping lives: §12 PLY-12 |
| TRN-14 | Hash | Off by default, 128 MB when on | keep | No memory check against parallel × 2 engines × hash |
| TRN-15 | Ponder | Off by default; always sent as `Ponder=true/false` | keep | Semantics: §12 PLY-15 |
| TRN-16 | Use endgame tablebases | On by default; off withholds every tablebase option from every engine | keep | |
| TRN-17 | Compatibility notes | "⚠ n compatibility notes": CPU oversubscription, missing executable, no thread option, values capped or raised, no hash option; never block Start | change | Count two engines per game and the tournaments already running |
| TRN-18 | Adjudication | Max moves (off, 300); draw (off; from ply 40, 8 moves, ≤ 8 cp); resign (off; 4 moves, ≥ 800 cp, both engines agree); natural endings always | keep | Defaults and one-sided resign: §12 PLY-16 |
| TRN-19 | Opening book | EPD or PGN file; format chips; sequential or random with a seed (default 0); PGN plies (8); limit count; preview "n openings loaded, e.g. …" | change | The preview loads the whole book on the UI thread at every keystroke (DEF-07); rejected lines are skipped silently; a book switched on without a file silently plays the start position; the folder is not remembered; no way to pick a fresh seed. The order is the CLI's (1.x tournaments are not migrated) |
| TRN-20 | Rating writeback | Never (default), All engines, Chosen engines (picker, nothing ticked by default) | change | Chosen with nothing ticked updates nothing and says nothing; a preset saved in Chosen mode loads empty |
| TRN-21 | PGN output file | Optional path; every game appended as it ends; failures only logged | decide | §12 |
| TRN-22 | Presets | Save as (name defaults to the tournament name), load, delete (×, no confirm); stores everything but the engine selection, overrides and the Chosen set | change | Confirm overwrite and delete; names differing only in case collide silently; errors only logged. What a preset carries: §12 |
| TRN-23 | Last-used form | Restored at launch (name forced to "Tournament") | keep | |
| TRN-24 | Start | Validates two engines, injects the global tablebase settings, creates and starts at once, switches to Arena; the form resets writeback, the Chosen set and the PGN path | change | The tournament is written before its opening book is loaded, which can leave an orphan with no games (unverified). Validate first, with the CLI's `--dry-run` (PLAN §G3) |
| TRN-25 | Legacy fields | Hidden `syzygy_path`, `syzygy50*` preset fields still take effect; `Estimate` writeback; knockout and SPRT enum values | drop | No 1.x tournament or preset is migrated |
| TRN-26 | Number entry | Drag-to-change fields | drop | An egui idiom; typed entry with steppers |
| TRN-27 | Keyboard | None; Escape closes modals | change | Part of the shortcut map (APP-24) |

## 4. Tournament list and results (Arena, Standings)

| ID | Item | 1.x | Class | Note |
|---|---|---|---|---|
| ARN-01 | Tournament list | Newest first; name, status (● Live, Finished, Stopped), "finished / total games", created time; collapsible, remembered per lens | change | The time is UTC shown as if local; no search or filter for the §G5 500 tournaments |
| ARN-02 | Loading | Opening Arena selects the newest tournament and loads it, replaying its games on the UI thread; every clicked tournament stays loaded, with a driver, for the session | change | The list is a rebuildable index over run directories and results come from short-lived queries (PLAN §G3); nothing is "loaded" to browse |
| ARN-03 | List actions | Ctrl and Shift multi-select; menu Start, Stop, Force-stop (n), Rename…, Delete (n)… | change | Show which actions apply (all are always enabled) |
| ARN-04 | Rename | Inline on the title (Enter, Escape) or a modal; blank refused | change | One pattern; a rename does not reach the PGN `Event` of later games in the same session |
| ARN-05 | Delete | Modal from the list, inline two-step in the live bar; force-stops, then removes the tournament and its games | change | One pattern; the PGN file and incident reports stay behind. Whether written-back ratings revert: §12 |
| ARN-06 | Delete key | Acts on a Ctrl/Shift selection only | change | Act on the selected tournament too |
| ARN-07 | Status | Running, Stopping, Stopped, Finished, Idle, each a coloured pill | keep | |
| ARN-08 | Start, Stop, Force-Stop | Start (or resume); Stop lets games in progress finish; Force-Stop discards them and re-queues them first | keep | PLAN §G2 two stop actions; close behaviour at 4.2 |
| ARN-09 | Parallel, live | 1 to 256 while running; lowering never cancels a game; stored for resume | decide | §12 PLY-26 |
| ARN-10 | Progress | "n / N games", bar, elapsed, "avg x/game", ETA (measured, or estimated from the time control) | change | Elapsed and averages restart at zero after a reload (DEF-12) |
| ARN-11 | Standings columns | # (medal 1 to 3), Engine (name + version), Elo, Δ, Pts, SB or Score, Perf, Gms, W-D-L, Avg nps, Avg depth, Time/move, Forfeits (n× time · n× crash) | keep | Every figure comes from the CLI (PLAN §G3), which today has neither SB, Perf, nps, depth, time per move nor the forfeit split (§8) |
| ARN-12 | Ranking | Points, then Sonneborn–Berger; equal on both share a place ("2-3"); a one-seed gauntlet pins the seed on top ("G", unranked) and ranks opponents by score share | keep | |
| ARN-13 | Elo and Δ | Elo shows what the library holds, or the start rating when the tournament does not update that engine; hover gives ± 95%; Δ chip against tournament start, "—" when not updated | keep | PR-02 |
| ARN-14 | Column order | Drag headers; order remembered | change | Add hide and reset |
| ARN-15 | Sorting | Click a header; arrow on the active one | change | Once sorted by something else, rank order cannot be restored |
| ARN-16 | Head-to-head | Matrix of W-D-L or per-game results (1 ½ 0) with heat tint and row/column cross-highlight; a one-seed gauntlet gets a per-opponent table instead | change | At 64 participants the headers and "Results" cells grow without limit |
| ARN-17 | Playing now | Cards per game in progress (round, White, Black); click to watch | keep | |
| ARN-18 | Terminations | Counts under Natural, Adjudicated, Errors with bars | change | Labels differ from the live banner's (LIV-08) |
| ARN-19 | Information | Created, format, time control, games per pair, parallel, adjudication, ponder, tablebases, openings, PGN file, ratings | change | Complete it: threads, hash, overrides, book order and seed |
| ARN-20 | Engine errors | Collapsible danger bar, newest 20 of 50, each naming the culprit, the outcome and the incident file | change | Lost on reload; the incident path is plain text (DEF-10) |
| ARN-21 | Unscored games | "not scored, the game was not played" with the reason | keep | |
| ARN-22 | Decisive / drawn split | Only in the fallback layer (ARN-24) | keep | Into the main results |
| ARN-23 | Export | Standings CSV, crosstable CSV, game PGN (skips unscored), native save dialog, "Saved x" note | change | Export copies the CLI's files (PLAN §G3); the 1.x standings CSV drops shared places, SB, Perf and writes a gauntlet seed as rank 0 |
| ARN-24 | Stored-detail layer | A second, older results view for a tournament that failed to load: resume, copy PGN path, a 12-column table | drop | Duplicate |
| ARN-25 | Finished games | Not browsable; "export the PGN" | change | PLAN §G3 adds a games browser on paged queries |
| ARN-26 | Empty states | "No tournaments yet"; "Waiting for the first game to finish…" (never actually shown) | change | A real before-the-first-result state |

## 5. Live view (Arena, Live)

| ID | Item | 1.x | Class | Note |
|---|---|---|---|---|
| LIV-01 | Games rail | Games in launch order with "R n · move m ●" or the result; resizable, collapsible | change | Hidden with fewer than two games in flight even when a finished game is frozen on screen |
| LIV-02 | Choosing and following | First game by default; a finished game stays frozen with its result; auto-follow (on when parallel is 1) moves 2 s later to the game launched after it | keep | |
| LIV-03 | Matchup header | White vs Black with Elo chips and round | keep | |
| LIV-04 | Opening name | ECO code and name from the lichess database, transposition-aware, standard starts only | keep | Chess logic: it has to arrive from the CLI (§8) |
| LIV-05 | Move list | Numbered SAN, last move highlighted | change | Numbering ignores a FEN start; book moves are not marked. SAN has to arrive from the CLI |
| LIV-06 | Material | Each side's extra pieces with the board's pieces, difference in the middle | keep | |
| LIV-07 | Board | cburnett pieces, wood squares for dark and light, last-move highlight, coordinates at 34 px squares and up; White at the bottom | change | Add flip |
| LIV-08 | Result banner | "Name wins · 1–0 · Checkmate" over the board | change | One set of termination labels (ARN-18) |
| LIV-09 | Engine panels | Logo, name, Elo chip, evaluation, thinking… or pondering…, depth/seldepth, nodes, nps, clock chip; the side to move gets an accent wash | keep | Evaluation point of view: §12 |
| LIV-10 | Clocks | Tick locally while thinking; "m:ss"; per move, nodes and depth show the current search time | change | No tenths, so fast games read "0:00" |
| LIV-11 | Search log | One line per completed depth, newest on top, 80 lines per side: score, depth, time, nodes, PV | change | PV in SAN (from the CLI) |
| LIV-12 | Evaluation graph | Both engines every ply including ponder evaluations; range snaps 1, 2, 3, 5, 10 pawns and only grows; mate at the edge | change | Add a hover readout |
| LIV-13 | Several tournaments | One live view per tournament, switched through the list | change | PLAN §G2: the live view shows all running tournaments |
| LIV-14 | Refresh | Redraw every 100 ms, backend at 30 Hz | drop | Implementation; PLAN §G5 coalescing to one render per frame |

## 6. Application shell and background behaviour

| ID | Item | 1.x | Class | Note |
|---|---|---|---|---|
| APP-01 | Navigation | Three tabs in a header, logo opens About, status bar at the bottom | change | 1.3 |
| APP-02 | Global status | The liveliest status across loaded tournaments | change | Cannot say which tournament |
| APP-03 | Theme | Dark, Light, System (default System) | keep | PLAN §G2 |
| APP-04 | Window | Default 1100×720, minimum 860×560; size and maximised state restored | change | The maximised size overwrites the normal size; position and monitor are not restored |
| APP-05 | About | Version, update check, tagline, GPL-3.0-or-later, repository link, credits: cburnett pieces (CC BY-SA 3.0), lichess openings (CC0), Inter and JetBrains Mono (OFL) | keep | The piece licence requires the credit |
| APP-06 | Update check | Manual, in About; GitHub releases, newest stable `gui-v` tag | change | The new channel reads this repository's `v` tags (PLAN §G8). Automatic checks: §12 |
| APP-07 | `--version` | Prints the version and exits | keep | Add `--help`; unknown flags are silently ignored |
| APP-08 | Portable mode | `--portable` keeps everything beside the executable | decide | §12 |
| APP-09 | Directory fallback | Without OS user directories, silently uses the executable's folder | change | Say where the data went |
| APP-10 | Startup error | Native dialog "Could not start Colosseum" for backend failures | change | A window or graphics failure is only printed to stderr, which on Windows is nothing |
| APP-11 | Application log | `logs/colosseum.log`, rotated at startup past 4 MiB | change | Rotate during a run too; open from the UI |
| APP-12 | Incident reports | Per-game file on time loss, crash, illegal move, setup failure: engines and paths, time control, clocks, start FEN, moves, timing, last 120 UCI lines and 40 stderr lines per engine | keep | The CLI writes them into the run directory; open them from the error (DEF-10) |
| APP-13 | Close while running | "Quit Colosseum?": Keep running, Stop & quit, Force-stop & quit | change | Stop & quit does not wait (DEF-01). 4.2 decides close behaviour |
| APP-14 | Engines die with the application | Windows Job Object, Linux process group and parent-death signal, kill-on-drop | keep | PLAN §G3 |
| APP-15 | Rating writeback | After every finished game, the covered engines' rounded rating written to the library; run from the UI frame loop | keep | PLAN §G3; conflicts: §12 |
| APP-16 | Data locations | `%APPDATA%\Colosseum\config` and `\data` (Windows), `~/.config/colosseum` and `~/.local/share/colosseum` (Linux), `~/Library/Application Support/Colosseum` (macOS) | change | Never these directories (PLAN §G8, CLI ADR-0012); read once for the library import |
| APP-17 | Several tournaments | Any number loaded and running; each has its own parallel limit; no shared budget | keep | PLAN §G2; warn when the sum oversubscribes the cores |
| APP-18 | Settings | `config.toml`: theme, window, tablebases, sort orders, panel widths, column order, list open per lens | change | One settings surface; atomic writes (1.x overwrites in place); `last_pgn_dir` and `last_openings_dir` are dead fields |
| APP-19 | Error surfaces | Inline red lines per tab, an export note, the errors bar, modals, log-only failures | change | One notification model; nothing log-only that the user needs to know (DEF-09) |
| APP-20 | Selectable text | Labels cannot be selected | change | Error messages and paths must be copyable |
| APP-21 | Keyboard | Delete (multi-selection), Enter and Escape in rename, Escape and click-outside close modals | change | A deliberate shortcut map at 1.2 and 1.4 |
| APP-22 | Second instance | Nothing prevents two copies on the same data | decide | §12 |
| APP-23 | Long runs | No sleep inhibition, OS notification, taskbar progress or tray | decide | §12 |

## 7. Visual identity and packaging

| ID | Item | 1.x | Class | Note |
|---|---|---|---|---|
| VIS-01 | Palette | Cool slate with a warm gold accent; paired dark and light token sets (bg darkest/panel/elevated/hover/input/faint, stroke, text/weak/faint, accent/bright, success, warn, danger, graph white/black, medals) | keep | Identity; 1.4 re-derives the tokens for both themes (PLAN §G6: never one derived from the other) |
| VIS-02 | Type | Inter (regular, semibold), JetBrains Mono for numbers; scale 20 / 14 / 12 / 13 mono | keep | |
| VIS-03 | Mark | Two gold rings on a dark rounded square | change | Keep the mark; one source asset (1.x maintains four) |
| VIS-04 | Tint convention | Semantic fill at 0.16, stroke 0.45, text in the colour itself | keep | PR-06 |
| VIS-05 | Components | Tab pill, status pill, chip, tinted buttons, section card, medal and seed badges, monogram avatar, choice chip, dropdown, checkbox, clear button, collapse strip, Δ chip, UCI option row | keep | Input to the 1.4 catalogue, as components, not their egui code |
| VIS-06 | Board and pieces | cburnett SVG pieces, wood squares | keep | |
| PKG-01 | Archives | Windows zip (x64, arm64); Linux and macOS tar.gz | keep | |
| PKG-02 | Windows installer | MSI, per machine, Start-menu shortcut, optional Add to PATH, fixed upgrade code, no downgrade | change | 4.7 decides the relationship to an installed 1.x; Add to PATH has little use for a GUI |
| PKG-03 | Linux packages | deb and rpm (`colosseum`, desktop entry, icon); Arch `pkg.tar.zst` | keep | AppImage is new (PLAN §G8) |
| PKG-04 | Desktop entry | "Chess Engine Tester", categories Game;BoardGame, keywords chess, engine, tournament, UCI, elo | keep | No file associations |
| PKG-05 | macOS | Branded dmg with a drag-to-Applications window; ad-hoc signed, not notarised | change | Signed and notarised (PLAN §G8) |
| PKG-06 | Identifiers | `colosseum`, `Colosseum.ChessGUI`, `com.colosseum.Colosseum`, `Colosseum` | change | One application identifier, different from 1.x's where the platform would confuse them |

## 8. Game playing and scoring (the CLI column feeds CLI 11.3)

Nothing here is implemented in the new application: the CLI plays and
scores every game (PLAN §G2). A *keep* means the product keeps the
behaviour, so the CLI must provide it. The CLI column records what the CLI
`tournament` command does today (`crates/colosseum-cli`, same date).

| ID | Item | 1.x | CLI today | Class | Note |
|---|---|---|---|---|---|
| PLY-01 | Schedules | Round robin by the circle method, gauntlet seeds against the field, repeated per cycle | same | keep | |
| PLY-02 | Colours | Alternate within an encounter | same | change | Not across cycles (DEF-14); CLI 11.3.5 |
| PLY-03 | Openings per encounter | One opening for both colours; the book cycles silently when short | refuses unless `--book-wrap`; reports reuse | change | Cycling allowed, reuse shown |
| PLY-04 | Opening order | Book seed, legacy shuffle | master seed, named RNG | change | The CLI's; nothing to stay compatible with |
| PLY-05 | Opening count cap | Truncate after ordering | absent | keep | CLI gap |
| PLY-06 | Book format | Chosen explicitly (EPD, PGN) | by extension | change | By extension, as the CLI does |
| PLY-07 | Time controls | Five kinds, one for both sides | same | keep | |
| PLY-08 | Forfeit tolerance | 2 s past the budget | 20 ms by default; `--margin-ms` takes any value | decide | The CLI already lets this application set its own; the default is 1.1.2 |
| PLY-09 | Engine processes | Fresh per game, spawned and configured concurrently; 30 s handshake | same for tournaments | keep | |
| PLY-10 | Option layers | Library, then tournament-wide, then per-engine overrides | flat `--option`, `--engine-option` | keep | This application composes the run file from its library |
| PLY-11 | Threads and hash | Mapped by exact-name allowlist to what each engine calls them, clamped to the declared range; never to look-alikes | sent verbatim, no clamp | keep | Where the mapping runs: §12 |
| PLY-12 | Compatibility notes | Before start (TRN-17) | absent | keep | Same question as PLY-11 |
| PLY-13 | Tablebases | Global paths, caches, 50-move rule, probe limit, compression injected; a switch strips every tablebase option | absent | keep | CLI gap or run-file composition: §12 |
| PLY-14 | Ponder | Always sent; allowed with any time control and without CPU separation | only with `--ponder`, clock controls and cores per engine | decide | §12 |
| PLY-15 | Adjudication | Natural endings first, then resign, draw, max moves; resign always two-sided; defaults TRN-18 | same logic; one-sided resign too; other defaults | decide | Defaults: §12 |
| PLY-16 | Faults | Time loss, crash, illegal move and setup failure after spawn lose the game | same | keep | |
| PLY-17 | Unscorable games | A spawn failure is not scored, the tournament goes on, the game is replayed at the next resume | the run stops (infrastructure error), never replayed | decide | §12 |
| PLY-18 | Fault allowance | None | run marked invalid past 1%, minimum 5 | decide | §12, with PLY-17 |
| PLY-19 | Engine identity | "name version" in PGN and every message | label or file stem | keep | This application passes "name version" as the label |
| PLY-20 | PGN tags | Event = tournament name, Site "Colosseum", real date, compact TimeControl (`3+0.03`), margins | fixed Event and Site, unknown date, per-side TimeControl; adds game, pair and opening tags | change | The CLI's tags plus the tournament's name and date |
| PLY-21 | Per-game search statistics | nps (derived from nodes/time when an engine reports 0), depth, time per move | absent | keep | CLI gap (ARN-11) |
| PLY-22 | Ranking and tie-breaks | Sonneborn–Berger, shared places, gauntlet score share | points, then identifier | keep | CLI gap (ARN-12) |
| PLY-23 | Ratings | Joint ML, 6 virtual draws against the prior, anchored to the tournament-start mean, 95% error bars; Chosen pins the others | same model; `--rating` seeds, `--fixed` names the pinned, `[fixed]` shown | keep | Decided 2026-09-29 (PLAN §G2): the CLI computes; this application passes library ratings as seeds, pins what *Chosen* leaves out, and writes the result back |
| PLY-24 | Terminations and forfeits | Breakdown per kind; forfeits per engine | per game only | keep | CLI gap (ARN-18, ARN-11) |
| PLY-25 | Live facts | Search info up to 10 per second plus every new depth; 80-line log; clocks; ponder evaluations | not yet (CLI Phase 13) | keep | CLI 13.3 |
| PLY-26 | Parallel change while running | Yes, stored for resume | fixed per run | decide | §12 |
| PLY-27 | Stop | Games in progress finish, no time limit | 30 s grace, then abandoned | change | CLI 13.4 decides with 4.2 |
| PLY-28 | Force-stop | Games discarded, re-queued first in launch order | abandoned, replayed on resume | keep | |
| PLY-29 | Resume | Explicit Start; a moved executable is repaired from the library | automatic on re-run; any config change refused | change | Changed engines are decided at CLI 11.3 (PLAN §G2) |
| PLY-30 | Play clock | Restarts at zero after a reload | carried in the journal | change | The CLI's |
| PLY-31 | Opening name, SAN, FEN | Computed in the GUI | not emitted | keep | CLI 11.4 protocol; no chess logic here |
| PLY-32 | Several tournaments | Several drivers in one process | one per process | keep | One CLI process per tournament (PLAN §G3) |
| PLY-33 | Persistence | SQLite rows | run directory | drop | CLI ADR-0011 |
| PLY-34 | Write failures | Store and PGN failures silent | fatal | change | The CLI's; reported here |
| PLY-35 | Shared runner quirks | Book and scoreless plies count in adjudication windows; the start position is not counted for threefold; an unparsable start FEN silently plays the standard position; a nodes/depth hang is scored as a time loss | same | change | Reported to the CLI repository as maintenance candidates |

## 9. Product rules from `GUIDELINES.md` (carried over)

1.x's guidelines mix two things. These are rules about the product and
carry into 1.4:

| ID | Rule |
|---|---|
| PR-01 | Engine identity is "name version" wherever one string names an engine |
| PR-02 | Ratings are a joint recompute, never incremental; the Elo shown is what the library holds; Δ is always against tournament start |
| PR-03 | Live numbers never move the layout: fixed-width columns on live data |
| PR-04 | No control changes size on hover or selection |
| PR-05 | Both themes designed and tested equally; text never below 11 px; a semantic colour is always paired with a glyph or text |
| PR-06 | Semantic colour appears as a tint (fill, stroke, text), never as a large saturated fill |
| PR-07 | Edits save themselves after a short pause and at once on leaving; brief "saved" feedback; no Save buttons |
| PR-08 | A destructive action is confirmed naming its target, by one pattern |
| PR-09 | Engine option precedence: library < tournament-wide < per-engine override; overrides are marked where the engine is listed |
| PR-10 | Right-click opens a context menu and selects the item |
| PR-11 | Rarely changed global settings sit collapsed with a one-line summary |
| PR-12 | Images are aspect-fitted, rounded, never cropped; the monogram fallback has the same shape |
| PR-13 | Multi-column forms take their column count from the content, not a fixed guess |
| PR-14 | A detail header keeps its geometry from record to record |
| PR-15 | A clear control sits to the right of a field and appears only when there is something to clear |
| PR-16 | Empty states: glyph, title, hint, and an action where one exists |
| PR-17 | Side panels collapse and expand the same way everywhere |
| PR-18 | No blank window at startup |
| PR-19 | No scrollbar when content fits; a scrollbar never covers content |
| PR-20 | Evaluation graph: White a cool blue, Black a warm orange |
| PR-21 | The live board is bound by height; the engine column takes the width left over |

## 10. egui workarounds (dropped)

These exist because of the toolkit and are not carried over (research §1).

| ID | Workaround |
|---|---|
| EW-01 | `selectable_label` banned in rows; framed choice chips instead |
| EW-02 | `egui::ComboBox` banned for its phantom scrollbar; a menu-button dropdown |
| EW-03 | Glyph allowlist; ✓ ✕ ▶ ▼ ⬜ ⬛ and "…" painted by hand |
| EW-04 | A second font family for real bold |
| EW-05 | Hover expansion forced to zero |
| EW-06 | Scrollbar lane and handle tuning (invisible in light mode otherwise) |
| EW-07 | Two-point inner margins so focus rings survive panel edges |
| EW-08 | Panels instead of frames because frames do not clip |
| EW-09 | Clip only the horizontal axis inside a scroll area |
| EW-10 | Polling background channels with timed repaints |
| EW-11 | Logo decode budget of three per frame and a per-pixel-size texture cache |
| EW-12 | Fixed-size allocation so grid cards align |
| EW-13 | Hidden window revealed after three frames |
| EW-14 | Right-to-left layouts drawn in reverse order; per-row id salts; geometric hover detection; background slots painted before content |
| EW-15 | Popup width and minimum scroll height fixed by hand |
| EW-16 | A global theme flag switched at the start of every frame |
| EW-17 | Windows console re-attachment for a GUI-subsystem executable |

## 11. Defects and lessons

**Defects in 1.x** the new application must not repeat. They are confirmed in
the code unless marked.

| ID | Defect |
|---|---|
| DEF-01 | "Stop & quit" says it lets games finish, but it sends Stop and closes at once; games in progress are killed with the application, as with Force-stop |
| DEF-02 | An unreadable `engines.json` (or `config.toml`) silently becomes an empty library (or defaults), and the next save overwrites it |
| DEF-03 | The engine editor's snapshot writes a stale Elo back over the rating writeback on the next edit |
| DEF-04 | Options hidden from the per-engine editor by substring ("cpu", "thread"), the heuristic the allowlist replaced for mapping |
| DEF-05 | An empty working directory is documented as the engine's folder but is the application's |
| DEF-06 | Folder scan launches every executable in the folder and keeps only the last error |
| DEF-07 | The opening-book preview loads the whole book on the UI thread at every keystroke |
| DEF-08 | Tablebase path fields trim on every keystroke, so a space cannot be typed |
| DEF-09 | Store, PGN, preset, settings and library write failures reach only the log |
| DEF-10 | Engine errors vanish on reload; incident paths are plain text |
| DEF-11 | An unscored game counts as finished in a session and as pending after a reload |
| DEF-12 | Elapsed time and averages restart after a reload, contrary to the 1.0.0 changelog |
| DEF-13 | Two running tournaments updating the same engine overwrite each other's writeback |
| DEF-14 | With one game per pair, colours do not alternate across cycles (the CLI's shared pairing: CLI 11.3.5) |
| DEF-15 | Parallel games capped at the cores in setup but at 256 in the live bar |
| DEF-16 | A rename does not reach later games' PGN `Event` in the same session |
| DEF-17 | Creation writes the tournament before loading its book; a bad book may leave an orphan (unverified) |
| DEF-18 | Loading history runs on the UI thread and loaded tournaments are never released |

**Lessons** recorded along the way (changelog, `CLAUDE.md`, records):

| ID | Lesson |
|---|---|
| LES-01 | Old engines crash and misbehave (Rybka, Junior, Hydra); treat an engine bug as a real possibility and show the evidence (incident reports) |
| LES-02 | Option names vary: an exact allowlist, never substrings (Rybka's "CPU Usage" throttle); names with spaces must still match (Shredder's Syzygy option) |
| LES-03 | Out-of-range values crash some engines (Rybka 3 hash): clamp to the declared range |
| LES-04 | Some engines report `nps 0` (Fruit): derive speed from nodes and time |
| LES-05 | Startup under load is slow: generous handshake timeouts (Deep Junior at 15 parallel games) |
| LES-06 | Stockfish 18 with strength limiting and Syzygy crashes in tablebase endgames: an engine defect, not the harness |
| LES-07 | Per-row inserts froze the UI for minutes: batch, and never block the UI on storage |
| LES-08 | Live numbers made auto-sized columns jitter |
| LES-09 | An update channel is a contract: 1.0.2 could not see 1.1.0 once the tag form changed |
| LES-10 | An unscorable game recorded as a draw quietly moved library ratings (fixed in 1.1.0): a game nobody played must never score |
| LES-11 | Force-stopped games must be re-queued, first and in order, or they are skipped for the session |
| LES-12 | Several documents drifted from the code (guidelines, data paths, `colosseum.db` versus `colosseum.sqlite`): specifications are checked against the implementation at each exit |

## 12. For the maintainer to decide

Each question is a numbered step, so none is lost: 1.1.1–1.1.4 here, before
the requirements are signed off; CLI 11.3.1–11.3.5 there, after the gap
analysis. Answers are dated in the table when taken.

**Here**

| ID | Step | Question | Answer |
|---|---|---|---|
| ENG-10 | 1.1.1 | Delete All: keep behind a modal, or drop? | |
| ENG-19 | 1.1.1 | Clone keeps the same "name version", which is ambiguous in PGN and standings. Keep, or require a distinct version? | |
| ENG-23 | 1.1.1 | Should the library rating show where it came from (typed or a tournament's) and keep a history? | |
| TRN-06 | 1.1.2 | Offer gauntlets with more than one seed? | |
| TRN-21 | 1.1.2 | Keep an optional PGN file outside the run directory, appended as games end, beside the export? | |
| TRN-22 | 1.1.2 | Should a preset also carry the engine selection and the overrides? | |
| PLY-08 | 1.1.2 | The default time allowed past the clock (`--margin-ms`; 1.x 2 s, CLI 20 ms), and may the user change it? | |
| ARN-05 | 1.1.3 | Should deleting a tournament revert the ratings it wrote into the library? | |
| LIV-09 | 1.1.3 | Evaluation from White's point of view (1.x live view) or the side to move (1.x PGN comments), or switchable? | |
| APP-06 | 1.1.4 | Check for updates automatically at startup? | |
| APP-08 | 1.1.4 | Keep a portable mode, and by flag or by a marker file? | |
| APP-15 | 1.1.4 | Rating writeback conflicts: two tournaments updating one engine, and a typed rating during a run | |
| APP-22 | 1.1.4 | Refuse a second instance on the same data? | |
| APP-23 | 1.1.4 | Keep the machine awake during a run; notify when a tournament finishes; progress on the taskbar or Dock? | |

**In the CLI repository**

| IDs | Step | Question |
|---|---|---|
| PLY-11, PLY-12, PLY-13 | CLI 11.3.1 | Threads and hash mapping, compatibility notes and tablebase settings: in the CLI, or composed here from the library's detected options into the run file? |
| PLY-14 | CLI 11.3.2 | Ponder: `Ponder=false` when off; allowed without a clock and without separate cores? |
| PLY-15, PLY-35 | CLI 11.3.3 | Adjudication defaults offered here, one-sided resign, book and scoreless plies in the windows, the start position in threefold |
| PLY-17, PLY-18, PLY-35 | CLI 11.3.4 | An engine that cannot start: stop the tournament (CLI) or skip and replay later (1.x)? A fault allowance that marks a tournament invalid? A fixed-search hang: time loss or crash? |
| PLY-26, DEF-14 | CLI 11.3.5 | Parallel games changed while a tournament runs; colours across cycles at one game per pair |

**Settled 2026-09-29**

- **Ratings** (PLY-23, APP-15 context): the CLI computes; this application
  keeps the library (PLAN §G2).
- **Time past the clock** (PLY-08): the CLI already takes any
  `--margin-ms`, so only this application's default is open (1.1.2).

## 13. Absent in 1.x, noted for 1.2

Raised while reading, not inventoried: library import and export; drag and
drop of executables; an engine health check showing the UCI transcript;
Polyglot books; moves-per-session and per-engine time controls; a visible
time margin; tablebase adjudication; tournament search, duplication and
re-run; configuration edits after creation; per-engine drill-down and rating
history; copying the FEN or PGN of the watched game; board flip and PV
arrows; a shortcut map.
