# stiff

stiff is a keyboard-first file manager aimed at fast, feature-rich file
exploration.

## Current Status

The current application direction is a GPUI shell with one browser pane by default,
optional splits, and file previews presented in their own pane.

The application UI is under active development with GPUI, the UI framework
developed by Zed. The single application binary is `stiff`.

### Current Experience

- Single-pane browsing by default, with an optional second browser pane.
- Independent browser-tab stacks in each pane.
- Vim-style navigation and file operations, including numeric movement counts.
- A preview pane opened with `gp`, sharing the window with the active browser.
- Background preview preloading after selection settles, with a loading skeleton
  while work is in progress.
- Bounded text reads and incremental extension when scrolling, so previewing a
  text file does not require loading the complete file.
- Pluggable preview handling for text, archive listings, and binary content.
- Dot-hidden entries and `.gitignore` matches hidden independently by default.
- Parent navigation with `h`, without a synthetic `..` entry in the listing.

## Key Map

### Browsing

| Key | Action |
|-----|--------|
| `j` / `k` | Move down / up |
| `J` / `K` | Move down / up by a page step |
| `gg` / `G` | Jump to top / bottom |
| `0` | Select the first row |
| `h` / `l` | Open parent / open selected entry |
| `s` | Toggle a second browser pane |
| `w`, `Tab`, `Ctrl+I` | Switch browser pane |
| `tn` / `tc` | Open / close a browser tab |
| `tl` / `th` | Switch to next / previous browser tab |
| `r` / `R` | Reload |
| `gh` | Toggle dot-hidden entries |
| `gH` | Toggle gitignored entries |
| `Ctrl+G` | Quick jump |
| `Alt+F7` | Search filenames recursively |
| `Alt+Shift+F7` | Search filenames in the current directory |
| `?` | Open the key map |

Counts apply to movement and jumps, for example `5j` and `12G`.

Filename search is case-insensitive and respects the active pane's hidden and
gitignore visibility settings. Alt+F7 searches the current tree, while
Alt+Shift+F7 limits the scope to the current directory. Press Escape to cancel
input or close results and restore the prior listing; opening a result navigates
to its parent and selects it.

### Files and Selection

| Key | Action |
|-----|--------|
| `v` | Toggle mark on selected entry |
| `V` | Toggle all marks |
| `uv` / `uV` | Clear marks |
| `yy` | Copy selection |
| `yp` / `yn` | Copy path / name |
| `yf` / `yc` | Copy files / file contents |
| `dd` | Mark selection for move |
| `pp` | Paste |
| `gc` | Cancel the active file-operation task |
| `dD` / `x` | Delete |
| `cw` / `C` | Rename |
| `nd` | Create directory |
| `gp` | Toggle preview pane |

### Preview Pane

| Key | Action |
|-----|--------|
| `gp` | Open or close preview for the selected entry |
| `Ctrl+W`, then `j` or `l` | Focus preview pane |
| `Ctrl+W`, then `h` or `k` | Focus browser pane |
| `Ctrl+W`, then `w` | Toggle pane focus |
| `j` / `l`, `k` / `h` | Scroll down / up while preview is focused |
| `Ctrl+D` / `Ctrl+U` | Scroll preview by a half page |

Moving the browser selection dismisses the displayed preview while retaining
cached preview data within the current directory. Changing directory clears that
cached state.

Paste conflicts prompt for skip, overwrite, automatic rename, or cancellation.
Holding Shift while choosing skip, overwrite, or rename applies that policy to
the remaining batch. File operations run through the status-bar task queue,
which reports item/byte progress and retains completion or failure summaries.

## Architecture

The GPUI code separates browsing state, input interpretation, presentation, and
background work:

| Path | Responsibility |
|------|----------------|
| `src/main.rs` | GPUI application entry point |
| `src/shell/` | App state, rendering, pane focus, and asynchronous operation execution |
| `src/features/file_browser/` | Directory state, command effects, components, visibility policy, and preview handlers |
| `src/features/keybind/` | Vim-style command registry and dispatch |
| `src/features/clipboard/` | Clipboard state and copy/move behavior |
| `src/features/layout/` | Single and dual browser pane layout policy |
| `src/core/` | Filesystem models and shared low-level helpers |


Directory reads and preview preparation run away from rendering. Preview content
is modeled independently of its renderer, allowing format-specific handlers and
future loading policies without coupling them to pane UI code.

## Direction

stiff is moving toward a general pane and tab workspace, rather than treating
two browser panels as the permanent default. The current preview system provides
the first split-pane workflow and the foundation for:

- additional file-format preview handlers;
- navigable archive previews;
- viewport-aware caching and preloading policies;
- explicit memory offloading strategies for large content; and
- tabs and more flexible pane arrangements.

## Build and Verify

Run the GPUI application:

```bash
cargo run --bin stiff -- /path/to/dir
```

Run the checks used for the current GPUI work:

```bash
cargo fmt --check
cargo test --lib
cargo clippy --lib --bin stiff -- -D warnings
cargo check --bin stiff
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for repository layout, testing, and code
style.
