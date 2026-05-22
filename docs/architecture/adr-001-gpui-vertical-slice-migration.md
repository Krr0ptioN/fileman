# ADR-001: GPUI Vertical Slice Migration

## Status
Accepted

## Context
FileMan is currently a Rust desktop file manager built on a custom `winit` +
`blade-egui` runtime. The codebase already has strong async filesystem, SFTP,
archive, preview, edit, replay, and image decode logic, but UI rendering,
keyboard handling, replay, and view state are tightly coupled to egui types.

The target is a faster, Zed-like desktop UI using GPUI and `gpui-component`,
with a premium Vercel-inspired theme and a modular, feature-driven codebase.
The migration must not regress existing file operations or async behavior.

## Decision
Adopt GPUI incrementally behind a new shell instead of replacing the current UI
in one pass.

Use a modular monolith with vertical feature slices:

```text
src/
  core/                    # framework-neutral domain and IO contracts
  app/                     # app composition, runtime commands, routing
  features/
    browser/               # panels, tabs, selection, marked entries
    command_palette/       # quick jump, search launch, command lookup
    preview/               # text/image/archive preview
    editor/                # file editor integration
    operations/            # copy/move/delete/rename/mkdir/pack confirmations
    remote/                # SFTP connection and remote navigation UI
    theme/                 # Vercel-style tokens and theme switching
  ui/
    gpui/                  # GPUI shell, shared components, keymap adapter
    legacy_egui/           # existing UI during migration
```

The first GPUI implementation should be a separate binary or feature-gated
entry point, for example `fileman-gpui`, until it can pass parity replay cases.

Use the upstream dependency names and keep versions pinned. The initial
buildable slice uses crates.io releases because they give reproducible builds
without floating `main` dependencies:

```toml
gpui = { version = "0.2.2", optional = true }
gpui-component = { version = "0.5.1", optional = true }
```

If a future slice needs unreleased GPUI APIs, pin exact git revisions before
landing the change. Do not add floating `main` dependencies to `Cargo.toml`;
that would make builds non-reproducible and hide API breakages.

## Rationale
GPUI gives the app a retained, GPU-accelerated component model that better fits
large virtualized lists, command-driven interactions, and editor-like UI
performance. `gpui-component` adds practical desktop components, including
dock/panel layout, virtualized list/table primitives, theming, and editor
building blocks, which avoids rebuilding the entire UI kit from scratch.

A vertical-slice migration keeps performance work honest:

- The browser slice can prove panel virtualization and keyboard latency first.
- The preview slice can prove text/image rendering without blocking directory
  navigation.
- The operations slice can reuse existing IO tasks before visual polish.
- The legacy egui UI remains shippable while GPUI parity is incomplete.

## Trade-offs
- Running both UI stacks temporarily increases code size and maintenance.
- Replay/snapshot tests need a GPUI adapter instead of direct egui event
  injection.
- Some existing egui-specific code must be split into framework-neutral state
  transitions before it can be reused.

## Performance Constraints
The GPUI shell should be built around these budgets:

- Directory listings: virtualized rows only; never allocate or paint every row
  for large folders.
- Input latency: key press to selection update should stay under one frame.
- IO: no filesystem, archive, preview, image decode, or SFTP work on the UI
  thread.
- Rendering: only notify/repaint affected views after state changes.
- Sorting/filtering: cache derived lists and recompute only when source entries
  or sort/search options change.
- Preview: stream or cap expensive content; do syntax highlighting off the UI
  path.

## Theme Direction
Use a Vercel-inspired premium theme without hard-coding brand assets:

- Neutral background stack: near-black, elevated charcoal, hairline borders.
- High contrast text with restrained secondary text.
- Single accent for focus/selection, defaulting to blue.
- Dense layout, 4-8px radii, precise separators, no decorative gradients.
- Command bar and panels should feel like a developer tool, not a landing page.

## Migration Order
1. Extract framework-neutral browser commands from `src/input.rs` and
   `src/app_state.rs`.
2. Add `fileman-gpui` entry point with a GPUI root, theme provider, and static
   two-panel browser shell.
3. Connect the browser slice to existing directory loading and selection state.
4. Port Vim/ranger keymaps through a GPUI keymap adapter.
5. Port operations dialogs and quick jump.
6. Port preview and editor.
7. Replace replay driver with a UI-neutral command harness plus GPUI smoke
   tests.
8. Remove legacy egui runtime after parity and performance checks pass.

## Revisit Trigger
Reconsider this decision if GPUI or `gpui-component` blocks required platforms,
cannot support the replay/snapshot strategy, or the browser slice cannot match
current behavior without significant framework-specific complexity.
