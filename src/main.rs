fn main() {
    let start_path = std::env::args_os().nth(1).map(std::path::PathBuf::from);
    fileman::shell::run(start_path);
}
