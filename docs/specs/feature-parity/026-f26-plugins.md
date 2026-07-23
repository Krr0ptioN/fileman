> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#27](https://github.com/Krr0ptioN/fileman/issues/27)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P2**

# F26 — Plugins (P2)

**Intent:** Extend commands and hooks without forking stiff.

## User Stories

1. As a stiff user, I want a plugin surface, so that community extensions can add commands without forking.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-26.1: Define a minimal plugin API: register command name, invoke on selection context, return effect/status.
2. FR-26.2: Plugins cannot block UI thread; long work is async.
3. FR-26.3: Load from a plugins directory; failures isolate to that plugin.
4. FR-26.4: Security: no implicit network; document trust model (local code executes with user privileges).
5. FR-26.5: First version may be process-spawn plugins (external binaries) before in-process scripting.

**Acceptance criteria**

- AC-26.1: Sample plugin registers and runs from colon/key.
- AC-26.2: Crashing plugin does not take down stiff process (for out-of-process); in-process must catch panics at boundary if used.

---

