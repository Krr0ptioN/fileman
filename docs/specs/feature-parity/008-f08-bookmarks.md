> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#9](https://github.com/Krr0ptioN/fileman/issues/9)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P0**

# F08 — Bookmarks (P0)

**Intent:** Persist letter (or named) bookmarks for instant jumps.

## User Stories

1. As a stiff user, I want letter bookmarks, so that I can jump to frequent paths instantly.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

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

