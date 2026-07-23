> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#2](https://github.com/Krr0ptioN/fileman/issues/2)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P0**

# F01 — Filename Search (P0)

**Intent:** Find and navigate to entries by name within a configurable scope
(current directory or recursive under current root).

## User Stories

1. As a stiff user, I want filename search in the current tree, so that I can find files without leaving the browser.
2. As a stiff user, I want to cancel or escape search and restore the prior listing, so that search never traps me.
3. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
4. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

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

**Out of this feature:** content/ripgrep search (see F34).

---

