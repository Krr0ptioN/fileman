# GPUI Performance Plan

This plan defines the first migration slice for making FileMan feel closer to
Zed: immediate keyboard response, virtualized file panels, and no UI-thread IO.

## First Slice: Browser Shell

Scope:

- New `fileman-gpui` binary or feature-gated entry point.
- GPUI root window with `gpui-component::Root`.
- Vercel-inspired theme tokens.
- Two-panel browser layout with header, virtualized rows, and command bar.
- Existing directory loader reused through framework-neutral commands.
- Existing Vim/ranger command model reused through a GPUI key adapter.

Not in first slice:

- Preview rendering.
- Editor.
- SFTP dialogs.
- Archive listing UI beyond existing loaded entries.
- Snapshot parity.

## Vertical Slice Shape

Each feature owns its UI, command adapter, and tests:

```text
features/browser/
  mod.rs
  model.rs        # browser-specific view model, derived/cached rows
  commands.rs     # open, parent, select, mark, sort, tab operations
  gpui_view.rs    # GPUI components for panel/header/rows
  keymap.rs       # GPUI key events to browser commands
  tests.rs
```

Shared dependencies flow inward:

```text
gpui_view -> keymap -> commands -> app_state/core
```

No feature should depend on another feature's GPUI view. Cross-feature work goes
through app-level commands, for example `OpenPreview`, `PrepareCopy`, or
`OpenQuickJump`.

## Browser Performance Rules

- Keep row view data small: index, name, kind, size label, mark state, selection
  state.
- Store expensive derived data in a cache keyed by directory token, sort mode,
  filter, and marked set version.
- Use virtualized list/table components for all directory rows.
- Selection changes update only active panel state and notify the active panel.
- Directory loads stream batches into model state; batch notifications should be
  coalesced to avoid repaint storms.
- Preview updates must subscribe to selection changes, not run in the selection
  handler itself.

## Theme Tokens

```text
bg_canvas        #0a0a0a
bg_panel         #111111
bg_panel_raised  #171717
border_subtle    #262626
border_focus     #3b82f6
text_primary     #fafafa
text_secondary   #a1a1aa
text_muted       #71717a
row_hover        #1f1f1f
row_selected     #0f2a4a
accent           #3b82f6
danger           #ef4444
warning          #f59e0b
success          #22c55e
```

Use the tokens through the GPUI theme layer instead of scattering literal colors
through feature views.

## Benchmark Targets

These are developer-facing targets, not hard test gates yet:

- Open a 100k-entry directory without freezing the UI thread.
- Move selection through a 100k-entry virtual list at display refresh rate.
- Keep directory load progress visible while SFTP/archive workers are active.
- Avoid more than one full list re-sort per directory mutation batch.
- Keep preview decoding/highlighting off the input path.

## Verification

For each migrated slice:

- Existing replay cases still pass on legacy UI.
- Add command-level tests that do not depend on egui or GPUI.
- Add one GPUI smoke test for startup and first panel render.
- Profile large directory navigation before replacing the legacy slice.
