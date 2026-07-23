
# stiff GPUI Migration PRD

## Summary

stiff is migrating from the legacy egui shell to a GPUI-based desktop shell
while preserving the current two-panel file manager behavior. The migration
must proceed in vertical slices so the legacy UI remains shippable until GPUI
has parity for browser navigation, key handling, file operations, preview, and
editing.

## Goals

- Keep keyboard navigation immediate, including Vim/ranger-style command chains.
- Keep filesystem, archive, preview, image decode, and SFTP work off the UI
  thread.
- Render large directory listings with virtualized rows only.
- Reuse existing framework-neutral filesystem and operation code.
- Separate input parsing, browser state transitions, and GPUI runtime effects.
- Preserve existing replay behavior for the legacy UI during migration.

## Non-Goals

- Replacing the legacy egui runtime in one pass.
- Rebuilding file operations, SFTP, archive, preview, or editor logic from
  scratch.
- Adding visual polish before browser command flow and directory loading are
  proven.
- Introducing floating unreleased GPUI dependencies.

## Product Requirements

1. The app provides a GPUI entry point named `stiff`.
2. The GPUI shell shows a two-panel browser layout with active-panel focus.
3. Browser panels support command-driven navigation, marking, rename, delete
   confirmation, copy/move preparation, paste, reload, help, and panel switching.
4. Keybinding code parses raw key input into domain commands without executing
   browser behavior.
5. Browser command execution mutates browser-owned state or returns explicit
   runtime effects for shell-owned work.
6. Shell code owns GPUI context, global state, async spawns, system clipboard
   writes, external open calls, and scroll/reveal runtime effects.
7. Directory loads run asynchronously and ignore stale load generations.
8. Existing replay tests remain valid for the legacy UI until a UI-neutral or
   GPUI replay path is available.

## Architecture Boundaries

- `src/shell/` owns GPUI application wiring, runtime effects, async tasks, and
  top-level composition.
- `src/features/keybind/` owns key parsing, command sequence state, help/leader
  behavior, and key sequence registration.
- `src/features/file_browser/` owns browser commands, browser state transitions,
  panel state, row data, and browser components.
- `src/features/clipboard/` owns clipboard state, target selection, paste
  planning, and direct clipboard helper logic.
- `src/features/layout/` owns global pane layout state.
- `src/core/` owns framework-neutral filesystem operations and models.

## Migration Tasks

1. Extract framework-neutral browser commands from legacy input and app-state
   flows.
   - Keybindings return `BrowserCommand` values.
   - Browser command execution runs without GPUI context.
   - Runtime-only work is represented as explicit command effects.
   - Command-level tests cover navigation, input-mode transitions, confirmations,
     and load/runtime effects.

2. Add the GPUI entry point with root window, theme provider, and static
   two-panel browser shell.
   - `stiff` starts the GPUI application shell.
   - Shell initializes global layout and clipboard state.
   - Startup renders two browser panels without requiring directory operations.

3. Connect the browser slice to existing directory loading and selection state.
   - Directory reads run on a background executor.
   - Loaded rows update the active panel through browser state helpers.
   - Stale async loads are ignored by generation checks.

4. Port Vim/ranger keymaps through a GPUI key adapter.
   - Raw GPUI key events map into keybind actions.
   - Vim command chains support counts, prefixes, and leader/help metadata.
   - Modal input and confirmations take priority over normal browser commands.

5. Port operations dialogs and quick jump.
   - Copy, move, paste, rename, delete, mkdir, and quick-jump flows use explicit
     command outcomes and shell runtime effects.
   - Long-running operations do not block input rendering.

6. Port preview and editor.
   - Preview subscribes to selection changes instead of running in the selection
     handler.
   - Text highlighting and image decoding stay off the UI input path.
   - Editing preserves existing save and new-file behavior.

7. Replace replay driver with a UI-neutral command harness plus GPUI smoke tests.
   - Command-level tests cover state transitions without a UI framework.
   - GPUI smoke tests verify startup and first panel render.
   - Legacy replay tests continue passing until replacement is complete.

8. Remove the legacy egui runtime after parity and performance checks pass.
   - GPUI supports the required browser, operations, preview, editor, and remote
     workflows.
   - Large-directory navigation meets the performance targets below.

## Performance Targets

- Open a 100k-entry directory without freezing the UI thread.
- Move selection through a 100k-entry virtual list at display refresh rate.
- Keep directory load progress visible while background workers are active.
- Avoid more than one full list re-sort per directory mutation batch.
- Keep preview decoding and syntax highlighting off the input path.

## Verification

- `cargo check` passes.
- Existing legacy replay cases pass after behavior changes.
- Command-level tests are added for each framework-neutral browser command slice.
- GPUI smoke coverage is added before replacing a legacy UI slice.
