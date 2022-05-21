pub fn read_file(filepath: &str) -> String {
    let file = std::fs::read_to_string(filepath).expect("failed to read file dipshit");
    file
}