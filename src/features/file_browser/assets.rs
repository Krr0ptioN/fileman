use std::borrow::Cow;

use gpui::{AssetSource, SharedString};

pub struct FilemanAssets;

impl AssetSource for FilemanAssets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        Ok(lucide_svg(path).map(|svg| Cow::Borrowed(svg.as_bytes())))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        if path == "icons" {
            Ok(LUCIDE_ASSETS
                .iter()
                .map(|&(name, _)| SharedString::from(name))
                .collect())
        } else {
            Ok(Vec::new())
        }
    }
}

const LUCIDE_ASSETS: &[(&str, &str)] = &[
    (
        "copy.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>"#,
    ),
    (
        "delete.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 11v6"/><path d="M14 11v6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M3 6h18"/><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>"#,
    ),
    (
        "external-link.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 3h6v6"/><path d="M10 14 21 3"/><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/></svg>"#,
    ),
    (
        "file.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/></svg>"#,
    ),
    (
        "folder-closed.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><path d="M2 10h20"/></svg>"#,
    ),
    (
        "globe.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/></svg>"#,
    ),
    (
        "info.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>"#,
    ),
    (
        "minus.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/></svg>"#,
    ),
    (
        "replace.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 4a2 2 0 0 1 2-2"/><path d="M16 2a2 2 0 0 1 2 2"/><path d="M20 6v2a2 2 0 0 1-2 2h-2"/><path d="M4 14v-2a2 2 0 0 1 2-2h2"/><path d="M8 20a2 2 0 0 1-2 2"/><path d="M6 22a2 2 0 0 1-2-2"/><path d="m18 14 4 4-4 4"/><path d="m6 10-4-4 4-4"/></svg>"#,
    ),
    (
        "settings.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.38a2 2 0 0 0-.73-2.73l-.15-.09a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2Z"/><circle cx="12" cy="12" r="3"/></svg>"#,
    ),
    (
        "square-terminal.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m7 11 2-2-2-2"/><path d="M11 13h4"/><rect width="18" height="18" x="3" y="3" rx="2" ry="2"/></svg>"#,
    ),
    (
        "star.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11.5 2.3a.6.6 0 0 1 1 0l2.9 5.9 6.5.9a.6.6 0 0 1 .3 1l-4.7 4.6 1.1 6.5a.6.6 0 0 1-.9.6L12 18.8l-5.8 3a.6.6 0 0 1-.9-.6l1.1-6.5-4.7-4.6a.6.6 0 0 1 .3-1l6.5-.9Z"/></svg>"#,
    ),
    (
        "star-off.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m2 2 20 20"/><path d="M8.5 8.2 11.5 2.3a.6.6 0 0 1 1 0l2.9 5.9 6.5.9a.6.6 0 0 1 .3 1l-3.5 3.4"/><path d="m14.7 14.7 3.9 6.5a.6.6 0 0 1-.9.6L12 18.8l-5.8 3a.6.6 0 0 1-.9-.6l1.1-6.5-4.7-4.6a.6.6 0 0 1 .3-1l2.7-.4"/></svg>"#,
    ),
];

fn lucide_svg(path: &str) -> Option<&'static str> {
    let name = path.strip_prefix("icons/").unwrap_or(path);
    LUCIDE_ASSETS
        .iter()
        .find_map(|&(asset_name, svg)| (asset_name == name).then_some(svg))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_icons_with_or_without_prefix() {
        assert!(lucide_svg("icons/file.svg").is_some());
        assert!(lucide_svg("file.svg").is_some());
        assert!(lucide_svg("icons/not-real.svg").is_none());
    }

    #[test]
    fn lists_icon_names() {
        let assets = FilemanAssets;
        let names = assets.list("icons").expect("icon list");
        assert!(names.iter().any(|name| name.as_ref() == "file.svg"));
        assert!(names.iter().any(|name| name.as_ref() == "settings.svg"));
        assert!(assets.list("missing").expect("missing list").is_empty());
    }
}
