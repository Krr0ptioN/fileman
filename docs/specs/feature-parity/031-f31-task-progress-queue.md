> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#32](https://github.com/Krr0ptioN/fileman/issues/32)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P0**

# F31 — Task / Progress Queue (P0)

**Intent:** Make long operations visible, cancellable, and non-blocking.

## User Stories

1. As a stiff user, I want a visible task/progress queue, so that long copies do not freeze or hide status.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-31.1: Mutating multi-file ops enqueue as tasks with progress (bytes/items).
2. FR-31.2: Status bar or panel lists running/queued/completed/failed.
3. FR-31.3: Cancel task best-effort; partial results reported.
4. FR-31.4: UI navigation remains responsive while tasks run.
5. FR-31.5: Errors aggregate without modal spam (summary + expandable detail P1).

**Acceptance criteria**

- AC-31.1: Large copy shows progress and completes with summary.
- AC-31.2: Cancel mid-copy stops further items and reports partial state.

---

