# Feature Parity — Shared Requirements

# stiff Feature Parity PRD — Ranger, xplr, and Peer File Managers

## Problem Statement

stiff’s GPUI shell already covers fast Vim-style browsing, dual panes, marks,
copy/move/paste, delete, rename, mkdir, preview, and hidden/gitignore toggles.
Power users coming from Ranger, xplr, lf, yazi, and similar keyboard-first file
managers still hit gaps: search, tabs, history, archives, remote filesystems,
bookmarks, tags, sorting, filtering, bulk rename, open-with rules, shell
hooks, trash, metadata, VCS cues, fuzzy jump, macros, Miller columns, undo,
conflicts, configuration, plugins, extra remotes, thumbnails, and pane sync.

Without a single requirements map, those gaps land ad hoc, break the command
seam, or ship without acceptance criteria.

## Solution

Deliver feature parity as vertical slices that extend the existing
framework-neutral command model. Each feature below is a full requirements
specification. Implementation adds domain commands and effects first; the
shell owns async IO, clipboard, external open, and GPUI presentation. Legacy
replay coverage is preserved or replaced by command-level tests until a
UI-neutral harness is ready.

## Primary Test Seam

- **Primary:** `BrowserCommand` (and related input-mode / effect outcomes)
  owns domain behavior. Prefer extending this enum (or closely related
  command enums) over new parallel control paths.
- **Secondary:** `keybind` maps keys to commands only.
- **Tertiary:** replay / GPUI smoke after command behavior is proven.

Shell owns runtime effects (async loads, elevate, system clipboard, open-with,
scroll/reveal). Preview keeps the existing preview-handler seam.

---

## Shared Non-Functional Requirements

All feature specs inherit these constraints:

- NF-1: Directory loads, searches, remote IO, archive reads, and previews must not block the UI input path.
- NF-2: Domain transitions must be expressible as commands/effects without GPUI types.
- NF-3: Stale async generations must be ignored.
- NF-4: Virtualized row rendering remains mandatory for large listings.
- NF-5: Existing GPUI browser commands must keep current behavior unless a feature explicitly changes them.
- NF-6: Prefer reusing `core`, `archive`, `sftp`, `elevate`, and `clipboard` logic over reimplementing IO.

## Implementation Decisions

1. Extend the framework-neutral browser command seam (`BrowserCommand` and
   related input modes / effects) as the primary integration point for each
   feature above; keybind only maps keys → commands.
2. Shell retains ownership of async tasks, elevation, system clipboard, and
   external process spawn (open-with, shell, fuzzy external tools).
3. Reuse existing `archive`, `sftp`, `elevate`, `clipboard`, and preview
   handlers before introducing parallel IO stacks.
4. Prefer per-tab state for cwd, history, marks, sort, and filter once tabs
   exist; until then, per-pane state is acceptable.
5. Config format will be chosen once (TOML recommended) and documented with an
   example; rifle/open-with rules may live in the same file or an adjacent one.
6. Trash is the safe default on platforms with a trash API; permanent delete
   remains explicit.
7. Plugins start as out-of-process command providers if in-process scripting
   would delay P0 parity.
8. Miller columns and thumbnail grid are layout modes gated behind toggles and
   must preserve virtualization budgets from ADR-001 / performance plan.
9. Conflict resolution hooks into the clipboard paste planner rather than ad
   hoc copy paths.
10. Features already sketched in legacy replay (`search`, `tabs`, `history`,
    `archive_*`, `edit*`) should be ported to the GPUI command seam with
    command-level tests first, then replay/smoke.
11. Quick Jump (`Ctrl+G`) remains exact/relative path entry; fuzzy jump is a
    separate feature (F20), not a replacement.
12. Do not block P0 browser parity on plugins, Miller columns, SMB/WebDAV, or
    thumbnail grid.

## Testing Decisions

1. Good tests assert external behavior: command outcomes, panel/tab state,
   filesystem effects, and status — not widget trees or private caches.
2. Primary tests live beside the command/executor modules (existing
   `file_browser` command tests and keybind tests as prior art).
3. Replay cases under `tests/cases/` are prior art for search, tabs, history,
   archive navigation, edit, and selection; extend or replace with
   UI-neutral harness as GPUI migration allows.
4. Remote and elevation features need integration tests behind fixtures or
   feature gates; do not require live network in default `cargo test`.
5. Performance-sensitive features (flat view, thumbnails, du, large search)
   need tests or benches that prove cancellation and stale-generation ignore.
6. Each feature slice must add tests for at least: happy path, cancel/escape,
   and one failure/edge path listed in its acceptance criteria.

## Out of Scope

- Pixel-perfect clones of Ranger, xplr, lf, or yazi UIs or exact default keymaps.
- Replacing GPUI with a terminal UI.
- Full IDE/LSP editing (in-app editor is intentionally small).
- Arbitrary FUSE providers beyond declared remote backends.
- Cloud vendor SDKs (S3, Drive, etc.) in this parity pass.
- Collaborative/multi-user locking.
- Mobile/touch-first redesign.
- Guaranteeing undo for every possible shell plugin side effect.
- Shipping a plugin marketplace.

## Further Notes

- Domain vocabulary: browser panel, active pane, marks, `BrowserCommand`,
  command effects, input mode, preview pane, quick jump, container/archive
  path, shell runtime effects — per `docs/architecture/app-architecture.md`
  and ADR-001.
- Interpret “xlpr” in the originating request as **xplr**.
- Peer references for gap analysis: Ranger (bookmarks, rifle, macros, miller,
  tags), xplr (modes, message-passing customization), lf/yazi (performance,
  previews, config). stiff should absorb capabilities, not their architectures.
- Suggested delivery order: F31 (progress) + F24 (conflicts) underpin safe ops;
  then F01–F06, F08, F10–F11, F14–F17, F25; then P1; then P2.
- This document is the authoritative requirements list for feature-parity
  work; migration mechanics remain in `docs/PRD.md` and architecture ADRs.

