> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#15](https://github.com/Krr0ptioN/fileman/issues/15)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P0**

# F14 — Open-With / Rifle Rules (P0)

**Intent:** Map file types to external programs with ordered rules.

## User Stories

1. As a stiff user, I want open-with rules (rifle-style), so that Enter opens the right program per file type.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-14.1: Rule table: match by mime/extension/name → command template.
2. FR-14.2: Enter/open uses first matching rule; fallback to system open.
3. FR-14.3: User can pick from multiple matches (open-with picker).
4. FR-14.4: Directories have separate policy (enter vs open in file manager vs external).
5. FR-14.5: Rules loaded from config (F28); sane built-in defaults ship.
6. FR-14.6: Commands run async; failures show stderr/status summary.

**Acceptance criteria**

- AC-14.1: Configured extension opens with configured program in test double/harness.
- AC-14.2: Unknown type falls back without crash.

---

