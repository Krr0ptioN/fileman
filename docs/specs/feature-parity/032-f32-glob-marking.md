> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#33](https://github.com/Krr0ptioN/fileman/issues/33)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P1**

# F32 — Glob Marking (P1)

**Intent:** Mark entries by glob/pattern in the current listing (or flat view).

## User Stories

1. As a stiff user, I want glob-based marking, so that I can select sets by pattern.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-32.1: Prompt for glob; mark all matches in current row model.
2. FR-32.2: Unmark-by-glob supported.
3. FR-32.3: Invalid glob reports status.
4. FR-32.4: Respects hidden/ignore visibility of current model.

**Acceptance criteria**

- AC-32.1: `*.txt` marks all visible text files.
- AC-32.2: No matches → status, marks unchanged.

---

