> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#31](https://github.com/Krr0ptioN/fileman/issues/31)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P1**

# F30 — Disk Usage Insight (P1)

**Intent:** Show space used by entries to find large subtrees.

## User Stories

1. As a stiff user, I want disk-usage insight, so that I can find what consumes space.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-30.1: Compute du-like sizes for directories async with cancel.
2. FR-30.2: Display in detail/column when ready; show progress placeholder meanwhile.
3. FR-30.3: Sort by computed size when available (integrates F10).
4. FR-30.4: Free space for volume containing cwd shown in status/chrome.

**Acceptance criteria**

- AC-30.1: Directory size eventually populates and matches rough `du` order.
- AC-30.2: Cancel/navigate away drops stale du updates.

---

