use std::path::{Path, PathBuf};

use ignore::gitignore::{Gitignore, GitignoreBuilder};

use crate::core::DirEntry;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VisibilityPolicy {
    pub show_hidden: bool,
    pub show_ignored: bool,
}

pub fn read_visible_fs_directory(
    directory: &Path,
    policy: VisibilityPolicy,
) -> anyhow::Result<Vec<DirEntry>> {
    let matcher = match policy.show_ignored {
        true => None,
        false => matcher_for_directory(directory),
    };
    crate::core::read_fs_directory_filtered(
        directory,
        |name| policy.show_hidden || !name.as_encoded_bytes().starts_with(b"."),
        |name, kind| {
            !matcher.as_ref().is_some_and(|matcher| {
                matcher
                    .matched_path_or_any_parents(
                        directory.join(name),
                        kind == crate::core::FsEntryKind::Directory,
                    )
                    .is_ignore()
            })
        },
    )
}

pub fn hide_gitignored_entries(directory: &Path, entries: Vec<DirEntry>) -> Vec<DirEntry> {
    let Some(matcher) = matcher_for_directory(directory) else {
        return entries;
    };

    entries
        .into_iter()
        .filter(|entry| {
            !matcher
                .matched_path_or_any_parents(directory.join(&entry.name), entry.is_dir)
                .is_ignore()
        })
        .collect()
}

pub fn path_is_gitignored(path: &Path, is_dir: bool) -> bool {
    path.parent()
        .and_then(matcher_for_directory)
        .is_some_and(|matcher| {
            matcher
                .matched_path_or_any_parents(path, is_dir)
                .is_ignore()
        })
}

fn matcher_for_directory(directory: &Path) -> Option<Gitignore> {
    let root = repository_root(directory)?;
    let mut directories = directory
        .ancestors()
        .take_while(|candidate| *candidate != root)
        .collect::<Vec<_>>();
    directories.push(root.as_path());
    directories.reverse();

    let mut builder = GitignoreBuilder::new(&root);
    for directory in directories {
        let gitignore = directory.join(".gitignore");
        if gitignore.is_file() {
            let _ = builder.add(gitignore);
        }
    }

    builder.build().ok()
}

fn repository_root(path: &Path) -> Option<PathBuf> {
    path.ancestors()
        .find(|directory| {
            let marker = directory.join(".git");
            marker.is_dir() || marker.is_file()
        })
        .map(Path::to_path_buf)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::core;

    #[test]
    fn hides_gitignored_entries_and_keeps_visible_entries() {
        let root = test_repository("filter");
        fs::write(root.join(".gitignore"), "generated/\n").unwrap();
        fs::create_dir_all(root.join("generated")).unwrap();
        fs::write(root.join("keep.txt"), "keep").unwrap();

        let entries = hide_gitignored_entries(&root, core::read_fs_directory(&root).unwrap());

        assert_eq!(
            entries
                .iter()
                .map(|entry| entry.name.as_str())
                .collect::<Vec<_>>(),
            vec![".git", ".gitignore", "keep.txt"]
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn nested_whitelist_restores_entry_visibility() {
        let root = test_repository("whitelist");
        fs::write(root.join(".gitignore"), "generated/*\n").unwrap();
        fs::create_dir_all(root.join("generated")).unwrap();
        fs::write(root.join("generated/.gitignore"), "!keep.txt\n").unwrap();
        fs::write(root.join("generated/keep.txt"), "keep").unwrap();
        fs::write(root.join("generated/drop.txt"), "drop").unwrap();

        let directory = root.join("generated");
        let entries =
            hide_gitignored_entries(&directory, core::read_fs_directory(&directory).unwrap());

        assert_eq!(
            entries
                .iter()
                .map(|entry| entry.name.as_str())
                .collect::<Vec<_>>(),
            vec!["keep.txt"]
        );

        fs::remove_dir_all(root).unwrap();
    }

    fn test_repository(suffix: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("stiff-ignored-{}-{suffix}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join(".git")).unwrap();
        root
    }
}
