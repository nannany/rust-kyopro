use std::iter::repeat;
use itertools::*;

fn main() {
    proconio::input! {
        n: usize,
        b: usize,
        k: usize,
        c: [usize; k]
    }

    let mut answer = 0;

    // cの重複配列を取得
    let repeated_permutations = repeat(c.clone()).take(n).multi_cartesian_product();

    for combination in repeated_permutations {
        let mut sum = 0;
        for i in 0..n {
            sum += combination[i] * exponentiation_by_squaring_with_modulo(10, n - i - 1, 1000000007);
        }
        if sum % b == 0 {
            answer += 1;
        }
    }

    println!("{}", answer);
}

/// Returns the value of x^n mod m.
fn exponentiation_by_squaring_with_modulo(x: usize, n: usize, m: usize) -> usize {
    let mut x = x;
    let mut n = n;
    let mut result = 1;

    while n > 0 {
        if n % 2 == 1 {
            result = result * x % m;
        }
        x = x * x % m;
        n /= 2;
    }

    result
}
