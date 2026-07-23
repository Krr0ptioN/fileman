> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#26](https://github.com/Krr0ptioN/fileman/issues/26)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P0**

# F25 — Config File and Key Remapping (P0)

**Intent:** User-editable configuration for keys, defaults, and feature toggles.

## User Stories

1. As a stiff user, I want a config file and remappable keys, so that stiff fits my muscle memory.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-25.1: Documented config path and format (TOML or RON — pick one).
2. FR-25.2: Remap key sequences to existing `BrowserCommand`s / control actions.
3. FR-25.3: Configure defaults: sort, trash vs permanent, hidden, columns, rifle rules path.
4. FR-25.4: Invalid config loads defaults + error status rather than refusing to start (or fail-soft with dialog).
5. FR-25.5: Reload config command without full restart when safe.
6. FR-25.6: Ship example config in repo docs.

**Acceptance criteria**

- AC-25.1: Remapped key invokes target command in test.
- AC-25.2: Broken config does not hard-crash startup.

---

