> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#16](https://github.com/Krr0ptioN/fileman/issues/16)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P0**

# F15 — Shell Command on Selection (P0)

**Intent:** Run `$SHELL -c` / explicit command with selection path placeholders.

## User Stories

1. As a stiff user, I want to run shell commands on the selection, so that arbitrary tools stay one keystroke away.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-15.1: Prompt for command with placeholders (`%f` selected, `%s` marked, `%d` cwd — final tokens TBD, document in help).
2. FR-15.2: Foreground wait vs background run modes (Ranger `!` vs `&` analogues).
3. FR-15.3: Working directory is active panel cwd.
4. FR-15.4: Output optionally captured to status/preview pane (P1 enhancement).
5. FR-15.5: Confirm when command is empty or dangerous patterns if policy enabled.
6. FR-15.6: Does not block key input handling thread; process IO is async.

**Acceptance criteria**

- AC-15.1: `echo %f` with selection runs and reports success.
- AC-15.2: Background mode returns focus immediately.

---

