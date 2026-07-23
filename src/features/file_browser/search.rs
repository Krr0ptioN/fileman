use std::path;

use crate::core;

use super::VisibilityPolicy;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilenameSearchScope {
    CurrentDirectory,
    Recursive,
}

impl FilenameSearchScope {
    pub fn label(self) -> &'static str {
        match self {
            Self::CurrentDirectory => "here",
            Self::Recursive => "tree",
        }
    }
}

pub fn search_fs_filenames(
    root: &path::Path,
    query: &str,
    scope: FilenameSearchScope,
    policy: VisibilityPolicy,
    cancelled: impl Fn() -> bool,
) -> anyhow::Result<Vec<core::DirEntry>> {
    let needle = query.to_lowercase();
    let mut builder = ignore::WalkBuilder::new(root);
    builder
        .hidden(!policy.show_hidden)
        .ignore(false)
        .git_ignore(!policy.show_ignored)
        .git_global(false)
        .git_exclude(false)
        .parents(!policy.show_ignored)
        .follow_links(false);
    if scope == FilenameSearchScope::CurrentDirectory {
        builder.max_depth(Some(1));
    }

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

        let relative = entry.path().strip_prefix(root).unwrap_or(entry.path());
        results.push(core::fs_entry_from_path(
            entry.path(),
            relative.to_string_lossy().into_owned(),
        )?);
    }
    results.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(results)
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
            FilenameSearchScope::Recursive,
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
            FilenameSearchScope::Recursive,
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
            FilenameSearchScope::Recursive,
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

    #[test]
    fn current_directory_scope_does_not_descend() {
        let root = test_root("scope");
        fs::create_dir_all(root.join("nested")).unwrap();
        fs::write(root.join("needle.txt"), "top").unwrap();
        fs::write(root.join("nested/needle.txt"), "nested").unwrap();

        let results = search_fs_filenames(
            &root,
            "needle",
            FilenameSearchScope::CurrentDirectory,
            VisibilityPolicy {
                show_hidden: false,
                show_ignored: false,
            },
            || false,
        )
        .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "needle.txt");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn visibility_matches_browser_gitignore_sources() {
        let root = test_root("show-ignored");
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::write(root.join(".gitignore"), "gitignored-needle.txt\n").unwrap();
        fs::write(root.join(".ignore"), "local-needle.txt\n").unwrap();
        fs::write(root.join("gitignored-needle.txt"), "gitignore").unwrap();
        fs::write(root.join("local-needle.txt"), "ignore-file").unwrap();

        let visible = search_fs_filenames(
            &root,
            "needle",
            FilenameSearchScope::Recursive,
            VisibilityPolicy {
                show_hidden: false,
                show_ignored: false,
            },
            || false,
        )
        .unwrap();
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].name, "local-needle.txt");

        let all = search_fs_filenames(
            &root,
            "needle",
            FilenameSearchScope::Recursive,
            VisibilityPolicy {
                show_hidden: false,
                show_ignored: true,
            },
            || false,
        )
        .unwrap();
        assert_eq!(all.len(), 2);
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
