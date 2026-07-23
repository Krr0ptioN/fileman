> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#35](https://github.com/Krr0ptioN/fileman/issues/35)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P1**

# F34 — Content Search (P1)

**Intent:** Find files by contents (ripgrep-style) from cwd.

## User Stories

1. As a stiff user, I want content search (ripgrep-style), so that I can find files by contents, not only names.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-34.1: Query prompt; recursive search with ignore rules aligned to visibility settings.
2. FR-34.2: Results list path + optional line preview snippet.
3. FR-34.3: Open result jumps to file; editor may land on line when F05 exists.
4. FR-34.4: Cancel stops worker; ignore stale results.
5. FR-34.5: Binary files skipped by default.

**Acceptance criteria**

- AC-34.1: Unique string in fixture tree returns that file.
- AC-34.2: Escape/cancel restores prior UI context.

---

