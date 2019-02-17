// (C) 2019 Fernando Gonzalez-Morales
// That's right.
use std::{env, fs};
use ramp::Int;
use rayon::prelude::*;

/* Look up factors for nums up to 2^16 = 65536
 * in a collection of primes less than 2^8 = 256
 * https://primes.utm.edu/nthprime/index.php#piofx */
static PRIMES: [i32; 54] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53,
                            59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113,
                            127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
                            191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251];

fn main() {
    let filename = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let nums: Vec<Int> = contents.lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();
    nums.par_iter().for_each( |n| {
        let d = if *n < 65536 { look_up(n) } else
                { pollard_brent(n, Int::from(2)) };
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

/* Ref: http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.117.1230&rep=rep1&type=pdf
 */
fn pollard_brent(num: &Int, seed: Int) -> Int {
    let (mut d, mut x, mut y) = (Int::one(), Int::from(2), Int::from(2));
    let f = |i: &Int| (i.pow_mod(&Int::from(2), &num) + &seed + num) % num;
    while d == 1 {
        x = f(&x);
        y = f(&f(&y));
        let diff: Int = if y > x { &y - &x } else { &x - &y };
        d = diff.gcd(num);
        if d == *num {
            return pollard_brent(num, &seed + 1);
        }
    }
    d
}
