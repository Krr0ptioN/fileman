> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#6](https://github.com/Krr0ptioN/fileman/issues/6)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P0**

# F05 — In-App Editor (P0)

**Intent:** Edit text files without leaving stiff for small changes.

## User Stories

1. As a stiff user, I want to edit text files in-app, so that small fixes do not require an external editor round-trip.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-5.1: Open selected text file in editor mode/pane.
2. FR-5.2: Insert/selection editing with save and discard/cancel.
3. FR-5.3: Dirty buffer warning on close/navigate away.
4. FR-5.4: Binary/unsupported types refuse edit with status, offer open-with instead.
5. FR-5.5: New-file flow can create then edit (coordinates with F20).
6. FR-5.6: Saves write atomically when feasible (temp + rename).

**Acceptance criteria**

- AC-5.1: Edit, save, reopen shows new contents.
- AC-5.2: Cancel discard leaves file unchanged.

---

