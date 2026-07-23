> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#24](https://github.com/Krr0ptioN/fileman/issues/24)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P1**

# F23 — Undo File Operations (P1)

**Intent:** Reverse recent mutating ops where safely possible.

## User Stories

1. As a stiff user, I want undo for recent file operations, so that mistakes are cheap to reverse.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-23.1: Stack of undo records for rename, move, copy (optional), trash-delete, mkdir, create file.
2. FR-23.2: Permanent delete and opaque external shell effects are non-undoable (reported).
3. FR-23.3: Undo command pops and applies inverse; failures report and keep stack consistent.
4. FR-23.4: Stack size bounded and cleared on quit or optionally persisted (default: session-only).
5. FR-23.5: Status announces what was undone.

**Acceptance criteria**

- AC-23.1: Rename then undo restores original name.
- AC-23.2: Trash-delete then undo restores path when trash supports it.

---

