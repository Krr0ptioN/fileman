> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#4](https://github.com/Krr0ptioN/fileman/issues/4)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P0**

# F03 — Navigation History (P0)

**Intent:** Back/forward through visited directories per tab/pane.

## User Stories

1. As a stiff user, I want back/forward directory history, so that I can retrace navigation like a browser.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-3.1: Successful directory changes push history (open selected, open parent, quick jump, search jump).
2. FR-3.2: Back restores previous cwd and best-effort selection.
3. FR-3.3: Forward works after back until a new branch truncates forward stack.
4. FR-3.4: History is per tab (preferred) or per pane if tabs absent.
5. FR-3.5: Failed loads do not push history.
6. FR-3.6: Keys: prior art Alt+Left / Alt+Right / Backspace for back.

**Acceptance criteria**

- AC-3.1: Enter dir → back → forward returns to entered dir.
- AC-3.2: After back, navigating elsewhere clears forward stack.

---

