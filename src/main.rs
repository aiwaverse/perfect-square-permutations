use std::fs;

use perfect_square_permutations::operations;

fn main() {
    println!("Hello, world!");
    let content = fs::read_to_string(
        "/home/phi/Documents/codes/rust/perfect-square-permutations/instances/instance_10000.dat",
    )
    .unwrap();
    let file_values: Vec<i32> = content
        .lines()
        .nth(1)
        .expect("Not two lines")
        .trim()
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", file_values)
}
