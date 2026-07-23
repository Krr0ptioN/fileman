> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#29](https://github.com/Krr0ptioN/fileman/issues/29)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P2**

# F28 — Thumbnail / Grid Mode (P2)

**Intent:** Visual browsing for image-heavy directories.

## User Stories

1. As a stiff user, I want a thumbnail/grid mode, so that image-heavy folders are scannable visually.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-28.1: Toggle list vs grid/thumbnail layout for the active panel.
2. FR-28.2: Thumbnails decoded off UI thread; cache bounded (memory + count).
3. FR-28.3: Keyboard selection and marks work in grid.
4. FR-28.4: Non-image files show icons/placeholders.
5. FR-28.5: Scroll virtualizes grid cells.

**Acceptance criteria**

- AC-28.1: Opening a folder of images shows thumbnails without freezing input.
- AC-28.2: Memory cache respects bound under stress test of many images.

---

