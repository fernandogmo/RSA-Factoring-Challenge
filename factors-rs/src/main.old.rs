// (C) 2019 Fernando Gonzalez-Morales
// That's right.
use std::{env, fs};
use ramp::Int;
use rayon::prelude::*;


fn main() {
    let filename = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let numbers: Vec<Int> = contents.par_lines()
                            .filter_map(|line| line.parse().ok())
                            .collect();
    //let (small, large): (Vec<_>, Vec<_>) = (&numbers).into_par_iter().partition(|n| n < &&1_205_000);
    numbers.par_iter().for_each(|n| {
        let d = pollard_brent(n, Int::from(2));
        println!("{}={}*{}", n, n / &d, &d);
    });
}
/* https://stackoverflow.com/a/2274520/9221785 */
fn look_up(num: &Int) -> Int {
    for p in PRIMES.iter() {
        let p = Int::from(*p);
        if num % &p == 0 {
            return p;
        }
    }
    Int::one()
}

/* Refs:
 * +http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.117.1230&rep=rep1&type=pdf
 * +https://comeoncodeon.wordpress.com/2010/09/18/pollard-rho-brent-integer-factorization/
 */
fn pollard_brent(num: &Int, seed: Int) -> Int {
    let f = |i: &Int| (i.pow_mod(&Int::from(2), &num) + &seed) % num;
    let (mut x, mut y, m) = (Int::from(2), Int::from(2), 2);
    let mut ys = Int::zero();
    let (mut d, mut q, mut r) = (Int::one(), Int::one(), 1);
    while d == 1 {
        x = f(&y);
        for _ in 0..r { y = f(&y); }
        let mut k = 0;
        while k < r && d == 1 {
            for _ in 0..(m.min(r - k)) {
                y = f(&y);
                q = q * (&x - &y).abs() % num;
            }
            ys = y.clone();
            d = q.gcd(num);
            //d = binary_gcd(&q, num);
            k += m;
        }
        r *= 2;
    }
    if d == *num {
        loop {
            ys = f(&ys);
            d = ((&x - &ys).abs()).gcd(num);
            //d = binary_gcd(&(&x - &ys).abs(), num);
            if d > 1 {
                break;
            }
        }
    }
    d
}
/*
fn binary_gcd(a: &Int, b: &Int) -> Int {
    if a == b { return a.clone(); };
    /*
     let (a, b) = if m >= n {
            (m.clone(), n.clone())
        } else {
            (n.clone(), m.clone())
        };
    */
	if a.is_zero() { return b.clone();}
	if b.is_zero() { return a.clone(); }
    if a.is_even() { // look for factors of 2
        if !(b.is_even()) { // b is odd, a is even
            return binary_gcd(&(a >> 1), b);
        }
        else { // both are even
            return binary_gcd(&(a >> 1), &(b >> 1)) << 1;
        }
    }
    if b.is_even() { // a is odd, b is even
        return binary_gcd(a, &(b >> 1));
    }
    // reduce larger argument
    if a > b { return binary_gcd(&((a - b) >> 1), b); }

    return binary_gcd(&((b - a) >> 1), a);
}

fn _stein_gcd(m: Int, n: Int) -> Int {
    match ((m, n), (m & 1, n & 1)) {
        ((x, y), _) if x == y               => y,
        ((0, x), _) | ((x, 0), _)           => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => _stein_gcd(x >> 1, y),
        ((x, y), (0, 0))                    => _stein_gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1))                    => { let (x, y) = (x.min(y), x.max(y));
                                                 _stein_gcd((y - x) >> 1, x)
                                               }
        _                                   => unreachable!(),
    }
}
/* Look up factors for nums up to 2^16 = 65536
 * in a collection of primes less than 2^8 = 256
 * https://primes.utm.edu/nthprime/index.php#piofx */
static PRIMES: [i32; 184] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53,
                            59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113,
                            127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
                            191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251,
                            257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317,
                            331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
                            401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463,
                            467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557,
                            563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619,
                            631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
                            709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787,
                            797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863,
                            877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953,
                            967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031,
                            1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097];
*/
