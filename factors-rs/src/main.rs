// (C) 2019 Fernando Gonzalez-Morales
// That's right.
#![allow(irrefutable_let_patterns)]
use std::{env, fs};
use ramp::Int;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use smallvec::SmallVec;


fn main() {
    let outputs = Arc::new(Mutex::new(SmallVec::<[String; 10]>::new()));
    let _primes: Vec<Int> = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
                            31, 37, 41, 43, 47, 53, 59, 61, 67,
                            71, 73, 79, 83, 89, 97].into_iter()
                                                   .map(|x| Int::from(*x))
                                                   .collect();
    let filename = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let numbers: Vec<Int> = contents.par_lines()
                            .filter_map(|line| line.parse().ok())
                            .collect();
    /*let (small, large): (Vec<Int>, Vec<Int>) = numbers.into_par_iter()
                                                  .partition(|n| *n < 10000);*/
    /*small.par_iter().for_each(|n| {
        let (p, q) = look_up(n, &primes);
        println!("{}={}*{}", n, p, q);
    });*/
    numbers.par_iter().for_each(|n| {
        let (p, q) = pollard_brent(n);
        let mut v = outputs.lock().unwrap();
        if v.len() == v.inline_size() {
            for s in v.drain() {
                println!("{}", s);
            }
        }
        v.push(format!("{}={}*{}", n, p, q));
    });
    let mut v = outputs.lock().unwrap();
    if !v.is_empty() {
        for s in v.drain() {
            println!("{}", s);
        }
    }
}

// https://stackoverflow.com/a/2274520/9221785
fn _look_up(n: &Int, ps: &Vec<Int>) -> (Int, Int) {
    for p in ps.iter() {
        if let (q, r) = n.divmod(p) {
            if r == 0 { return (p.clone(), q) };
        }
    }
    (Int::one(), n.clone())
}

// Refs:
// +http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.117.1230&rep=rep1&type=pdf
// +https://comeoncodeon.wordpress.com/2010/09/18/pollard-rho-brent-integer-factorization/
fn pollard_brent(num: &Int) -> (Int, Int) {
    let (mut x, mut y) = (Int::from(2), Int::from(2));
    let (mut d, mut q) = (Int::one(), Int::one());
    let (mut ys, mut r) = (Int::zero(), 1);
    const M: i32 = 71;
    let f = |i: &Int| (i.pow_mod(&Int::from(2), &num) + M) % num;
    while d == 1 {
        x = f(&y);
        for _ in 0..r {
            y = f(&y);
        }
        let mut k = 0_i32;
        while k < r && d == 1 {
            for _ in 0..(M.min(r - k)) {
                y = f(&y);
                q = q * (&x - &y).abs() % num;
            }
            ys = y.clone();
            d = q.gcd(num);
            k += M;
        }
        r *= 2;
    }
    if d == *num {
        loop {
            ys = f(&ys);
            d = ((&x - &ys).abs()).gcd(num);
            if d > 1 {
                break;
            }
        }
    }
    (d.clone(), num/d)
}
