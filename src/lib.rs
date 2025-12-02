use std::{fs::read_to_string, path::Path};

pub fn read_input(file: &str) -> String {
    let day = Path::new(file).file_prefix().unwrap().to_str().unwrap();
    let input_file = format!("./inputs/{day}");
    read_to_string(&input_file).unwrap_or_else(|_| panic!("couldn't read file {input_file}"))
}
