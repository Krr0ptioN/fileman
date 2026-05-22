pub fn format_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    let b = bytes as f64;
    match b {
        value if value >= GB => format!("{:.1}G", value / GB),
        value if value >= MB => format!("{:.1}M", value / MB),
        value if value >= KB => format!("{:.1}K", value / KB),
        _ => format!("{}B", bytes),
    }
}

pub fn format_date(epoch_secs: u64) -> String {
    let secs = epoch_secs as i64;
    let days_since_epoch = secs.div_euclid(86400);
    let time_of_day = secs.rem_euclid(86400) as u64;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;

    let z = days_since_epoch + 719468;
    let era = z.div_euclid(146097);
    let doe = z.rem_euclid(146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };

    format!("{y:04}-{m:02}-{d:02} {hours:02}:{minutes:02}")
}

pub fn format_mode(mode: u32) -> String {
    let file_type = if mode & 0o40000 != 0 {
        'd'
    } else if mode & 0o120000 != 0 {
        'l'
    } else {
        '-'
    };
    let mut out = String::with_capacity(10);
    out.push(file_type);
    for (mask, ch) in mode_bits() {
        out.push(if mode & mask != 0 { ch } else { '-' });
    }
    out
}

fn mode_bits() -> [(u32, char); 9] {
    [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'),
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'),
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'),
    ]
}
