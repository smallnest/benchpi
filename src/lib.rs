use itertools::Itertools;
use rayon::prelude::*;
use std::thread::spawn;

pub fn pi(n: u32) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..n {
        let k = (2 * i + 1) as f64;
        let sub_result = 4.0 / k;
        result = if i % 2 == 0 {
            result + sub_result
        } else {
            result - sub_result
        };
    }
    result
}

pub fn pi_with_thread(n: u32, num: usize) -> f64 {
    let mut result: f64 = 0.0;

    let mut thread_handlers = vec![];

    for chunk in &(0..n).chunks(num) {
        let worklist = chunk.collect::<Vec<_>>();

        thread_handlers.push(spawn(move || {
            let mut result_in_child: f64 = 0.0;
            for i in worklist {
                let k = (2 * i + 1) as f64;
                let sub_result = 4.0 / k;
                result_in_child = if i % 2 == 0 {
                    result_in_child + sub_result
                } else {
                    result_in_child - sub_result
                };
            }
            result_in_child
        }));
    }

    for handle in thread_handlers {
        result += handle.join().unwrap();
    }

    result
}

pub fn pi_with_rayon(n: u32) -> f64 {
    (0..n)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|&i| {
            let k = (2 * i + 1) as f64;
            if i % 2 == 0 {
                4.0 / k
            } else {
                -4.0 / k
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;
    const ITER_NUM: u32 = 1000000;

    #[test]
    fn test_pi() {
        println!("pi = {}", pi(ITER_NUM));
    }

    #[test]
    fn test_pi_with_thread() {
        println!("pi_with_thread = {}", pi_with_thread(ITER_NUM, 4));
    }

    #[test]
    fn test_pi_with_rayon() {
        println!("pi_with_rayon = {}", pi_with_rayon(ITER_NUM));
    }
}
