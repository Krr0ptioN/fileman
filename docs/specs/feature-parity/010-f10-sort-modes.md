> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#11](https://github.com/Krr0ptioN/fileman/issues/11)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P0**

# F10 — Sort Modes (P0)

**Intent:** User-controlled ordering of directory rows.

## User Stories

1. As a stiff user, I want configurable sort modes, so that I can order by name, size, mtime, or type.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

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

