> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#34](https://github.com/Krr0ptioN/fileman/issues/34)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P2**

# F33 — Mount / Volume Browser (P2)

**Intent:** Discover and open mounts, removable media, and common roots.

## User Stories

1. As a stiff user, I want mount/volume browsing, so that removable and network mounts are discoverable.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-33.1: List mounts/volumes appropriate to OS.
2. FR-33.2: Jump opens that path in active tab.
3. FR-33.3: Refresh detects new mounts.
4. FR-33.4: Unreadable mounts show error status.

**Acceptance criteria**

- AC-33.1: Selecting a listed mount changes cwd to its path.
- AC-33.2: Empty mount list is non-fatal.

---

