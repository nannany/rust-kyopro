fn main() {
    proconio::input! {
        n : usize,
        mut a : [i64; n-1]
    }

    let mut dp: Vec<i64> = vec![0; n];
    let mut sum: Vec<i64> = vec![0; n + 1];

    for i in (0..n - 1).rev() {
        let ai = a[i];

        let mut numerator: i64;
        numerator = sum[i + 1] - sum[i + ai as usize + 1];

        numerator += ai + 1;
        dp[i] = numerator * (pow(ai, 998244351)) % 998244353;
        sum[i] = sum[i + 1] + dp[i];
    }

    println!("{}", dp[0]);
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

