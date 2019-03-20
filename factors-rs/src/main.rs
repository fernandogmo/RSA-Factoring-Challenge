// (C) 2019 Fernando Gonzalez-Morales
// That's right.
use std::{env, fs};
use ramp::Int;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use smallvec::SmallVec;


fn main() {
    let outputs = Arc::new(Mutex::new(SmallVec::<[String; 10]>::new()));
    let filename = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let numbers: Vec<Int> = contents.lines()
        .filter_map(|line| line.parse().ok())
        .collect();
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
