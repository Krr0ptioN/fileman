use std::path;

use crate::archive::ContainerKind;

#[derive(Clone)]
pub enum IOTask {
    Copy {
        src: path::PathBuf,
        dst_dir: path::PathBuf,
    },
    CopyContainer {
        kind: ContainerKind,
        archive_path: path::PathBuf,
        inner_path: String,
        dst_dir: path::PathBuf,
        display_name: String,
    },
    CopyContainerDir {
        kind: ContainerKind,
        archive_path: path::PathBuf,
        inner_path: String,
        dst_dir: path::PathBuf,
        display_name: String,
    },
    Move {
        src: path::PathBuf,
        dst_dir: path::PathBuf,
    },
    Delete {
        target: path::PathBuf,
    },
    Rename {
        src: path::PathBuf,
        new_name: String,
    },
    WriteFile {
        path: path::PathBuf,
        contents: Vec<u8>,
    },
    Mkdir {
        path: path::PathBuf,
    },
    SetProps {
        path: path::PathBuf,
        mode: u32,
        uid: u32,
        gid: u32,
        recursive: bool,
    },
    Pack {
        sources: Vec<path::PathBuf>,
        archive_path: path::PathBuf,
        kind: ContainerKind,
    },
    WriteRemoteFile {
        host: String,
        path: String,
        contents: Vec<u8>,
    },
    CopyRemoteToLocal {
        host: String,
        remote_path: String,
        dst_dir: path::PathBuf,
        name: String,
        is_dir: bool,
    },
    CopyLocalToRemote {
        src: path::PathBuf,
        host: String,
        remote_dir: String,
        is_dir: bool,
    },
    DeleteRemote {
        host: String,
        items: Vec<(String, bool)>,
    },
    RenameRemote {
        host: String,
        src: String,
        new_name: String,
    },
    MkdirRemote {
        host: String,
        path: String,
    },
    CopyRemoteToLocalAndOpen {
        host: String,
        remote_path: String,
        local_path: path::PathBuf,
    },
    CopyRemoteSameHost {
        host: String,
        src_path: String,
        dst_dir: String,
        name: String,
    },
    MoveRemoteSameHost {
        host: String,
        src_path: String,
        dst_dir: String,
        name: String,
    },
    CopyContainerAndOpen {
        kind: ContainerKind,
        archive_path: path::PathBuf,
        inner_path: String,
        dst_dir: path::PathBuf,
        display_name: String,
    },
    CopyRemoteCrossHost {
        src_host: String,
        src_path: String,
        dst_host: String,
        dst_dir: String,
        name: String,
        is_dir: bool,
    },
    Elevated(Box<IOTask>),
}
