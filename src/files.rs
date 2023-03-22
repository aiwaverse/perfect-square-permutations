use glob::glob;
use std::fs;

pub fn generate_solution(path: &str) -> Vec<i32> {
    let content = fs::read_to_string(path).unwrap();
    let file_values: Vec<i32> = content
        .lines()
        .nth(1)
        .expect("Not two lines")
        .trim()
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    return file_values;
}

pub fn all_instances(root: &str) -> Vec<String> {
    let mut files = Vec::new();
    for file in glob(format!("{root}/instances/instance*.dat").as_str()).unwrap() {
        match file {
            Ok(path) => files.push(path.display().to_string()),
            Err(e) => panic!("{e}"),
        }
    }
    return files;
}
