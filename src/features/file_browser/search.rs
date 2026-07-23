use std::{fs, path, time};

use crate::core;

use super::VisibilityPolicy;

pub fn search_fs_filenames(
    root: &path::Path,
    query: &str,
    policy: VisibilityPolicy,
    cancelled: impl Fn() -> bool,
) -> anyhow::Result<Vec<core::DirEntry>> {
    let needle = query.to_lowercase();
    let mut builder = ignore::WalkBuilder::new(root);
    builder
        .hidden(!policy.show_hidden)
        .git_ignore(!policy.show_ignored)
        .git_global(!policy.show_ignored)
        .git_exclude(!policy.show_ignored)
        .parents(!policy.show_ignored)
        .follow_links(false);

    let mut results = Vec::new();
    for entry in builder.build() {
        if cancelled() {
            break;
        }
        let entry = entry?;
        if entry.path() == root {
            continue;
        }
        let name = entry.file_name().to_string_lossy();
        if !name.to_lowercase().contains(&needle) {
            continue;
        }

        let metadata = entry.metadata().ok();
        let is_dir = entry.file_type().is_some_and(|kind| kind.is_dir());
        let relative = entry.path().strip_prefix(root).unwrap_or(entry.path());
        results.push(core::DirEntry {
            name: relative.to_string_lossy().into_owned(),
            is_dir,
            is_symlink: entry.file_type().is_some_and(|kind| kind.is_symlink()),
            is_executable: executable(metadata.as_ref()),
            link_target: None,
            location: core::EntryLocation::Fs(entry.into_path()),
            size: metadata.as_ref().filter(|_| !is_dir).map(fs::Metadata::len),
            modified: metadata.as_ref().and_then(modified_secs),
        });
    }
    results.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(results)
}

#[cfg(unix)]
fn executable(metadata: Option<&fs::Metadata>) -> bool {
    use std::os::unix::fs::PermissionsExt;

    metadata.is_some_and(|metadata| metadata.permissions().mode() & 0o111 != 0)
}

#[cfg(not(unix))]
fn executable(_: Option<&fs::Metadata>) -> bool {
    false
}

fn modified_secs(metadata: &fs::Metadata) -> Option<u64> {
    metadata
        .modified()
        .ok()
        .and_then(|modified| modified.duration_since(time::UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn recursive_search_matches_names_case_insensitively() {
        let root = test_root("matches");
        fs::create_dir_all(root.join("nested")).unwrap();
        fs::write(root.join("nested/Needle.TXT"), "match").unwrap();
        fs::write(root.join("nested/other.txt"), "miss").unwrap();

        let results = search_fs_filenames(
            &root,
            "needle",
            VisibilityPolicy {
                show_hidden: false,
                show_ignored: false,
            },
            || false,
        )
        .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "nested/Needle.TXT");
        assert!(matches!(
            results[0].location,
            core::EntryLocation::Fs(ref path) if path == &root.join("nested/Needle.TXT")
        ));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn recursive_search_respects_hidden_and_gitignore_visibility() {
        let root = test_root("visibility");
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::create_dir_all(root.join(".hidden")).unwrap();
        fs::create_dir_all(root.join("ignored")).unwrap();
        fs::write(root.join(".gitignore"), "ignored/\n").unwrap();
        fs::write(root.join(".hidden/needle.txt"), "hidden").unwrap();
        fs::write(root.join("ignored/needle.txt"), "ignored").unwrap();
        fs::write(root.join("needle.txt"), "visible").unwrap();

        let results = search_fs_filenames(
            &root,
            "needle",
            VisibilityPolicy {
                show_hidden: false,
                show_ignored: false,
            },
            || false,
        )
        .unwrap();

        assert_eq!(
            results
                .iter()
                .map(|entry| entry.name.as_str())
                .collect::<Vec<_>>(),
            vec!["needle.txt"]
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn cancelled_search_stops_without_results() {
        let root = test_root("cancelled");
        fs::write(root.join("needle.txt"), "match").unwrap();

        let results = search_fs_filenames(
            &root,
            "needle",
            VisibilityPolicy {
                show_hidden: true,
                show_ignored: true,
            },
            || true,
        )
        .unwrap();

        assert!(results.is_empty());
        fs::remove_dir_all(root).unwrap();
    }

    fn test_root(suffix: &str) -> path::PathBuf {
        let root =
            std::env::temp_dir().join(format!("stiff-search-{}-{suffix}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        root
    }
}
