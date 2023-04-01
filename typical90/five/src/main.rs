use std::iter::repeat;
use itertools::*;

fn main() {
    proconio::input! {
        n: usize,
        b: usize,
        k: usize,
        c: [usize; k]
    }

    // dp := dp[i][j] := 上i桁目まで見たときに、bで割った余りがjになるような数の個数
    let mut dp: Vec<Vec<usize>> = vec![vec![0; b]; n + 1];

    for i in 0..n {
        for j in 0..b {
            for &c in &c {
                if i == 0 && j == 0 {
                    dp[i + 1][c % b] += 1;
                    continue;
                }

                let next = (j * 10 + c) % b;
                dp[i + 1][next] += dp[i][j] % 1000000007;
            }
        }
    }

    println!("{}", dp[n][0] % 1000000007);
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
