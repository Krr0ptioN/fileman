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

## User Stories

1. As a stiff user, I want filename search in the current tree, so that I can find files without leaving the browser.
2. As a stiff user, I want to cancel or escape search and restore the prior listing, so that search never traps me.
3. As a stiff user, I want browser tabs per pane, so that I can keep multiple directories open without dual-pane only.
4. As a stiff user, I want to open, switch, and close tabs from the keyboard, so that tab workflow stays Vim-native.
5. As a stiff user, I want back/forward directory history, so that I can retrace navigation like a browser.
6. As a stiff user, I want to enter archives as directories, so that I can browse zip/tar contents without extracting first.
7. As a stiff user, I want to extract and create archives, so that packing workflows stay inside stiff.
8. As a stiff user, I want to edit text files in-app, so that small fixes do not require an external editor round-trip.
9. As a stiff user, I want SFTP browsing with the same commands as local FS, so that remote work feels local.
10. As a stiff user, I want elevated chmod/chown when permission is denied, so that admin tasks do not force an external terminal.
11. As a stiff user, I want letter bookmarks, so that I can jump to frequent paths instantly.
12. As a stiff user, I want file tags, so that I can classify entries independently of directory layout.
13. As a stiff user, I want configurable sort modes, so that I can order by name, size, mtime, or type.
14. As a stiff user, I want incremental filter/typeahead, so that long directories shrink to matching rows while I type.
15. As a stiff user, I want a flattened recursive listing, so that I can operate across a subtree without drilling.
16. As a stiff user, I want bulk rename, so that I can rename many marked files in one pass.
17. As a stiff user, I want open-with rules (rifle-style), so that Enter opens the right program per file type.
18. As a stiff user, I want to run shell commands on the selection, so that arbitrary tools stay one keystroke away.
19. As a stiff user, I want trash-aware delete, so that accidental deletes are recoverable by default.
20. As a stiff user, I want create file, symlink, and hardlink actions, so that filesystem graph edits are first-class.
21. As a stiff user, I want metadata columns (size, mtime, mode, owner), so that I can inspect entries without `stat`.
22. As a stiff user, I want VCS status in the listing, so that dirty/ignored/untracked files are visible while browsing a repo.
23. As a stiff user, I want fuzzy path jump (fzf/zoxide-style), so that deep paths are reachable without typing full paths.
24. As a stiff user, I want macros and colon commands, so that I can compose and repeat multi-step workflows.
25. As a stiff user, I want optional Miller columns, so that parent/current/child context is visible at once.
26. As a stiff user, I want undo for recent file operations, so that mistakes are cheap to reverse.
27. As a stiff user, I want paste conflict resolution, so that overwrites are explicit rather than silent.
28. As a stiff user, I want a config file and remappable keys, so that stiff fits my muscle memory.
29. As a stiff user, I want a plugin surface, so that community extensions can add commands without forking.
30. As a stiff user, I want SMB/WebDAV (and similar) remotes, so that non-SFTP shares work in the same browser.
31. As a stiff user, I want a thumbnail/grid mode, so that image-heavy folders are scannable visually.
32. As a stiff user, I want pane sync and compare, so that dual-pane copy/review workflows are deliberate.
33. As a stiff user, I want disk-usage insight, so that I can find what consumes space.
34. As a stiff user, I want a visible task/progress queue, so that long copies do not freeze or hide status.
35. As a stiff user, I want glob-based marking, so that I can select sets by pattern.
36. As a stiff user, I want one-key copy/move to the opposite pane, so that dual-pane transfers match classic FM muscle memory.
37. As a stiff user, I want mount/volume browsing, so that removable and network mounts are discoverable.
38. As a stiff user, I want content search (ripgrep-style), so that I can find files by contents, not only names.
39. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
40. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

---

## Feature Requirements

Each subsection is a complete requirements specification for one feature.
Priority bands: **P0** (parity blockers for daily Ranger/xplr use), **P1**
(strong power-user), **P2** (differentiation / later).

Shared non-functional requirements for all features:

- NF-1: Directory loads, searches, remote IO, archive reads, and previews must not block the UI input path.
- NF-2: Domain transitions must be expressible as commands/effects without GPUI types.
- NF-3: Stale async generations must be ignored.
- NF-4: Virtualized row rendering remains mandatory for large listings.
- NF-5: Existing GPUI browser commands must keep current behavior unless a feature explicitly changes them.
- NF-6: Prefer reusing `core`, `archive`, `sftp`, `elevate`, and `clipboard` logic over reimplementing IO.

---

### F01 — Filename Search (P0)

**Intent:** Find and navigate to entries by name within a configurable scope
(current directory or recursive under current root).

**Functional requirements**

1. FR-1.1: User can open filename search from a keybinding (prior art: Alt+F7 in replay cases).
2. FR-1.2: Search enters an input mode; normal browser chains are suspended until submit/cancel.
3. FR-1.3: On submit, results replace or overlay the active panel listing with matching entries.
4. FR-1.4: Escape/cancel restores the previous listing and selection when possible.
5. FR-1.5: Empty query submit is a no-op (no spurious mkdir/create).
6. FR-1.6: Matching is case-insensitive by default; case-sensitive mode is configurable later.
7. FR-1.7: Search runs off the UI thread; progress/status is visible while running.
8. FR-1.8: Opening a result navigates to its parent and selects the entry (or opens if directory policy says so).
9. FR-1.9: Dot-hidden / gitignore visibility settings apply to search results consistently with browse mode, unless search opts into “include hidden.”

**Acceptance criteria**

- AC-1.1: Replay-equivalent: open search, type a known name, Enter, then Escape restores original listing.
- AC-1.2: No filesystem mutation occurs from search alone.
- AC-1.3: Cancelling a slow search does not apply a later result set.

**Out of this feature:** content/ripgrep search (see F38).

---

### F02 — Browser Tabs (P0)

**Intent:** Multiple directory contexts per pane without requiring dual-pane.

**Functional requirements**

1. FR-2.1: Each browser pane owns an ordered tab list with one active tab.
2. FR-2.2: New tab clones the current directory (and later: optional empty/home).
3. FR-2.3: Next/previous tab switching is keyboard-driven.
4. FR-2.4: Close tab activates a neighbor; closing the last tab is a no-op (pane remains).
5. FR-2.5: Tab state includes cwd, selection, marks, scroll/reveal intent, and visibility flags owned by that tab.
6. FR-2.6: Dual-pane mode keeps independent tab stacks per pane.
7. FR-2.7: Reload and navigation mutate only the active tab.

**Acceptance criteria**

- AC-2.1: Open tab, navigate away, switch back — prior directory and selection restored.
- AC-2.2: Close non-last tab does not close the pane.
- AC-2.3: Panel switch (Tab) still switches panes, not tabs, unless remapped.

---

### F03 — Navigation History (P0)

**Intent:** Back/forward through visited directories per tab/pane.

**Functional requirements**

1. FR-3.1: Successful directory changes push history (open selected, open parent, quick jump, search jump).
2. FR-3.2: Back restores previous cwd and best-effort selection.
3. FR-3.3: Forward works after back until a new branch truncates forward stack.
4. FR-3.4: History is per tab (preferred) or per pane if tabs absent.
5. FR-3.5: Failed loads do not push history.
6. FR-3.6: Keys: prior art Alt+Left / Alt+Right / Backspace for back.

**Acceptance criteria**

- AC-3.1: Enter dir → back → forward returns to entered dir.
- AC-3.2: After back, navigating elsewhere clears forward stack.

---

### F04 — Archive Browse, Extract, Compress (P0)

**Intent:** Treat supported archives as navigable containers; pack/unpack explicitly.

**Functional requirements**

1. FR-4.1: Opening a supported archive enters container browse mode with listing from existing archive core.
2. FR-4.2: Parent/back exits nested archive paths correctly (including nested archives if supported).
3. FR-4.3: Preview of archive lists entries without full extract.
4. FR-4.4: Extract command writes selected/marked entries to a chosen destination (default: cwd or opposite pane).
5. FR-4.5: Create-archive command packs selection into a chosen format (zip/tar/…).
6. FR-4.6: Copy/move/delete inside archives follow existing container IO helpers; unsupported ops report clear status.
7. FR-4.7: Progress reported for large pack/unpack.

**Acceptance criteria**

- AC-4.1: Navigate into archive, into subdir, back out to FS without crash.
- AC-4.2: Extract produces expected files; cancel mid-op does not leave silent half-state without status.

---

### F05 — In-App Editor (P0)

**Intent:** Edit text files without leaving stiff for small changes.

**Functional requirements**

1. FR-5.1: Open selected text file in editor mode/pane.
2. FR-5.2: Insert/selection editing with save and discard/cancel.
3. FR-5.3: Dirty buffer warning on close/navigate away.
4. FR-5.4: Binary/unsupported types refuse edit with status, offer open-with instead.
5. FR-5.5: New-file flow can create then edit (coordinates with F20).
6. FR-5.6: Saves write atomically when feasible (temp + rename).

**Acceptance criteria**

- AC-5.1: Edit, save, reopen shows new contents.
- AC-5.2: Cancel discard leaves file unchanged.

---

### F06 — SFTP Remote Browsing (P0)

**Intent:** Browse and operate on SFTP paths with the same command model as local FS.

**Functional requirements**

1. FR-6.1: Connect using ssh config hosts and/or explicit user@host.
2. FR-6.2: Directory listing, open, parent, marks, copy/move/delete, mkdir, rename work remotely via existing SFTP module.
3. FR-6.3: Connection failure surfaces actionable status (auth, host key, network).
4. FR-6.4: Sessions are reusable; disconnect is explicit or on quit.
5. FR-6.5: Mixed local/remote dual-pane transfers are supported where core already allows.
6. FR-6.6: Remote paths display distinctly in chrome (host + path).

**Acceptance criteria**

- AC-6.1: Connect, list, enter dir, copy file to local pane succeeds in integration test environment.
- AC-6.2: Failed auth does not hang the UI thread.

---

### F07 — Elevated Permissions (chmod / chown / privileged IO) (P1)

**Intent:** Recover from permission errors and change mode/owner when elevation is available.

**Functional requirements**

1. FR-7.1: chmod command prompts for mode (octal or symbolic) for selection/marks.
2. FR-7.2: chown command prompts for owner/group when platform supports it.
3. FR-7.3: When an operation fails with EACCES/EPERM and elevation is available, offer elevated retry.
4. FR-7.4: Elevation uses existing elevate helpers; unavailable elevation reports how to enable.
5. FR-7.5: Recursive flag is explicit for directories.

**Acceptance criteria**

- AC-7.1: chmod on owned file updates mode visible in metadata (F21) or `stat`.
- AC-7.2: Denied op without elevation shows clear status, no silent no-op.

---

### F08 — Bookmarks (P0)

**Intent:** Persist letter (or named) bookmarks for instant jumps.

**Functional requirements**

1. FR-8.1: Set bookmark on current directory (and optionally selected path) to a slot (e.g. `m` then letter).
2. FR-8.2: Jump to bookmark (`'` then letter) loads that path in the active tab.
3. FR-8.3: Bookmarks persist across sessions in user config/state.
4. FR-8.4: List bookmarks in help or a picker.
5. FR-8.5: Missing target bookmark reports status and does not crash.
6. FR-8.6: Bookmarks may point at local or remote URIs once remotes exist.

**Acceptance criteria**

- AC-8.1: Set bookmark, change dir, jump returns to bookmarked path.
- AC-8.2: Restart app; bookmarks still available.

---

### F09 — Tags (P2)

**Intent:** Assign lightweight tags to paths for classification outside folders.

**Functional requirements**

1. FR-9.1: Add/remove tag on selection/marks.
2. FR-9.2: Filter listing or search by tag.
3. FR-9.3: Tags persist in a sidecar store (not requiring xattrs, though xattrs may be optional later).
4. FR-9.4: Row UI shows tag indicator without breaking virtualization budgets.
5. FR-9.5: Untagged default path remains unchanged for users who ignore tags.

**Acceptance criteria**

- AC-9.1: Tag file, filter by tag, only tagged visible.
- AC-9.2: Remove tag clears filter membership.

---

### F10 — Sort Modes (P0)

**Intent:** User-controlled ordering of directory rows.

**Functional requirements**

1. FR-10.1: Sort keys: name, size, mtime, extension/kind; secondary name tie-break.
2. FR-10.2: Toggle directories-first.
3. FR-10.3: Toggle ascending/descending.
4. FR-10.4: Sort setting is per tab (preferred) and persists optionally.
5. FR-10.5: Sort applies after filter/flat transforms.
6. FR-10.6: Natural/version-aware name sort is preferred for name mode.

**Acceptance criteria**

- AC-10.1: Switching to size desc places largest files first among files.
- AC-10.2: Directories-first on/off changes order predictably.

---

### F11 — Incremental Filter / Typeahead (P0)

**Intent:** Narrow the current listing as the user types, without a separate search results page.

**Functional requirements**

1. FR-11.1: Activate filter mode; typed characters narrow visible rows.
2. FR-11.2: Selection stays on best match; movement operates within filtered set.
3. FR-11.3: Clear filter restores full listing and prior selection when possible.
4. FR-11.4: Filter does not mutate disk; marks outside filter remain until cleared.
5. FR-11.5: Optional fuzzy vs substring match modes.
6. FR-11.6: Works with large dirs via index/filter over row model, not full re-read.

**Acceptance criteria**

- AC-11.1: Type unique prefix → one row → Enter opens that entry.
- AC-11.2: Escape clears filter and shows full list.

---

### F12 — Flattened Recursive View (P1)

**Intent:** Show a recursive file list under the current root for bulk ops.

**Functional requirements**

1. FR-12.1: Toggle flat mode lists files (and optionally dirs) under cwd recursively.
2. FR-12.2: Depth limit or cancel for huge trees.
3. FR-12.3: Relative path shown in name/detail so collisions are distinguishable.
4. FR-12.4: Marks/ops apply to underlying real paths.
5. FR-12.5: Exiting flat mode restores normal listing.

**Acceptance criteria**

- AC-12.1: Flat view includes nested file; marking + copy copies the real nested path.
- AC-12.2: Cancel during flatten does not leave partial silent state without status.

---

### F13 — Bulk Rename (P1)

**Intent:** Rename many marked entries via a structured multi-rename UI or pattern.

**Functional requirements**

1. FR-13.1: Requires one or more marked (or selected) entries.
2. FR-13.2: Presents editable target names (buffer/list) or pattern (find/replace, numbering).
3. FR-13.3: Validates collisions and illegal names before applying.
4. FR-13.4: Applies renames sequentially/transactionally with per-item status; stops or continues on error per policy.
5. FR-13.5: Dry-run preview of resulting names.
6. FR-13.6: Undo integration when F26 exists.

**Acceptance criteria**

- AC-13.1: Two marked files renamed to unique names both succeed.
- AC-13.2: Conflicting targets aborted before any rename (or rolled back if partial policy chosen — document choice in implementation).

---

### F14 — Open-With / Rifle Rules (P0)

**Intent:** Map file types to external programs with ordered rules.

**Functional requirements**

1. FR-14.1: Rule table: match by mime/extension/name → command template.
2. FR-14.2: Enter/open uses first matching rule; fallback to system open.
3. FR-14.3: User can pick from multiple matches (open-with picker).
4. FR-14.4: Directories have separate policy (enter vs open in file manager vs external).
5. FR-14.5: Rules loaded from config (F28); sane built-in defaults ship.
6. FR-14.6: Commands run async; failures show stderr/status summary.

**Acceptance criteria**

- AC-14.1: Configured extension opens with configured program in test double/harness.
- AC-14.2: Unknown type falls back without crash.

---

### F15 — Shell Command on Selection (P0)

**Intent:** Run `$SHELL -c` / explicit command with selection path placeholders.

**Functional requirements**

1. FR-15.1: Prompt for command with placeholders (`%f` selected, `%s` marked, `%d` cwd — final tokens TBD, document in help).
2. FR-15.2: Foreground wait vs background run modes (Ranger `!` vs `&` analogues).
3. FR-15.3: Working directory is active panel cwd.
4. FR-15.4: Output optionally captured to status/preview pane (P1 enhancement).
5. FR-15.5: Confirm when command is empty or dangerous patterns if policy enabled.
6. FR-15.6: Does not block key input handling thread; process IO is async.

**Acceptance criteria**

- AC-15.1: `echo %f` with selection runs and reports success.
- AC-15.2: Background mode returns focus immediately.

---

### F16 — Trash-Aware Delete (P0)

**Intent:** Default delete moves to platform trash; permanent delete is explicit.

**Functional requirements**

1. FR-16.1: Default delete confirmation states trash vs permanent based on config.
2. FR-16.2: Permanent delete remains available (current `dD`/`x` semantics may map to permanent or become two commands).
3. FR-16.3: Trash backend uses Freedesktop trash on Linux when available; clear fallback messaging otherwise.
4. FR-16.4: Marked multi-delete supported.
5. FR-16.5: Failures per item reported; partial success listed.

**Acceptance criteria**

- AC-16.1: Trash delete removes from listing and appears in trash (or documented fallback).
- AC-16.2: Permanent delete removes file without trash entry.

---

### F17 — Create File, Symlink, Hardlink (P0)

**Intent:** Create non-directory nodes and links.

**Functional requirements**

1. FR-17.1: New empty file with name prompt (alongside existing mkdir).
2. FR-17.2: Create symlink: target = selection (or prompted), link name prompted.
3. FR-17.3: Create hardlink where filesystem allows; error if cross-device.
4. FR-17.4: Collision handling consistent with rename/mkdir.
5. FR-17.5: After create, listing reloads and selects new entry.

**Acceptance criteria**

- AC-17.1: New file appears and is empty.
- AC-17.2: Symlink resolves to intended target.
- AC-17.3: Hardlink failure on cross-device is explicit.

---

### F18 — Metadata Columns (P1)

**Intent:** Show size, mtime, permissions, owner/group in the row or detail region.

**Functional requirements**

1. FR-18.1: Columns are toggleable; narrow layouts degrade to detail string (current `detail` field).
2. FR-18.2: Values come from listing metadata; expensive owner lookups cached.
3. FR-18.3: Symlinks show link target or decorated name without breaking sort.
4. FR-18.4: Column set is configurable (F28).
5. FR-18.5: Virtualization: column text is precomputed in row model, not per-paint FS calls.

**Acceptance criteria**

- AC-18.1: Enabling mtime column shows stable timestamps after load.
- AC-18.2: No FS syscall storm while scrolling a large dir.

---

### F19 — VCS Status in Listing (P1)

**Intent:** Show git (initial VCS) status decorations on rows.

**Functional requirements**

1. FR-19.1: Detect repo root for cwd; decorate modified/added/untracked/ignored/conflicted.
2. FR-19.2: Status computation is async and coalesced; stale updates ignored.
3. FR-19.3: Toggle decorations on/off.
4. FR-19.4: Works with existing gitignore hide (`gH`) without contradiction.
5. FR-19.5: Non-git directories show no decorations (no error spam).

**Acceptance criteria**

- AC-19.1: Modified file in a test repo shows modified decoration.
- AC-19.2: Outside a repo, listing unchanged aside from absent decorations.

---

### F20 — Fuzzy Path Jump (fzf / zoxide style) (P1)

**Intent:** Jump to deep paths via fuzzy finder and/or frecency.

**Functional requirements**

1. FR-20.1: Fuzzy finder over files/dirs under a root or from a path index.
2. FR-20.2: Optional zoxide/frecency backend for directory jump.
3. FR-20.3: Extends or complements existing Quick Jump (`Ctrl+G` path entry) — Quick Jump remains for exact/relative path typing.
4. FR-20.4: Selection loads directory or selects file.
5. FR-20.5: External tool integration allowed (spawn fzf) **or** in-app fuzzy list — pick one primary in implementation decision; document it.

**Acceptance criteria**

- AC-20.1: Fuzzy match unique deep path jumps correctly.
- AC-20.2: Cancel leaves cwd unchanged.

---

### F21 — Macros and Colon Commands (P1)

**Intent:** Compose multi-command sequences and run named/ex commands.

**Functional requirements**

1. FR-21.1: Colon prompt parses command name + args into domain commands.
2. FR-21.2: Built-in commands mirror essential browser ops (cd, delete, mark, sort, filter, …).
3. FR-21.3: Macro record/replay of command sequences (optional P2 if colon lands first).
4. FR-21.4: Unknown commands error in status, no panic.
5. FR-21.5: Help lists colon commands.

**Acceptance criteria**

- AC-21.1: `:sort size` (example) changes sort mode via command seam.
- AC-21.2: Invalid colon input shows error status.

---

### F22 — Miller Columns (P2)

**Intent:** Optional multi-column cascade (parent | current | child preview).

**Functional requirements**

1. FR-22.1: Toggle Miller layout for the active browser workspace.
2. FR-22.2: Left column shows parent listing; center current; right child of selection if directory.
3. FR-22.3: Keyboard focus model defined (which column receives j/k).
4. FR-22.4: Compatible with dual-pane mode or mutually exclusive — decision documented.
5. FR-22.5: Performance: only visible columns load; child column cancels stale loads.

**Acceptance criteria**

- AC-22.1: Selecting a directory populates child column without blocking center navigation.
- AC-22.2: Disabling Miller restores prior layout.

---

### F23 — Undo File Operations (P1)

**Intent:** Reverse recent mutating ops where safely possible.

**Functional requirements**

1. FR-23.1: Stack of undo records for rename, move, copy (optional), trash-delete, mkdir, create file.
2. FR-23.2: Permanent delete and opaque external shell effects are non-undoable (reported).
3. FR-23.3: Undo command pops and applies inverse; failures report and keep stack consistent.
4. FR-23.4: Stack size bounded and cleared on quit or optionally persisted (default: session-only).
5. FR-23.5: Status announces what was undone.

**Acceptance criteria**

- AC-23.1: Rename then undo restores original name.
- AC-23.2: Trash-delete then undo restores path when trash supports it.

---

### F24 — Paste Conflict Resolution (P0)

**Intent:** When paste/copy/move hits existing names, user chooses policy.

**Functional requirements**

1. FR-24.1: Detect collisions before or during paste plan.
2. FR-24.2: Policies: skip, overwrite, rename (auto suffix), cancel remaining.
3. FR-24.3: Apply-to-all option for batch.
4. FR-24.4: Default policy configurable (F28); safe default is prompt.
5. FR-24.5: Integrates with clipboard paste planner.

**Acceptance criteria**

- AC-24.1: Paste onto existing name with skip leaves destination unchanged and continues batch.
- AC-24.2: Cancel aborts remaining planned ops.

---

### F25 — Config File and Key Remapping (P0)

**Intent:** User-editable configuration for keys, defaults, and feature toggles.

**Functional requirements**

1. FR-25.1: Documented config path and format (TOML or RON — pick one).
2. FR-25.2: Remap key sequences to existing `BrowserCommand`s / control actions.
3. FR-25.3: Configure defaults: sort, trash vs permanent, hidden, columns, rifle rules path.
4. FR-25.4: Invalid config loads defaults + error status rather than refusing to start (or fail-soft with dialog).
5. FR-25.5: Reload config command without full restart when safe.
6. FR-25.6: Ship example config in repo docs.

**Acceptance criteria**

- AC-25.1: Remapped key invokes target command in test.
- AC-25.2: Broken config does not hard-crash startup.

---

### F26 — Plugins (P2)

**Intent:** Extend commands and hooks without forking stiff.

**Functional requirements**

1. FR-26.1: Define a minimal plugin API: register command name, invoke on selection context, return effect/status.
2. FR-26.2: Plugins cannot block UI thread; long work is async.
3. FR-26.3: Load from a plugins directory; failures isolate to that plugin.
4. FR-26.4: Security: no implicit network; document trust model (local code executes with user privileges).
5. FR-26.5: First version may be process-spawn plugins (external binaries) before in-process scripting.

**Acceptance criteria**

- AC-26.1: Sample plugin registers and runs from colon/key.
- AC-26.2: Crashing plugin does not take down stiff process (for out-of-process); in-process must catch panics at boundary if used.

---

### F27 — Additional Remotes (SMB / WebDAV / …) (P2)

**Intent:** Browse non-SFTP network shares with the same browser UX.

**Functional requirements**

1. FR-27.1: Pluggable remote backend interface shared with SFTP patterns (list, read, write, mkdir, rename, delete).
2. FR-27.2: SMB and/or WebDAV as first additional backends.
3. FR-27.3: Auth prompts and credential caching policy documented.
4. FR-27.4: URI scheme in chrome and bookmarks.
5. FR-27.5: Feature-detect unavailable backends cleanly on unsupported platforms.

**Acceptance criteria**

- AC-27.1: Against a test server/fixture, list and copy one file to local.
- AC-27.2: Unsupported build shows clear “backend unavailable” status.

---

### F28 — Thumbnail / Grid Mode (P2)

**Intent:** Visual browsing for image-heavy directories.

**Functional requirements**

1. FR-28.1: Toggle list vs grid/thumbnail layout for the active panel.
2. FR-28.2: Thumbnails decoded off UI thread; cache bounded (memory + count).
3. FR-28.3: Keyboard selection and marks work in grid.
4. FR-28.4: Non-image files show icons/placeholders.
5. FR-28.5: Scroll virtualizes grid cells.

**Acceptance criteria**

- AC-28.1: Opening a folder of images shows thumbnails without freezing input.
- AC-28.2: Memory cache respects bound under stress test of many images.

---

### F29 — Pane Sync and Compare (P1)

**Intent:** Coordinate dual panes for transfer and difference review.

**Functional requirements**

1. FR-29.1: Sync cwd: secondary follows primary (optional relative sync).
2. FR-29.2: One-key copy/move selection to opposite pane destination.
3. FR-29.3: Compare mode highlights names present only on left/right (basic name set diff; content diff P2).
4. FR-29.4: Sync can be toggled off; no surprising cwd changes when off.
5. FR-29.5: Works with tabs (sync applies to active tabs).

**Acceptance criteria**

- AC-29.1: Copy-to-opposite lands files in other pane cwd.
- AC-29.2: Compare marks unique names on each side.

---

### F30 — Disk Usage Insight (P1)

**Intent:** Show space used by entries to find large subtrees.

**Functional requirements**

1. FR-30.1: Compute du-like sizes for directories async with cancel.
2. FR-30.2: Display in detail/column when ready; show progress placeholder meanwhile.
3. FR-30.3: Sort by computed size when available (integrates F10).
4. FR-30.4: Free space for volume containing cwd shown in status/chrome.

**Acceptance criteria**

- AC-30.1: Directory size eventually populates and matches rough `du` order.
- AC-30.2: Cancel/navigate away drops stale du updates.

---

### F31 — Task / Progress Queue (P0)

**Intent:** Make long operations visible, cancellable, and non-blocking.

**Functional requirements**

1. FR-31.1: Mutating multi-file ops enqueue as tasks with progress (bytes/items).
2. FR-31.2: Status bar or panel lists running/queued/completed/failed.
3. FR-31.3: Cancel task best-effort; partial results reported.
4. FR-31.4: UI navigation remains responsive while tasks run.
5. FR-31.5: Errors aggregate without modal spam (summary + expandable detail P1).

**Acceptance criteria**

- AC-31.1: Large copy shows progress and completes with summary.
- AC-31.2: Cancel mid-copy stops further items and reports partial state.

---

### F32 — Glob Marking (P1)

**Intent:** Mark entries by glob/pattern in the current listing (or flat view).

**Functional requirements**

1. FR-32.1: Prompt for glob; mark all matches in current row model.
2. FR-32.2: Unmark-by-glob supported.
3. FR-32.3: Invalid glob reports status.
4. FR-32.4: Respects hidden/ignore visibility of current model.

**Acceptance criteria**

- AC-32.1: `*.txt` marks all visible text files.
- AC-32.2: No matches → status, marks unchanged.

---

### F33 — Mount / Volume Browser (P2)

**Intent:** Discover and open mounts, removable media, and common roots.

**Functional requirements**

1. FR-33.1: List mounts/volumes appropriate to OS.
2. FR-33.2: Jump opens that path in active tab.
3. FR-33.3: Refresh detects new mounts.
4. FR-33.4: Unreadable mounts show error status.

**Acceptance criteria**

- AC-33.1: Selecting a listed mount changes cwd to its path.
- AC-33.2: Empty mount list is non-fatal.

---

### F34 — Content Search (P1)

**Intent:** Find files by contents (ripgrep-style) from cwd.

**Functional requirements**

1. FR-34.1: Query prompt; recursive search with ignore rules aligned to visibility settings.
2. FR-34.2: Results list path + optional line preview snippet.
3. FR-34.3: Open result jumps to file; editor may land on line when F05 exists.
4. FR-34.4: Cancel stops worker; ignore stale results.
5. FR-34.5: Binary files skipped by default.

**Acceptance criteria**

- AC-34.1: Unique string in fixture tree returns that file.
- AC-34.2: Escape/cancel restores prior UI context.

---

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
