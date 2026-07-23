> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#28](https://github.com/Krr0ptioN/fileman/issues/28)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P2**

# F27 — Additional Remotes (SMB / WebDAV / …) (P2)

**Intent:** Browse non-SFTP network shares with the same browser UX.

## User Stories

1. As a stiff user, I want SMB/WebDAV (and similar) remotes, so that non-SFTP shares work in the same browser.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-27.1: Pluggable remote backend interface shared with SFTP patterns (list, read, write, mkdir, rename, delete).
2. FR-27.2: SMB and/or WebDAV as first additional backends.
3. FR-27.3: Auth prompts and credential caching policy documented.
4. FR-27.4: URI scheme in chrome and bookmarks.
5. FR-27.5: Feature-detect unavailable backends cleanly on unsupported platforms.

**Acceptance criteria**

- AC-27.1: Against a test server/fixture, list and copy one file to local.
- AC-27.2: Unsupported build shows clear “backend unavailable” status.

---

