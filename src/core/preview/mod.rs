use std::{
    fs,
    io::{self, BufRead, Read},
    path::Path,
};

use crate::core::{EntryLocation, container_display_path};

pub fn format_preview_info(kind: &str, location: &EntryLocation) -> String {
    match *location {
        EntryLocation::Fs(ref path) => format!("{kind}\n{}", path.to_string_lossy()),
        EntryLocation::Container {
            kind: container_kind,
            ref archive_path,
            ref inner_path,
        } => {
            let display = container_display_path(container_kind, archive_path, inner_path);
            format!("{kind}\n{display}")
        }
        EntryLocation::Remote { ref host, ref path } => format!("{kind}\n{host}:{path}"),
    }
}

pub fn read_text_preview(path: &Path, max_bytes: usize) -> anyhow::Result<String> {
    let bytes = read_prefix(path, max_bytes)?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextPreviewRead {
    pub text: String,
    pub lines_read: usize,
    pub bytes_read: usize,
    pub truncated: bool,
}

pub fn read_text_lines_prefix(
    path: &Path,
    first_line: usize,
    max_lines: usize,
    max_bytes: usize,
) -> anyhow::Result<TextPreviewRead> {
    let file = fs::File::open(path)?;
    let file_len = file.metadata().ok().map(|metadata| metadata.len());
    let mut reader = io::BufReader::new(file.take(max_bytes as u64));
    let mut text = String::new();
    let mut line = Vec::new();
    let mut lines_read = 0usize;
    let mut bytes_read = 0usize;

    while lines_read < first_line.saturating_add(max_lines) {
        line.clear();
        let read = reader.read_until(b'\n', &mut line)?;
        if read == 0 {
            break;
        }

        bytes_read += read;
        if lines_read >= first_line {
            text.push_str(&String::from_utf8_lossy(&line));
        }
        lines_read += 1;

        if bytes_read >= max_bytes {
            break;
        }
    }

    let loaded_lines = lines_read.saturating_sub(first_line).min(max_lines);
    Ok(TextPreviewRead {
        text,
        lines_read: loaded_lines,
        bytes_read,
        truncated: file_len
            .map(|len| (bytes_read as u64) < len)
            .unwrap_or(bytes_read >= max_bytes),
    })
}

pub fn read_bytes_prefix(path: &Path, max_bytes: usize) -> anyhow::Result<Vec<u8>> {
    Ok(read_prefix(path, max_bytes)?)
}

pub fn hexdump(bytes: &[u8]) -> String {
    hexdump_with_width(bytes, 16)
}

pub fn hexdump_with_width(bytes: &[u8], width: usize) -> String {
    let width = width.clamp(4, 32);
    let mut out = String::new();
    for (offset, chunk) in bytes.chunks(width).enumerate() {
        write_hex_line(&mut out, offset * width, chunk, width);
    }
    out
}

pub fn is_probably_text(bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        return true;
    }
    if bytes.contains(&0) {
        return false;
    }
    let bytes = bytes.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(bytes);
    std::str::from_utf8(bytes).is_ok() || printable_ratio(bytes) > 0.85
}

fn read_prefix(path: &Path, max_bytes: usize) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(path)?;
    let mut buf = Vec::new();
    file.by_ref().take(max_bytes as u64).read_to_end(&mut buf)?;
    Ok(buf)
}

fn write_hex_line(out: &mut String, offset: usize, chunk: &[u8], width: usize) {
    out.push_str(&format!("{offset:08x}: "));
    for i in 0..width {
        if i < chunk.len() {
            out.push_str(&format!("{:02x} ", chunk[i]));
        } else {
            out.push_str("   ");
        }
        if i == (width / 2).saturating_sub(1) {
            out.push(' ');
        }
    }
    out.push(' ');
    for &byte in chunk {
        out.push(if (0x20..=0x7e).contains(&byte) {
            byte as char
        } else {
            '.'
        });
    }
    out.push('\n');
}

fn printable_ratio(bytes: &[u8]) -> f32 {
    let printable = bytes
        .iter()
        .filter(|byte| matches!(byte, 0x09 | 0x0A | 0x0D | 0x20..=0x7E | 0x80..=0xFF))
        .count();
    printable as f32 / bytes.len().max(1) as f32
}
