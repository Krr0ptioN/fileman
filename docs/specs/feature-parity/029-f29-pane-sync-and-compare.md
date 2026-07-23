> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#30](https://github.com/Krr0ptioN/fileman/issues/30)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P1**

# F29 — Pane Sync and Compare (P1)

**Intent:** Coordinate dual panes for transfer and difference review.

## User Stories

1. As a stiff user, I want pane sync and compare, so that dual-pane copy/review workflows are deliberate.
2. As a stiff user, I want one-key copy/move to the opposite pane, so that dual-pane transfers match classic FM muscle memory.
3. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
4. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-29.1: Sync cwd: secondary follows primary (optional relative sync).
2. FR-29.2: One-key copy/move selection to opposite pane destination.
3. FR-29.3: Compare mode highlights names present only on left/right (basic name set diff; content diff P2).
4. FR-29.4: Sync can be toggled off; no surprising cwd changes when off.
5. FR-29.5: Works with tabs (sync applies to active tabs).

**Acceptance criteria**

- AC-29.1: Copy-to-opposite lands files in other pane cwd.
- AC-29.2: Compare marks unique names on each side.

---

