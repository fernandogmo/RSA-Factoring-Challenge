// (c) Fernando Gonzalez-Morales, 2019
// That's right.
use std::env;
use std::fs;
use std::cmp::{max, min};
use rand::Rng;
use num::BigUint;
use num::cast::ToPrimitive;
use num::traits::identities::One;
//use primal;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();
    let nums: Vec<u128> = contents.lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();
    for n in nums {
        let d = pollard_rho(n);
        println!("{}={}*{}", n, n / d, d);
    }
}

fn pollard_rho(num: u128) -> u128 {
    if num == 1 { return num; }
    if num % 2 == 0 { return 2; }
    let k = rand::thread_rng().gen_range(2, num);
    let (mut d, mut x, mut y) = (1, k, k);
    let f = |i| (mod_pow(i, 2, num) + k + num) % num;
    while d == 1 {
        x = f(x);
        y = f(f(y));
        let diff = if y > x { y - x } else { x - y };
        d = stein_gcd(diff, num);
        if d == num {
            return 1;
        }
    }
    d
}

fn mod_pow(a: u128, b: u128, m: u128) -> u128 {
    let mut r = BigUint::one();
    let mut x = BigUint::from(a);
    let mut y = b;
    while y > 0 {
        if y & 1 == 1 { r = (r * &x) % m; }
        y = y >> 1;
        x = (&x * &x) % m;
    }
    use std::u128;
    let mask = BigUint::from(u128::MAX);
    (r & mask).to_u128().unwrap()
}

fn stein_gcd(m: u128, n: u128) -> u128 {
    match ((m, n), (m & 1, n & 1)) {
        ((x, y), _) if x == y               => y,
        ((0, x), _) | ((x, 0), _)           => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => stein_gcd(x >> 1, y),
        ((x, y), (0, 0))                    => stein_gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1))                    => { let (x, y) = (min(x, y), max(x, y));
                                                 stein_gcd((y - x) >> 1, x)
                                               }
        _                                   => unreachable!(),
    }
}

// SCRATCH PAD :D
/*  let biggest = *nums.iter().max().unwrap();
    let sieve = primal::Primes::new(biggest);
    for n in nums {
        for d in sieve {
            if n % d == 0 {
                println!("{}={}*{}", n, n / d, d);
                break;
}}}}

fn _pollard_rho(num: u128) -> u128 {
    if num == 1 { return num; }
    if num % 2 == 0 { return 2; }
    let mut x = rand::thread_rng().gen_range(2, num);
    let mut y = x;
    let c = rand::thread_rng().gen_range(1, num);
    let f = |i| (mod_pow(i, 2, num) + c + num) % num;
    let mut d = 1;
    while d == 1 {
        x = f(x);
        y = f(f(y));
        let diff = if x > y { x - y } else { y - x };
        d = stein_gcd(diff, num);
        if d == num {
            return 1;
        }
    }
    d
}
*/
