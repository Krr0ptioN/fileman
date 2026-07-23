> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#25](https://github.com/Krr0ptioN/fileman/issues/25)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P0**

# F24 — Paste Conflict Resolution (P0)

**Intent:** When paste/copy/move hits existing names, user chooses policy.

## User Stories

1. As a stiff user, I want paste conflict resolution, so that overwrites are explicit rather than silent.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-24.1: Detect collisions before or during paste plan.
2. FR-24.2: Policies: skip, overwrite, rename (auto suffix), cancel remaining.
3. FR-24.3: Apply-to-all option for batch.
4. FR-24.4: Default policy configurable (F28); safe default is prompt.
5. FR-24.5: Integrates with clipboard paste planner.

**Acceptance criteria**

- AC-24.1: Paste onto existing name with skip leaves destination unchanged and continues batch.
- AC-24.2: Cancel aborts remaining planned ops.

---

