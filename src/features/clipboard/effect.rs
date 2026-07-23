use std::path::PathBuf;

use gpui::{Context, UpdateGlobal};

use super::{
    copy::{copy_file_contents, copy_files, copy_target_name, copy_target_path},
    paste::{PastePlan, plan_paste},
    selection::prepare_clipboard,
    types::{ClipboardKind, ClipboardState},
};
use crate::features::file_browser::FileTarget;

pub enum ClipboardEffect {
    Prepare {
        kind: ClipboardKind,
        targets: Vec<FileTarget>,
    },
    CopyPath(Option<FileTarget>),
    CopyName(Option<FileTarget>),
    CopyFileContents(Option<FileTarget>),
    CopyFiles(Vec<FileTarget>),
    PasteInto(PathBuf),
}

pub struct ClipboardEffectOutcome {
    pub status: String,
    pub paste: Option<PastePlan>,
}

pub fn apply_clipboard_effect<T>(
    effect: ClipboardEffect,
    cx: &mut Context<T>,
) -> ClipboardEffectOutcome {
    match effect {
        ClipboardEffect::Prepare { kind, targets } => {
            let status = ClipboardState::update_global(cx, |clipboard, _| {
                prepare_clipboard(clipboard, kind, targets)
            });
            ClipboardEffectOutcome {
                status,
                paste: None,
            }
        }
        ClipboardEffect::CopyPath(target) => ClipboardEffectOutcome {
            status: copy_target_path(target, cx),
            paste: None,
        },
        ClipboardEffect::CopyName(target) => ClipboardEffectOutcome {
            status: copy_target_name(target, cx),
            paste: None,
        },
        ClipboardEffect::CopyFileContents(target) => ClipboardEffectOutcome {
            status: copy_file_contents(target, cx),
            paste: None,
        },
        ClipboardEffect::CopyFiles(targets) => ClipboardEffectOutcome {
            status: copy_files(targets, cx),
            paste: None,
        },
        ClipboardEffect::PasteInto(dst_dir) => paste_into(dst_dir, cx),
    }
}

fn paste_into<T>(dst_dir: PathBuf, cx: &mut Context<T>) -> ClipboardEffectOutcome {
    let plan = ClipboardState::update_global(cx, |clipboard, _| plan_paste(clipboard, dst_dir));
    match plan {
        PastePlan::Empty => ClipboardEffectOutcome {
            status: "clipboard empty".to_string(),
            paste: None,
        },
        plan => ClipboardEffectOutcome {
            status: String::new(),
            paste: Some(plan),
        },
    }
}
