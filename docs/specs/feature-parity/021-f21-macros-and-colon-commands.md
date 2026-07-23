> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#22](https://github.com/Krr0ptioN/fileman/issues/22)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P1**

# F21 — Macros and Colon Commands (P1)

**Intent:** Compose multi-command sequences and run named/ex commands.

## User Stories

1. As a stiff user, I want macros and colon commands, so that I can compose and repeat multi-step workflows.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-21.1: Colon prompt parses command name + args into domain commands.
2. FR-21.2: Built-in commands mirror essential browser ops (cd, delete, mark, sort, filter, …).
3. FR-21.3: Macro record/replay of command sequences (optional P2 if colon lands first).
4. FR-21.4: Unknown commands error in status, no panic.
5. FR-21.5: Help lists colon commands.

**Acceptance criteria**

- AC-21.1: `:sort size` (example) changes sort mode via command seam.
- AC-21.2: Invalid colon input shows error status.

---

