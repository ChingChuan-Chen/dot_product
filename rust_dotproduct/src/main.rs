extern crate time;
extern crate rayon;
extern crate packed_simd;
use packed_simd::f64x4;
use packed_simd::f64x8;
use rayon::prelude::*;
use std::vec::Vec;
use time::PreciseTime;

fn main() {
    let n = 100000000;
    let x: Vec<f64> = vec![0.2; n];
    let y: Vec<f64> = vec![0.1; n];

    println!("Rust:");
    let start = PreciseTime::now();
    let res: f64 = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum();
    let end = PreciseTime::now();
    println!("res: {}", res);
    println!("printf: {} seconds", start.to(end));

    println!("Rust: (SIMD f64x4)");
    let start = PreciseTime::now();
    let res: f64 = x
        .chunks_exact(4)
        .map(f64x4::from_slice_unaligned)
        .zip(y.chunks_exact(4).map(f64x4::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<f64x4>()
        .sum();
    let end = PreciseTime::now();
    println!("res: {}", res);
    println!("printf: {} seconds", start.to(end));

    println!("Rust: (SIMD f64x8)");
    let start = PreciseTime::now();
    let res: f64 = x
        .chunks_exact(8)
        .map(f64x8::from_slice_unaligned)
        .zip(y.chunks_exact(8).map(f64x8::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<f64x8>()
        .sum();
    let end = PreciseTime::now();
    println!("res: {}", res);
    println!("printf: {} seconds", start.to(end));

    println!("Rust: (parallism)");
    let start = PreciseTime::now();
    let res: f64 = x.par_iter().zip(y.par_iter()).map(|(a, b)| a * b).sum();
    let end = PreciseTime::now();
    println!("res: {}", res);
    println!("printf: {} seconds", start.to(end));

    println!("Rust: (parallism + SIMD)");
    let start = PreciseTime::now();
    let res: f64 = x
        .par_chunks(8)
        .map(f64x8::from_slice_unaligned)
        .zip(y.par_chunks(8).map(f64x8::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<f64x8>()
        .sum();
    let end = PreciseTime::now();
    println!("res: {}", res);
    println!("printf: {} seconds", start.to(end));
}
