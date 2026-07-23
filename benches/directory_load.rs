use std::{
    fs,
    hint::black_box,
    path::Path,
    time::{Duration, Instant},
};

const DEFAULT_ENTRY_COUNT: usize = 10_000;
const SAMPLE_COUNT: usize = 10;

fn main() {
    let entry_count = std::env::var("STIFF_BENCH_ENTRIES")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(DEFAULT_ENTRY_COUNT);
    let root = std::env::temp_dir().join(format!("stiff-directory-bench-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    create_fixture(&root, entry_count);

    let all_entry_samples = (0..SAMPLE_COUNT)
        .map(|_| {
            let start = Instant::now();
            let entries = stiff::core::read_fs_directory(black_box(&root))
                .expect("benchmark directory should load");
            black_box(entries);
            start.elapsed()
        })
        .collect::<Vec<_>>();
    let legacy_hidden_filter_samples = (0..SAMPLE_COUNT)
        .map(|_| {
            let start = Instant::now();
            let mut entries = stiff::core::read_fs_directory(black_box(&root))
                .expect("benchmark directory should load");
            entries.retain(|entry| !entry.name.starts_with('.'));
            black_box(entries);
            start.elapsed()
        })
        .collect::<Vec<_>>();
    let filtered_ingestion_samples = (0..SAMPLE_COUNT)
        .map(|_| {
            let start = Instant::now();
            let entries = stiff::features::file_browser::read_visible_fs_directory(
                &root,
                stiff::features::file_browser::VisibilityPolicy {
                    show_hidden: false,
                    show_ignored: true,
                },
            )
            .expect("benchmark directory should load");
            black_box(entries);
            start.elapsed()
        })
        .collect::<Vec<_>>();

    let _ = fs::remove_dir_all(&root);
    report("all_entries", entry_count, &all_entry_samples);
    report(
        "hide_after_loading",
        entry_count,
        &legacy_hidden_filter_samples,
    );
    report(
        "filter_during_ingestion",
        entry_count,
        &filtered_ingestion_samples,
    );
}

fn create_fixture(root: &Path, entry_count: usize) {
    fs::create_dir_all(root).expect("create benchmark root");
    for index in 0..entry_count {
        let extension = match index % 4 {
            0 => "rs",
            1 => "txt",
            2 => "jpg",
            _ => "unknown",
        };
        let prefix = if index % 2 == 0 { ".hidden" } else { "entry" };
        fs::write(root.join(format!("{prefix}-{index:08}.{extension}")), b"x")
            .expect("create benchmark entry");
    }
}

fn report(name: &str, entry_count: usize, samples: &[Duration]) {
    let total = samples.iter().sum::<Duration>();
    let mean = total / samples.len() as u32;
    let min = samples.iter().min().copied().unwrap_or_default();
    let max = samples.iter().max().copied().unwrap_or_default();
    println!(
        "{name} entries={entry_count} samples={} mean={mean:?} min={min:?} max={max:?}",
        samples.len()
    );
}
