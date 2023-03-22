use criterion::{black_box, criterion_group, criterion_main, Criterion};
use perfect_square_permutations::operations;
use rand::{distributions::Uniform, Rng};
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    let range = Uniform::from(0..30000);
    let values: Vec<i32> = rand::thread_rng().sample_iter(&range).take(10000).collect();
    c.bench_function("count perfect squares", |b| {
        b.iter(|| operations::count_perfect_squares(black_box(&values)))
    });
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
    c.bench_function("first improvement solution", |b| {
        b.iter_batched_ref(
            || file_values.clone(),
            |mut sol| operations::first_improvement_solution(&mut sol),
            criterion::BatchSize::SmallInput,
        );
    });
    // c.bench_function("best improvement solution, limit is 9999", |b| {
    //     b.iter_batched_ref(
    //         || file_values.clone(),
    //         |mut sol| operations::best_improvement_solution(&mut sol, 9999),
    //         criterion::BatchSize::SmallInput,
    //     );
    // });
    c.bench_function("best improvement solution, limit is 1000", |b| {
        b.iter_batched_ref(
            || file_values.clone(),
            |mut sol| operations::best_improvement_solution(&mut sol, 1000),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
