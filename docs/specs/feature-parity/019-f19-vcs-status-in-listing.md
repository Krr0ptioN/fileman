> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#20](https://github.com/Krr0ptioN/fileman/issues/20)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P1**

# F19 — VCS Status in Listing (P1)

**Intent:** Show git (initial VCS) status decorations on rows.

## User Stories

1. As a stiff user, I want VCS status in the listing, so that dirty/ignored/untracked files are visible while browsing a repo.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-19.1: Detect repo root for cwd; decorate modified/added/untracked/ignored/conflicted.
2. FR-19.2: Status computation is async and coalesced; stale updates ignored.
3. FR-19.3: Toggle decorations on/off.
4. FR-19.4: Works with existing gitignore hide (`gH`) without contradiction.
5. FR-19.5: Non-git directories show no decorations (no error spam).

**Acceptance criteria**

- AC-19.1: Modified file in a test repo shows modified decoration.
- AC-19.2: Outside a repo, listing unchanged aside from absent decorations.

---

