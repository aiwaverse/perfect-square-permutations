use std::collections::VecDeque;

use crate::operations;

pub fn lahc(solution: &mut Vec<i32>, max_size: usize) -> i32 {
    let mut s = operations::count_perfect_squares(solution);
    let mut s_star = s;
    let mut f = VecDeque::from(vec![s; max_size]);
    let mut i = 0;
    while i < 1000 {
        if i % 100 == 0 {
            println!("Iteration {}", i);
        }
        let s_linha = operations::first_improvement_solution(solution);
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
    return s_star;
}
