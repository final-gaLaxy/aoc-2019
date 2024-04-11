use std::{env, fs};



pub fn read_file(folder: &str, name: &str) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join(folder).join(format!("{name}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

pub mod template {
    use std::fmt::Write;
    use crate::read_file;

    pub fn run() -> String {
        let mut result: String = String::new();

        // Parse input
        let input: &str = &read_file("data", "template");

        writeln!(&mut result, "{}", input).unwrap();

        result
    }
}

pub mod day01 {
    use std::fmt::Write;

    pub fn run() -> String {
        let mut result: String = String::new();
        writeln!(&mut result, "World").unwrap();

        result
    }
}