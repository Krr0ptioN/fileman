use super::{
    selection::{delete_status, toggle_targets},
    state::{BrowserPanel, FileTarget, InputMode, PendingConfirm},
};

pub fn selected_target(panel: &BrowserPanel) -> Option<FileTarget> {
    panel
        .selected_row()
        .filter(|row| row.name != "..")
        .map(FileTarget::from_row)
}

pub fn effective_targets(panel: &BrowserPanel) -> Vec<FileTarget> {
    match panel.marked.is_empty() {
        true => selected_target(panel).into_iter().collect(),
        false => panel
            .rows
            .iter()
            .filter(|row| row.name != ".." && panel.marked.contains(&row.path))
            .map(FileTarget::from_row)
            .collect(),
    }
}

pub fn toggle_marked(panel: &mut BrowserPanel, count: usize) -> usize {
    if panel.rows.is_empty() {
        return 0;
    }

    for _ in 0..count.max(1) {
        let Some(row) = panel.rows.get(panel.selected_index) else {
            break;
        };
        let marked = std::sync::Arc::make_mut(&mut panel.marked);
        if row.name != ".." && !marked.remove(&row.path) {
            marked.insert(row.path.clone());
        }
        if panel.selected_index + 1 < panel.rows.len() {
            panel.selected_index += 1;
        }
    }

    panel.marked.len()
}

pub fn toggle_all_marks(panel: &mut BrowserPanel) -> String {
    let selectable = panel.rows.iter().filter(|row| row.name != "..").count();
    if selectable == 0 {
        return "nothing selectable".to_string();
    }

    match all_selectable_marked(panel) {
        true => {
            std::sync::Arc::make_mut(&mut panel.marked).clear();
            "marks cleared".to_string()
        }
        false => {
            let rows = panel.rows.clone();
            let marked = std::sync::Arc::make_mut(&mut panel.marked);
            for row in rows.iter() {
                if row.name != ".." {
                    marked.insert(row.path.clone());
                }
            }
            format!("{} marked", panel.marked.len())
        }
    }
}

pub fn prepare_delete(pending: &mut Option<PendingConfirm>, targets: Vec<FileTarget>) -> String {
    if targets.is_empty() {
        return "nothing selected".to_string();
    }

    match *pending {
        Some(PendingConfirm::Delete(ref mut selected)) => {
            let status = delete_status(toggle_targets(selected, &targets));
            if selected.is_empty() {
                *pending = None;
            }
            status
        }
        None => {
            let status = format!("delete {} item(s)? y/enter to confirm", targets.len());
            *pending = Some(PendingConfirm::Delete(targets));
            status
        }
    }
}

pub fn start_rename(input_mode: &mut InputMode, target: Option<FileTarget>) -> String {
    match target {
        Some(target) => {
            let status = format!("rename: {}", target.name);
            *input_mode = InputMode::Rename {
                input: target.name.clone(),
                target,
            };
            status
        }
        None => "nothing selected".to_string(),
    }
}

pub fn start_new_directory(input_mode: &mut InputMode, parent: std::path::PathBuf) -> String {
    *input_mode = InputMode::NewDirectory {
        parent,
        input: "new_dir".to_string(),
    };
    "new directory: new_dir".to_string()
}

pub fn start_quick_jump(input_mode: &mut InputMode, base: std::path::PathBuf) -> String {
    *input_mode = InputMode::QuickJump {
        base,
        input: String::new(),
    };
    "jump: ".to_string()
}

fn all_selectable_marked(panel: &BrowserPanel) -> bool {
    panel
        .rows
        .iter()
        .filter(|row| row.name != "..")
        .all(|row| panel.marked.contains(&row.path))
}
