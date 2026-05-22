use std::path::Path;

pub fn is_archive_path(path: &Path) -> bool {
    has_extension(path, &["zip", "tar", "gz", "bz2", "xz", "7z", "rar"])
}

pub fn is_archive_name(name: &str) -> bool {
    is_archive_path(Path::new(name))
}

pub fn is_audio_path(path: &Path) -> bool {
    has_extension(
        path,
        &[
            "mp3", "wav", "flac", "ogg", "opus", "m4a", "aac", "alac", "aiff", "wma",
        ],
    )
}

pub fn is_audio_name(name: &str) -> bool {
    is_audio_path(Path::new(name))
}

pub fn is_binary_path(path: &Path) -> bool {
    has_extension(path, &["exe", "dll", "so", "dylib", "bin", "o", "obj"])
}

pub fn is_binary_name(name: &str) -> bool {
    is_binary_path(Path::new(name))
}

pub fn is_code_path(path: &Path) -> bool {
    has_extension(
        path,
        &[
            "rs", "c", "cpp", "h", "hpp", "py", "js", "ts", "go", "java", "sh", "bat", "yml",
            "yaml", "toml", "json", "ron", "nix", "cmake",
        ],
    )
}

pub fn is_code_name(name: &str) -> bool {
    is_code_path(Path::new(name))
}

pub fn is_image_path(path: &Path) -> bool {
    has_extension(
        path,
        &[
            "png", "jpg", "jpeg", "gif", "bmp", "webp", "tga", "hdr", "dds", "tiff", "ico", "svg",
        ],
    )
}

pub fn is_image_name(name: &str) -> bool {
    is_image_path(Path::new(name))
}

pub fn is_media_name(name: &str) -> bool {
    is_image_name(name) || is_audio_name(name) || is_video_name(name)
}

pub fn is_pdf_path(path: &Path) -> bool {
    has_extension(path, &["pdf"])
}

pub fn is_pdf_name(name: &str) -> bool {
    is_pdf_path(Path::new(name))
}

pub fn is_text_path(path: &Path) -> bool {
    has_extension(
        path,
        &[
            "txt", "md", "json", "toml", "yaml", "yml", "rs", "log", "ini", "csv", "nix",
        ],
    )
}

pub fn is_text_name(name: &str) -> bool {
    is_text_path(Path::new(name))
}

pub fn is_video_path(path: &Path) -> bool {
    has_extension(
        path,
        &[
            "mp4", "m4v", "mkv", "avi", "mov", "webm", "mpg", "mpeg", "flv", "wmv",
        ],
    )
}

pub fn is_video_name(name: &str) -> bool {
    is_video_path(Path::new(name))
}

fn has_extension(path: &Path, extensions: &[&str]) -> bool {
    let Some(extension) = path.extension().and_then(|ext| ext.to_str()) else {
        return false;
    };
    extensions.contains(&extension.to_ascii_lowercase().as_str())
}
