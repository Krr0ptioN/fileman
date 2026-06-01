# stiff Application Architecture

This document describes the current GPUI application architecture. It focuses
on ownership boundaries, command flow, file operations, and how a user action
moves from keyboard input to rendered UI.

## Architectural Goals

stiff is being rebuilt as a modular GPUI file manager. The architecture is a
feature-driven modular monolith:

- `src/shell/` owns GPUI application wiring, runtime effects, async tasks, and
  top-level composition.
- `src/features/keybind/` owns keyboard parsing and key sequence mapping.
- `src/features/file_browser/` owns file-browser state transitions, panels,
  rows, browser commands, and browser components.
- `src/features/clipboard/` owns clipboard state, copy/move selection state,
  paste planning, and direct clipboard writes.
- `src/features/layout/` owns global layout mode used by multiple views.
- `src/core/` owns framework-neutral filesystem operations.

The intended dependency direction is:

```text
shell
  -> features/keybind
  -> features/file_browser
  -> features/clipboard
  -> features/layout
  -> core
```

`keybind` must not execute file-browser behavior. It should only parse input
and return a domain command. `file_browser` owns execution of browser commands.
`shell` applies runtime effects that require GPUI context, globals, async
spawns, or OS clipboard access.

## Top-Level Shell

The GPUI shell is split across `src/shell/`:

```text
src/shell/
  mod.rs          # module root and public run export
  app.rs          # GPUI Application startup and window creation
  state.rs        # StiffShell fields and panel accessors
  input.rs        # raw GPUI key event consumption
  input_modes.rs  # rename, confirm, control, and held navigation handlers
  key_handler.rs  # AppKeyHandler implementation for keybind dispatch
  commands.rs     # applies file_browser command outcomes to runtime effects
  operations.rs   # async directory loads and file operations
  render.rs       # GPUI Render implementation
```

`StiffShell` is the root GPUI entity. It stores app-level UI state:

- primary and secondary `BrowserPanel`
- active panel side
- Vim command state
- input mode, such as rename
- pending confirmations, such as delete
- held navigation acceleration state
- keybind registry
- help and leader popup state
- operation-in-flight flag
- status text

The shell does not own file-browser command logic. It owns the state required to
connect domain logic to GPUI runtime behavior.

## Feature Boundaries

### `features/keybind`

The keybind feature owns input interpretation:

- raw GPUI key events to command characters
- help key behavior
- leader-map open/close behavior
- held navigation acceleration keys
- Vim/ranger chain state
- key sequence registry
- mapping key sequences to `BrowserCommand`

Important files:

```text
features/keybind/
  dispatch.rs         # ordering of modal, control, cancel, help, leader, nav, Vim handling
  vim_state.rs        # stateful Vim/ranger command-chain parser
  gpui_keys.rs        # GPUI KeyDownEvent to command char
  modes.rs            # rename and confirm key actions
  control.rs          # control shortcuts such as pane switching
  navigation.rs       # held up/down/j/k acceleration
  registry/           # keybind registry and help/leader metadata
  browser/            # file-manager keymap registration
```

`keybind` returns `BrowserCommand` values. It does not mutate panels, run file
operations, write to clipboard, or load directories.

### `features/file_browser`

The file-browser feature owns browser-specific state transitions and rendering
components.

Important command files:

```text
features/file_browser/
  command.rs           # BrowserCommand enum
  command_executor.rs  # BrowserCommand -> BrowserCommandOutcome
  command_effect.rs    # runtime effects requested from shell
  command_state.rs     # mutable view of browser state used during command execution
  actions.rs           # selection, mark, delete, rename helpers
  navigation.rs        # parent/open-selected navigation decisions
  state.rs             # BrowserPanel, FileTarget, InputMode, PendingConfirm
```

Important component files:

```text
features/file_browser/components/
  chrome.rs            # title and command bars
  header.rs            # panel header
  panel.rs             # virtualized panel list
  row.rs               # row shell
  row_content.rs       # row content and metadata display
  leader_map.rs        # leader popup
  help_popup.rs        # keybinding help popup
  variants/            # layout strategies
```

`file_browser` may mutate browser panels, input mode, pending confirmations, and
marks. It does not directly use GPUI runtime context, spawn async tasks, or write
to the OS clipboard.

### `features/clipboard`

The clipboard feature owns clipboard-specific state and planning:

- `ClipboardState` as a GPUI global
- copy/move target selection
- clipboard target highlighting
- paste planning
- copy path/name/file contents helpers

The shell calls clipboard helpers when a browser command outcome requests
clipboard work.

### `features/layout`

The layout feature owns global pane layout state. `LayoutState` is registered as
a GPUI global by the shell. Components can read the global to adapt to single or
dual pane mode.

### `core`

`core` contains framework-neutral filesystem operations. Directory loading and
file operations call into `core` from shell async tasks or file operation
objects.

## Command Lifecycle

This is the full path from pressing a key to seeing the result.

### 1. GPUI Delivers a Key Event

`render.rs` attaches the key listener to the root element:

```rust
.on_key_down(cx.listener(Self::on_key_down))
```

The listener enters `shell/input.rs`:

```text
StiffShell::on_key_down
  -> dispatch_key_command
  -> keybind::handle_key_command
```

If a key is handled, shell consumes the event:

```text
window.prevent_default()
cx.stop_propagation()
cx.notify()
```

That prevents duplicate platform handling and schedules the GPUI entity to
render updated state.

### 2. Keybind Dispatch Chooses the Correct Mode

`features/keybind/dispatch.rs` defines the input priority:

```text
modal input
  -> control keys
  -> cancel key
  -> help key
  -> leader key
  -> held navigation
  -> Vim/ranger key sequence
```

Modal input means current focused app mode wins. For example, rename mode treats
letters as text input instead of browser commands.

### 3. Modal, Control, and Navigation Keys

Some key paths are handled before Vim command chains:

- Rename mode is interpreted by `keybind::rename_key_action` and applied in
  `shell/input_modes.rs`.
- Confirm mode is interpreted by `keybind::confirm_key_action` and applied in
  `shell/input_modes.rs`.
- Pane switching is interpreted by `keybind::control_action`.
- Held navigation is interpreted by `keybind::navigation_input` and accelerated
  by `HeldNavigation`.

Held navigation directly updates the active `BrowserPanel` selection because it
is an immediate repeated UI interaction:

```text
held j/k/up/down
  -> HeldNavigation::rows_for
  -> active_panel_mut().select_relative(...)
  -> panel.reveal_selected()
```

### 4. Vim/Ranger Command Chains

For normal-mode command chains, `keybind` uses `VimCommandState`.

Example: user presses `y`, then `p`.

```text
first key: y
  -> VimCommandState sees a valid prefix
  -> BrowserVimOutcome::Pending("y")
  -> shell.status = "y"

second key: p
  -> VimCommandState completes sequence "yp"
  -> KeybindRegistry maps "yp" to BrowserCommand::CopyPath
  -> BrowserVimOutcome::Command { command, sequence }
```

The important rule is that `keybind` stops here. It does not copy paths. It only
produces the command.

### 5. File Browser Executes the Command

The shell passes the command to `features/file_browser`:

```text
StiffShell::execute_browser_command
  -> file_browser::execute_browser_command
```

The file-browser command executor receives a `BrowserCommandState`, which is a
temporary mutable view over:

- primary panel
- secondary panel
- active side
- input mode
- pending confirmations

The executor mutates browser-owned state directly when the operation is purely
browser state, for example:

```text
BrowserCommand::Move
  -> active_panel_mut().select_relative(...)

BrowserCommand::ToggleMark
  -> file_browser::toggle_marked(...)

BrowserCommand::Rename
  -> file_browser::start_rename(...)
```

For work that requires shell/runtime services, the executor returns a
`BrowserCommandOutcome` with a `BrowserCommandEffect`.

Examples:

```text
BrowserCommand::Copy
  -> BrowserCommandEffect::PrepareClipboard { kind, targets }

BrowserCommand::Paste
  -> BrowserCommandEffect::PasteInto(dst_dir)

BrowserCommand::OpenSelected on a directory
  -> BrowserCommandEffect::LoadActive { path, prefer_name }

BrowserCommand::TogglePaneMode
  -> BrowserCommandEffect::TogglePaneMode
```

This keeps file-browser command logic in `file_browser` without giving it access
to GPUI globals, async spawning, or OS clipboard APIs.

### 6. Shell Applies Runtime Effects

`shell/commands.rs` applies `BrowserCommandOutcome`.

The shell first applies status text, if present. Then it matches the effect:

```text
None
  -> no runtime work

LoadActive
  -> load_panel(...)

PrepareClipboard
  -> ClipboardState::update_global(...)

CopyPath / CopyName / CopyFileContents
  -> clipboard helper writes to OS clipboard

PasteInto
  -> plan_paste(...)
  -> run_operation(FileOperation::Paste)

TogglePaneMode
  -> LayoutState::update_global(...)

OpenHelp
  -> sets help popup state

ReloadActive
  -> load_panel(active path)
```

This is where runtime integration belongs because it requires `Context<Self>`,
GPUI globals, async tasks, or system clipboard access.

### 7. Async Directory Loading

Directory loading is shell/runtime work:

```text
load_panel(side, path, prefer_name)
  -> mark panel loading
  -> increment load_generation
  -> spawn background task
  -> core::read_fs_directory(path)
  -> update shell entity on UI thread
  -> BrowserCommandState::apply_loaded(...)
  -> cx.notify()
```

`load_generation` protects against stale async results. If an older directory
load finishes after a newer one, `apply_loaded` ignores it.

Loaded entries are converted into `FileRow` values. The selected row is restored
from `prefer_name` when possible, then the panel reveals the selected row.

### 8. Async File Operations

File operations use `FileOperation` in `features/file_browser/ops.rs`.

Examples:

- paste copy
- paste move
- delete
- rename

The shell runs operations through `run_operation`:

```text
run_operation(operation)
  -> reject if operation_in_flight
  -> set operation_in_flight
  -> set pending status
  -> spawn operation.run() on background executor
  -> update shell on UI thread
  -> clear active marks on success
  -> reload both panels
```

The operation implementation itself remains outside GPUI runtime code.

### 9. Rendering the Updated State

After handled input or async updates, shell calls `cx.notify()`. GPUI then calls
`Render` for `StiffShell` in `shell/render.rs`.

Rendering composes the top-level UI:

```text
TitleBar
PanelLayout
CommandBar
LeaderMap, conditionally
HelpPopup, conditionally
```

`PanelLayout` is a strategy component. It decides whether panels render as:

- single active panel
- dual split
- dual stacked

It reads `LayoutState` from GPUI global state and also considers viewport shape.

Each `FilePanel` renders:

```text
PanelHeader
virtualized row list
```

Rows are virtualized with GPUI's `uniform_list`. Only visible rows are built and
painted. Each row receives derived intent:

- selected
- active
- marked
- copy target
- move target
- pending delete target

The row components then render icons, metadata, and visual treatment such as
selected or active styling.

## Example: `yp` Copy Path

```text
user presses y
  -> shell/input.rs on_key_down
  -> keybind dispatch
  -> VimCommandState pending "y"
  -> shell.status = "y"
  -> cx.notify()
  -> CommandBar displays "y"

user presses p
  -> shell/input.rs on_key_down
  -> keybind dispatch
  -> VimCommandState completes "yp"
  -> KeybindRegistry returns BrowserCommand::CopyPath
  -> shell/commands.rs sends command to file_browser executor
  -> file_browser returns BrowserCommandEffect::CopyPath(selected_target)
  -> shell applies effect with copy_target_path(...)
  -> clipboard helper writes selected path to OS clipboard
  -> shell.status = "copied path <name>"
  -> cx.notify()
  -> CommandBar displays copied-path status
```

## Example: `l` Open Selected Directory

```text
user presses l
  -> keybind maps "l" to BrowserCommand::OpenSelected
  -> file_browser::selected_navigation(active_panel)
  -> if selected row is a directory:
       BrowserCommandEffect::LoadActive { path, prefer_name }
  -> shell.load_panel(active, path, prefer_name)
  -> shell marks panel loading and spawns background directory read
  -> core::read_fs_directory runs off the UI thread
  -> loaded rows return to shell entity
  -> BrowserCommandState::apply_loaded updates panel rows
  -> cx.notify()
  -> PanelLayout/FilePanel render the new directory rows
```

## Example: `dD` Delete

```text
user presses d
  -> VimCommandState pending "d"
  -> leader map can display d-prefixed commands

user presses D
  -> keybind maps "dD" to BrowserCommand::Delete
  -> file_browser computes effective targets
  -> file_browser::prepare_delete updates pending_confirm
  -> status becomes "delete N item(s)? y/enter to confirm"
  -> render shows pending delete state on rows

user presses y
  -> confirm mode takes priority before Vim commands
  -> confirm_key_action returns Confirm
  -> shell.run_operation(FileOperation::Delete)
  -> operation runs in background
  -> shell clears marks and reloads panels on success
```

## Help and Leader Display

The keybind registry is also the source of help and leader metadata.

- `HelpPopup` uses `keybinds.help_groups()`.
- `LeaderMap` uses `keybinds.leader_continuations(prefix)`.
- General leader entries are shown when leader mode opens with no prefix.
- Prefix-specific entries are shown while a Vim sequence is pending.

This keeps the visible key documentation derived from the actual registered
keybinds.

## Design Rules

Current architectural rules:

- Keybind maps input to commands; it does not execute domain behavior.
- File-browser executes browser commands and returns runtime effects.
- Shell applies runtime effects that require GPUI context, globals, async work,
  or system clipboard access.
- Components are structs implementing GPUI rendering traits, not free
  `render_*` functions.
- Component modules should stay small and focused.
- `mod.rs` files should expose modules and re-exports only.
- Filesystem and operation work must stay off the UI thread.
- Render code should consume prepared state and avoid hidden side effects.

## Current Known Debt

The architecture is cleaner than the previous shell-centered version, but there
are still areas to keep improving:

- Rename and confirm mode application still lives in shell because it directly
  starts runtime operations.
- Clipboard effects are applied in shell because GPUI context is needed for
  globals and OS clipboard writes.
- `BrowserCommandState` is intentionally a temporary adapter. If file-browser
  state grows, it should become a real owned feature state instead of a borrowed
  view over shell fields.
- `README.md` still describes older egui-era features and should be refreshed
  once the GPUI file manager becomes the only supported UI.
