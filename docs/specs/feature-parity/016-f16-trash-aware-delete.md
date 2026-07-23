> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#17](https://github.com/Krr0ptioN/fileman/issues/17)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P0**

# F16 — Trash-Aware Delete (P0)

**Intent:** Default delete moves to platform trash; permanent delete is explicit.

## User Stories

1. As a stiff user, I want trash-aware delete, so that accidental deletes are recoverable by default.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-16.1: Default delete confirmation states trash vs permanent based on config.
2. FR-16.2: Permanent delete remains available (current `dD`/`x` semantics may map to permanent or become two commands).
3. FR-16.3: Trash backend uses Freedesktop trash on Linux when available; clear fallback messaging otherwise.
4. FR-16.4: Marked multi-delete supported.
5. FR-16.5: Failures per item reported; partial success listed.

**Acceptance criteria**

- AC-16.1: Trash delete removes from listing and appears in trash (or documented fallback).
- AC-16.2: Permanent delete removes file without trash entry.

---

