use std::collections::VecDeque;

use crate::operations;
use std::time::Instant;

pub fn lahc(solution: &mut Vec<i32>, max_size: usize) -> i32 {
    let mut s = operations::count_perfect_squares(solution);
    let mut s_star = s;
    let mut f = VecDeque::from(vec![s; max_size]);
    let mut i = 0;
    let mut i_idle = 0;
    let instant = Instant::now();
    while i_idle < 100 && instant.elapsed().as_secs_f64() < (10.0 * 60.0) {
        let s_linha = operations::first_improvement_solution(solution);
        if s_linha <= s {
            i_idle += 1;
        } else {
            i_idle = 0;
        }
        if s_linha > f[0] || s_linha > s {
            s = s_linha;
        }
        if s > f[0] {
            f.pop_front();
            f.push_back(s)
        }
        if s > s_star {
            s_star = s;
        }
        i += 1;
    }
    println!(
        "Finished at {} iterations and {:?} seconds",
        i,
        instant.elapsed().as_secs_f64()
    );
    return s_star;
}
