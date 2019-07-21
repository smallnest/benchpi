use itertools::Itertools;
use rayon::prelude::*;
use std::thread::spawn;

#[macro_use]
extern crate may;

// https://en.wikipedia.org/wiki/Bailey–Borwein–Plouffe_formula
fn bbp(k: u32) -> f64 {
    let a1 = 4.0 / (8 * k + 1) as f64;
    let a2 = 2.0 / (8 * k + 4) as f64;
    let a3 = 1.0 / (8 * k + 5) as f64;
    let a4 = 1.0 / (8 * k + 6) as f64;

    (a1 - a2 - a3 - a4) / ((16 as f64).powi(k as i32))
}
pub fn pi(n: u32) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..n {
        result += bbp(i);
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
                result_in_child += bbp(i);
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
    (0..n).collect::<Vec<_>>().par_iter().map(|&i| bbp(i)).sum()
}

pub fn pi_with_may(n: u32, num: usize) -> f64 {
    let mut result: f64 = 0.0;

    may::config().set_workers(num).set_io_workers(0);

    let v = (0..n).map(|i| go!(move || { bbp(i) })).collect::<Vec<_>>();

    for i in v {
        result += i.join().unwrap();
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::*;
    const ITER_NUM: u32 = 10000;

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

    #[test]
    fn test_pi_with_may() {
        println!("pi_with_rayon = {}", pi_with_may(ITER_NUM, 4));
    }
}
