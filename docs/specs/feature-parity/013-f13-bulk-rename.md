> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#14](https://github.com/Krr0ptioN/fileman/issues/14)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P1**

# F13 — Bulk Rename (P1)

**Intent:** Rename many marked entries via a structured multi-rename UI or pattern.

## User Stories

1. As a stiff user, I want bulk rename, so that I can rename many marked files in one pass.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-13.1: Requires one or more marked (or selected) entries.
2. FR-13.2: Presents editable target names (buffer/list) or pattern (find/replace, numbering).
3. FR-13.3: Validates collisions and illegal names before applying.
4. FR-13.4: Applies renames sequentially/transactionally with per-item status; stops or continues on error per policy.
5. FR-13.5: Dry-run preview of resulting names.
6. FR-13.6: Undo integration when F26 exists.

**Acceptance criteria**

- AC-13.1: Two marked files renamed to unique names both succeed.
- AC-13.2: Conflicting targets aborted before any rename (or rolled back if partial policy chosen — document choice in implementation).

---

