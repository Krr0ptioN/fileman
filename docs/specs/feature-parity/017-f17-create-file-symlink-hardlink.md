> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#18](https://github.com/Krr0ptioN/fileman/issues/18)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P0**

# F17 — Create File, Symlink, Hardlink (P0)

**Intent:** Create non-directory nodes and links.

## User Stories

1. As a stiff user, I want create file, symlink, and hardlink actions, so that filesystem graph edits are first-class.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-17.1: New empty file with name prompt (alongside existing mkdir).
2. FR-17.2: Create symlink: target = selection (or prompted), link name prompted.
3. FR-17.3: Create hardlink where filesystem allows; error if cross-device.
4. FR-17.4: Collision handling consistent with rename/mkdir.
5. FR-17.5: After create, listing reloads and selects new entry.

**Acceptance criteria**

- AC-17.1: New file appears and is empty.
- AC-17.2: Symlink resolves to intended target.
- AC-17.3: Hardlink failure on cross-device is explicit.

---

