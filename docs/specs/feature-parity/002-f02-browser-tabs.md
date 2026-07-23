> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#3](https://github.com/Krr0ptioN/fileman/issues/3)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P0**

# F02 — Browser Tabs (P0)

**Intent:** Multiple directory contexts per pane without requiring dual-pane.

## User Stories

1. As a stiff user, I want browser tabs per pane, so that I can keep multiple directories open without dual-pane only.
2. As a stiff user, I want to open, switch, and close tabs from the keyboard, so that tab workflow stays Vim-native.
3. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
4. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-2.1: Each browser pane owns an ordered tab list with one active tab.
2. FR-2.2: New tab clones the current directory (and later: optional empty/home).
3. FR-2.3: Next/previous tab switching is keyboard-driven.
4. FR-2.4: Close tab activates a neighbor; closing the last tab is a no-op (pane remains).
5. FR-2.5: Tab state includes cwd, selection, marks, scroll/reveal intent, and visibility flags owned by that tab.
6. FR-2.6: Dual-pane mode keeps independent tab stacks per pane.
7. FR-2.7: Reload and navigation mutate only the active tab.

**Acceptance criteria**

- AC-2.1: Open tab, navigate away, switch back — prior directory and selection restored.
- AC-2.2: Close non-last tab does not close the pane.
- AC-2.3: Panel switch (Tab) still switches panes, not tabs, unless remapped.

---

