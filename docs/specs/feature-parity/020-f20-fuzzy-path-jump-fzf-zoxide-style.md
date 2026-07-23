> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#21](https://github.com/Krr0ptioN/fileman/issues/21)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P1**

# F20 — Fuzzy Path Jump (fzf / zoxide style) (P1)

**Intent:** Jump to deep paths via fuzzy finder and/or frecency.

## User Stories

1. As a stiff user, I want fuzzy path jump (fzf/zoxide-style), so that deep paths are reachable without typing full paths.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-20.1: Fuzzy finder over files/dirs under a root or from a path index.
2. FR-20.2: Optional zoxide/frecency backend for directory jump.
3. FR-20.3: Extends or complements existing Quick Jump (`Ctrl+G` path entry) — Quick Jump remains for exact/relative path typing.
4. FR-20.4: Selection loads directory or selects file.
5. FR-20.5: External tool integration allowed (spawn fzf) **or** in-app fuzzy list — pick one primary in implementation decision; document it.

**Acceptance criteria**

- AC-20.1: Fuzzy match unique deep path jumps correctly.
- AC-20.2: Cancel leaves cwd unchanged.

---

