> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#23](https://github.com/Krr0ptioN/fileman/issues/23)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P2**

# F22 — Miller Columns (P2)

**Intent:** Optional multi-column cascade (parent | current | child preview).

## User Stories

1. As a stiff user, I want optional Miller columns, so that parent/current/child context is visible at once.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

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

