# Feature Parity Specs (Ranger / xplr / peers)

Index of per-feature requirements for stiff file-manager parity work.

- Shared seams, NFs, testing: [000-shared.md](./000-shared.md)
- Parent tracking issue: [#1](https://github.com/Krr0ptioN/fileman/issues/1)
- Legacy monolith pointer: [../file-manager-feature-parity.md](../file-manager-feature-parity.md)

Suggested delivery order: **F31** + **F24** first, then remaining **P0**, then **P1**, then **P2**.

| ID | Feature | Priority | Issue |
|----|---------|----------|-------|
| F01 | [Filename Search](./001-f01-filename-search.md) | P0 | [#2](https://github.com/Krr0ptioN/fileman/issues/2) |
| F02 | [Browser Tabs](./002-f02-browser-tabs.md) | P0 | [#3](https://github.com/Krr0ptioN/fileman/issues/3) |
| F03 | [Navigation History](./003-f03-navigation-history.md) | P0 | [#4](https://github.com/Krr0ptioN/fileman/issues/4) |
| F04 | [Archive Browse, Extract, Compress](./004-f04-archive-browse-extract-compress.md) | P0 | [#5](https://github.com/Krr0ptioN/fileman/issues/5) |
| F05 | [In-App Editor](./005-f05-in-app-editor.md) | P0 | [#6](https://github.com/Krr0ptioN/fileman/issues/6) |
| F06 | [SFTP Remote Browsing](./006-f06-sftp-remote-browsing.md) | P0 | [#7](https://github.com/Krr0ptioN/fileman/issues/7) |
| F07 | [Elevated Permissions (chmod / chown / privileged IO)](./007-f07-elevated-permissions-chmod-chown-privileged-io.md) | P1 | [#8](https://github.com/Krr0ptioN/fileman/issues/8) |
| F08 | [Bookmarks](./008-f08-bookmarks.md) | P0 | [#9](https://github.com/Krr0ptioN/fileman/issues/9) |
| F09 | [Tags](./009-f09-tags.md) | P2 | [#10](https://github.com/Krr0ptioN/fileman/issues/10) |
| F10 | [Sort Modes](./010-f10-sort-modes.md) | P0 | [#11](https://github.com/Krr0ptioN/fileman/issues/11) |
| F11 | [Incremental Filter / Typeahead](./011-f11-incremental-filter-typeahead.md) | P0 | [#12](https://github.com/Krr0ptioN/fileman/issues/12) |
| F12 | [Flattened Recursive View](./012-f12-flattened-recursive-view.md) | P1 | [#13](https://github.com/Krr0ptioN/fileman/issues/13) |
| F13 | [Bulk Rename](./013-f13-bulk-rename.md) | P1 | [#14](https://github.com/Krr0ptioN/fileman/issues/14) |
| F14 | [Open-With / Rifle Rules](./014-f14-open-with-rifle-rules.md) | P0 | [#15](https://github.com/Krr0ptioN/fileman/issues/15) |
| F15 | [Shell Command on Selection](./015-f15-shell-command-on-selection.md) | P0 | [#16](https://github.com/Krr0ptioN/fileman/issues/16) |
| F16 | [Trash-Aware Delete](./016-f16-trash-aware-delete.md) | P0 | [#17](https://github.com/Krr0ptioN/fileman/issues/17) |
| F17 | [Create File, Symlink, Hardlink](./017-f17-create-file-symlink-hardlink.md) | P0 | [#18](https://github.com/Krr0ptioN/fileman/issues/18) |
| F18 | [Metadata Columns](./018-f18-metadata-columns.md) | P1 | [#19](https://github.com/Krr0ptioN/fileman/issues/19) |
| F19 | [VCS Status in Listing](./019-f19-vcs-status-in-listing.md) | P1 | [#20](https://github.com/Krr0ptioN/fileman/issues/20) |
| F20 | [Fuzzy Path Jump (fzf / zoxide style)](./020-f20-fuzzy-path-jump-fzf-zoxide-style.md) | P1 | [#21](https://github.com/Krr0ptioN/fileman/issues/21) |
| F21 | [Macros and Colon Commands](./021-f21-macros-and-colon-commands.md) | P1 | [#22](https://github.com/Krr0ptioN/fileman/issues/22) |
| F22 | [Miller Columns](./022-f22-miller-columns.md) | P2 | [#23](https://github.com/Krr0ptioN/fileman/issues/23) |
| F23 | [Undo File Operations](./023-f23-undo-file-operations.md) | P1 | [#24](https://github.com/Krr0ptioN/fileman/issues/24) |
| F24 | [Paste Conflict Resolution](./024-f24-paste-conflict-resolution.md) | P0 | [#25](https://github.com/Krr0ptioN/fileman/issues/25) |
| F25 | [Config File and Key Remapping](./025-f25-config-file-and-key-remapping.md) | P0 | [#26](https://github.com/Krr0ptioN/fileman/issues/26) |
| F26 | [Plugins](./026-f26-plugins.md) | P2 | [#27](https://github.com/Krr0ptioN/fileman/issues/27) |
| F27 | [Additional Remotes (SMB / WebDAV / …)](./027-f27-additional-remotes-smb-webdav.md) | P2 | [#28](https://github.com/Krr0ptioN/fileman/issues/28) |
| F28 | [Thumbnail / Grid Mode](./028-f28-thumbnail-grid-mode.md) | P2 | [#29](https://github.com/Krr0ptioN/fileman/issues/29) |
| F29 | [Pane Sync and Compare](./029-f29-pane-sync-and-compare.md) | P1 | [#30](https://github.com/Krr0ptioN/fileman/issues/30) |
| F30 | [Disk Usage Insight](./030-f30-disk-usage-insight.md) | P1 | [#31](https://github.com/Krr0ptioN/fileman/issues/31) |
| F31 | [Task / Progress Queue](./031-f31-task-progress-queue.md) | P0 | [#32](https://github.com/Krr0ptioN/fileman/issues/32) |
| F32 | [Glob Marking](./032-f32-glob-marking.md) | P1 | [#33](https://github.com/Krr0ptioN/fileman/issues/33) |
| F33 | [Mount / Volume Browser](./033-f33-mount-volume-browser.md) | P2 | [#34](https://github.com/Krr0ptioN/fileman/issues/34) |
| F34 | [Content Search](./034-f34-content-search.md) | P1 | [#35](https://github.com/Krr0ptioN/fileman/issues/35) |
