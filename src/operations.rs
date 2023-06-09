use num_integer::sqrt;
use rand::Rng;

pub fn is_perfect_square(i: i32) -> bool {
    return sqrt(i).pow(2) == i;
}

pub fn count_perfect_squares(v: &Vec<i32>) -> i32 {
    let mut count = 0;
    for (i, el1) in v.into_iter().enumerate() {
        if i < v.len() - 2 {
            let el2 = v[i + 1];
            if is_perfect_square(el1 + el2) {
                count += 1;
            }
        }
    }
    return count;
}

pub fn first_improvement_solution(solution: &mut Vec<i32>) -> i32 {
    let best_val = count_perfect_squares(solution);
    for i in 0..solution.len() - 1 {
        let n: usize = rand::thread_rng().gen_range(0..solution.len());
        solution.swap(i, n);
        let new_val = count_perfect_squares(solution);
        if new_val > best_val {
            return new_val;
        }
        solution.swap(n, i);
    }
    return best_val;
}

pub fn best_improvement_solution(solution: &mut Vec<i32>, limit: usize) -> i32 {
    let mut best_val = count_perfect_squares(solution);
    let mut best_solution: Vec<i32> = Vec::new();
    for _ in 0..limit {
        let n1: usize = rand::thread_rng().gen_range(0..solution.len());
        let n2: usize = rand::thread_rng().gen_range(0..solution.len());
        solution.swap(n1, n2);
        let new_val = count_perfect_squares(solution);
        if new_val > best_val {
            best_solution = solution.clone();
            best_val = new_val;
        }
        solution.swap(n2, n1);
    }
    *solution = best_solution;
    return best_val;
}
