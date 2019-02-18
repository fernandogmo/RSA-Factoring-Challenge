// (C) 2019 Fernando Gonzalez-Morales
// That's right.
use std::{env, fs};
//use std::cmp::{max, min};
//use std::iter::TakeWhile;
//use rand::Rng;
use ramp::Int;
//use num::BigUint;
//use num::cast::ToPrimitive;
//use num::traits::identities::One;
use rayon::prelude::*;
//use primal::Primes;

/* Look up factors for nums up to 2^16 = 65536
 * in a collection of primes less than 2^8 = 256
 * https://primes.utm.edu/nthprime/index.php#piofx */
static _PRIMES: [i32; 54] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53,
                            59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113,
                            127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
                            191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251];

fn main() {
    let filename = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let nums: Vec<Int> = contents.lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();
    //for n in nums { println!("{}", n); }
    //let edge_case = 239809320265259;
    nums.par_iter().for_each( |n| {
        let d = if *n < 65536 { _look_up(n) } else
                { pollard_brent(n, Int::from(2)) };
        println!("{}={}*{}", n, n / &d, &d);
    });
}
/* https://stackoverflow.com/a/2274520/9221785 */
fn _look_up(num: &Int) -> Int {
    for p in _PRIMES.iter() {
        let p = Int::from(*p);
        if num % &p == 0 {
            return p;
        }
    }
    Int::one()
}

/* Ref: http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.117.1230&rep=rep1&type=pdf
 */
fn pollard_brent(num: &Int, seed: Int) -> Int {
    //let k = rand::thread_rng().gen_range(2, num);
    //let k = Int::from(2);
    //let (mut d, x, y) = (Int::one(), &k, &k);
    let (mut d, mut x, mut y) = (Int::one(), Int::from(2), Int::from(2));
    //let f = |i| (mod_pow(i, 2, num) + k + num) % num;
    let f = |i: &Int| (i.pow_mod(&Int::from(2), &num) + &seed + num) % num;
    while d == 1 {
        x = f(&x);
        y = f(&f(&y));
        let diff: Int = if y > x { &y - &x } else { &x - &y };
        //d = stein_gcd(diff, num);
        d = diff.gcd(num);
        if d == *num {
            return pollard_brent(num, &seed + 1);
        }
    }
    d
}
/*
fn mod_pow(a: Int, b: Int, m: Int) -> Int {
    let mut r = Int::one();
    let mut a = Int::from(a);
    let mut b = b;
    while b > 0 {
        if b & 1 == 1 { r = (r * &a) % m; }
        b = b >> 1;
        a = (&a * &a) % m;
    }
    r
    /*
    use std::u128;
    let mask = BigUint::from(u128::MAX);
    (r & mask).to_u128().unwrap()
    */
}
fn _stein_gcd(m: Int, n: Int) -> Int {
    match ((m, n), (m & 1, n & 1)) {
        ((x, y), _) if x == y               => y,
        ((0, x), _) | ((x, 0), _)           => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => _stein_gcd(x >> 1, y),
        ((x, y), (0, 0))                    => _stein_gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1))                    => { let (x, y) = (min(x, y), max(x, y));
                                                 _stein_gcd((y - x) >> 1, x)
                                               }
        _                                   => unreachable!(),
    }
}
*/

//-----------------------------------------------------------

// SCRATCH PAD :D
/*  let biggest = *nums.iter().max().unwrap();
    let sieve = primal::Primes::new(biggest);
    for n in nums {
        for d in sieve {
            if n % d == 0 {
                println!("{}={}*{}", n, n / d, d);
                break;
}}}}
/*fn _pollard_rho(num: u128) -> u128 {
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29].iter() {
        if num % p == 0 { return *p; }
    }
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
}*/

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
