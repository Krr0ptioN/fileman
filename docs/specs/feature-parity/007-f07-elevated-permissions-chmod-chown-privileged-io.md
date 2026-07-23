> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#8](https://github.com/Krr0ptioN/fileman/issues/8)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P1**

# F07 — Elevated Permissions (chmod / chown / privileged IO) (P1)

**Intent:** Recover from permission errors and change mode/owner when elevation is available.

## User Stories

1. As a stiff user, I want elevated chmod/chown when permission is denied, so that admin tasks do not force an external terminal.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-7.1: chmod command prompts for mode (octal or symbolic) for selection/marks.
2. FR-7.2: chown command prompts for owner/group when platform supports it.
3. FR-7.3: When an operation fails with EACCES/EPERM and elevation is available, offer elevated retry.
4. FR-7.4: Elevation uses existing elevate helpers; unavailable elevation reports how to enable.
5. FR-7.5: Recursive flag is explicit for directories.

**Acceptance criteria**

- AC-7.1: chmod on owned file updates mode visible in metadata (F21) or `stat`.
- AC-7.2: Denied op without elevation shows clear status, no silent no-op.

---

