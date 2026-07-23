> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#13](https://github.com/Krr0ptioN/fileman/issues/13)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P1**

# F12 — Flattened Recursive View (P1)

**Intent:** Show a recursive file list under the current root for bulk ops.

## User Stories

1. As a stiff user, I want a flattened recursive listing, so that I can operate across a subtree without drilling.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-12.1: Toggle flat mode lists files (and optionally dirs) under cwd recursively.
2. FR-12.2: Depth limit or cancel for huge trees.
3. FR-12.3: Relative path shown in name/detail so collisions are distinguishable.
4. FR-12.4: Marks/ops apply to underlying real paths.
5. FR-12.5: Exiting flat mode restores normal listing.

**Acceptance criteria**

- AC-12.1: Flat view includes nested file; marking + copy copies the real nested path.
- AC-12.2: Cancel during flatten does not leave partial silent state without status.

---

