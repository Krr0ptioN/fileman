> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#7](https://github.com/Krr0ptioN/fileman/issues/7)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P0**

# F06 — SFTP Remote Browsing (P0)

**Intent:** Browse and operate on SFTP paths with the same command model as local FS.

## User Stories

1. As a stiff user, I want SFTP browsing with the same commands as local FS, so that remote work feels local.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-6.1: Connect using ssh config hosts and/or explicit user@host.
2. FR-6.2: Directory listing, open, parent, marks, copy/move/delete, mkdir, rename work remotely via existing SFTP module.
3. FR-6.3: Connection failure surfaces actionable status (auth, host key, network).
4. FR-6.4: Sessions are reusable; disconnect is explicit or on quit.
5. FR-6.5: Mixed local/remote dual-pane transfers are supported where core already allows.
6. FR-6.6: Remote paths display distinctly in chrome (host + path).

**Acceptance criteria**

- AC-6.1: Connect, list, enter dir, copy file to local pane succeeds in integration test environment.
- AC-6.2: Failed auth does not hang the UI thread.

---

