fn main() {
    let start_path = std::env::args_os().nth(1).map(std::path::PathBuf::from);
    fileman::gpui_shell::run(start_path);
}
