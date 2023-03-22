use std::time::Instant;

pub mod files;
pub mod lahc;
pub mod operations;
fn main() {
    let files = files::all_instances(".");
    let mut instances: Vec<(&String, Vec<i32>)> = files
        .iter()
        .map(|f| (f, files::generate_solution(f)))
        .collect();
    instances.sort_by_key(|e| e.1.len());
    for (name, sol) in instances.iter() {
        println!("{}", name);
        let mut s = sol.clone();
        let now = Instant::now();
        let x = lahc::lahc(&mut s, 100);
        println!("Best solution found is {}.", x);
        let elapsed_time = now.elapsed();
        println!("Took {} seconds.", elapsed_time.as_secs());
    }
}
