#![feature(test)]

extern crate test;

use benchpi::*;
use num_cpus;
use test::Bencher;

const ITER_NUM: u32 = 10000;

#[bench]
fn bench_single_thread(bench: &mut Bencher) {
    bench.iter(|| pi(ITER_NUM));
}

#[bench]
fn bench_multiple_thread(bench: &mut Bencher) {
    let num = num_cpus::get();

    bench.iter(|| pi_with_thread(ITER_NUM, num));
}

#[bench]
fn bench_with_rayon(bench: &mut Bencher) {
    bench.iter(|| pi_with_rayon(ITER_NUM));
}
