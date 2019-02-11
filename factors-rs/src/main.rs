use std::env;
use std::fs;
use std::cmp::{max, min};
use rand::Rng;
use num_integer::Roots;
use primal;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();
    let nums: Vec<u128> = contents.lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();
    for n in nums {
        let d = if (n % 2) == 0 { 2 }
           else if (n % 3) == 0 { 3 }
           else if (n % 5) == 0 { 5 }
           else if (n % 7) == 0 { 7 }
           else {pollard_rho(n)};
        println!("{}={}*{}", n, n / d, d);
    }
}
/*    let biggest:  = *nums.iter().max().unwrap();
    let sieve = primal::Primes::new(biggest.sqrt());
    for n in nums {
        for d in sieve {
            if n % d == 0 {
                println!("{}={}*{}", n, n / d, d);
                break;
            }
        }
    }
}*/

fn _pollard_rho(num: u128) -> u128 {
    if num == 1 {return num;}
    if num % 2 == 0 {return 2;}
    let mut x = rand::thread_rng().gen_range(2, num);
    let mut y = x;
    let c = rand::thread_rng().gen_range(1, num);
    let f = |i| (_mod_pow(i, 2, num) + c + num) % num;
    let mut d = 1;
    while d == 1 {
        x = f(x);
        y = f(f(y));
        let diff = if x > y { x - y } else { y - x };
        d = stein_gcd(diff, num);
        if d == num {
            break;
        }
    }
    d
}

fn pollard_rho(num: u128) -> u128 {
    let k = rand::thread_rng().gen_range(2, num);
    let (mut d, mut n, mut p) = (1, k, k);
    let f = |x| (x * x + k) % num;
    while d == 1 {
        n = f(n);
        p = f(f(p));
        let diff = if p > n { p - n } else { n - p};
        d = stein_gcd(diff, num);
        if d == num { break; }
    }
    d
}

fn _mod_pow(a: u128, b: u128, m: u128) -> u128 {
    let mut r = 1;
    while b > 0 {
        if b & 1 == 1 { r = (r * a) % m; }
        let mut b = b >> 1;
        let mut a = (a * a) % m;
    }
    r
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
