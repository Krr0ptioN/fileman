> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#5](https://github.com/Krr0ptioN/fileman/issues/5)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P0**

# F04 — Archive Browse, Extract, Compress (P0)

**Intent:** Treat supported archives as navigable containers; pack/unpack explicitly.

## User Stories

1. As a stiff user, I want to enter archives as directories, so that I can browse zip/tar contents without extracting first.
2. As a stiff user, I want to extract and create archives, so that packing workflows stay inside stiff.
3. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
4. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-4.1: Opening a supported archive enters container browse mode with listing from existing archive core.
2. FR-4.2: Parent/back exits nested archive paths correctly (including nested archives if supported).
3. FR-4.3: Preview of archive lists entries without full extract.
4. FR-4.4: Extract command writes selected/marked entries to a chosen destination (default: cwd or opposite pane).
5. FR-4.5: Create-archive command packs selection into a chosen format (zip/tar/…).
6. FR-4.6: Copy/move/delete inside archives follow existing container IO helpers; unsupported ops report clear status.
7. FR-4.7: Progress reported for large pack/unpack.

**Acceptance criteria**

- AC-4.1: Navigate into archive, into subdir, back out to FS without crash.
- AC-4.2: Extract produces expected files; cancel mid-op does not leave silent half-state without status.

---

