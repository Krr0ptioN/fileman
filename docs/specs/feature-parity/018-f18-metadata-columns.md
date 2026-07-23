> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#19](https://github.com/Krr0ptioN/fileman/issues/19)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P1**

# F18 — Metadata Columns (P1)

**Intent:** Show size, mtime, permissions, owner/group in the row or detail region.

## User Stories

1. As a stiff user, I want metadata columns (size, mtime, mode, owner), so that I can inspect entries without `stat`.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

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

