pub fn read_file(file_path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(file_path)
}