fn main() {
    proconio::input! {
        n : usize,
        mut a : [i64; n-1]
    }

    let mut dp: Vec<i64> = vec![0; n];
    let mut sum: Vec<i64> = vec![0; n];

    for i in (0..n - 1).rev() {
        let ai = a[i];

        let mut numerator: i64 = 0;
        for j in 1..=ai {
            numerator += dp[i + j as usize];
        }
        numerator += ai + 1;
        dp[i] = numerator * (pow(ai, 998244351)) % 998244353;
    }

    println!("{}", dp[0] % 998244353);
}

pub fn pow(mut x: i64, mut n: i64) -> i64 {
    let mut t: i64 = 1;
    while n > 0 {
        if n & 1 == 1 {
            t = t * x % 998244353;
        }
        x = x * x % 998244353;
        n >>= 1;
    }
    t
}

