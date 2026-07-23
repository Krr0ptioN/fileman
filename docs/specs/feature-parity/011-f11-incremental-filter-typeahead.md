> Part of [Feature Parity index](./README.md).  
> GitHub issue: [#12](https://github.com/Krr0ptioN/fileman/issues/12)  
> Inherits [shared requirements](./000-shared.md) (NFs, seams, testing).  
> Priority: **P0**

# F11 — Incremental Filter / Typeahead (P0)

**Intent:** Narrow the current listing as the user types, without a separate search results page.

## User Stories

1. As a stiff user, I want incremental filter/typeahead, so that long directories shrink to matching rows while I type.
2. As a maintainer, I want each feature behind the command seam, so that agents can implement and test without GPUI coupling.
3. As a maintainer, I want acceptance criteria per feature, so that partial ports are detectable.

**Functional requirements**

1. FR-11.1: Activate filter mode; typed characters narrow visible rows.
2. FR-11.2: Selection stays on best match; movement operates within filtered set.
3. FR-11.3: Clear filter restores full listing and prior selection when possible.
4. FR-11.4: Filter does not mutate disk; marks outside filter remain until cleared.
5. FR-11.5: Optional fuzzy vs substring match modes.
6. FR-11.6: Works with large dirs via index/filter over row model, not full re-read.

**Acceptance criteria**

- AC-11.1: Type unique prefix → one row → Enter opens that entry.
- AC-11.2: Escape clears filter and shows full list.

---

