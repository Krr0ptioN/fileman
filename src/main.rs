fn main() {
    let start_path = std::env::args_os().nth(1).map(std::path::PathBuf::from);
    stiff::shell::run(start_path);
}
