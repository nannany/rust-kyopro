fn main() {
    proconio::input! {
        n : usize,
        mut a : [i64; n-1]
    }

    let mut dp: Vec<i64> = vec![0; n];

    for i in (0..n - 1).rev() {
        let ai = a[i];

        let mut numerator: i64 = 0;
        for j in 1..=ai {
            numerator += dp[i + j as usize]
        }
        numerator += ai + 1;
        dp[i] = numerator / ai;
    }

    println!("{}", dp[0]);
}

