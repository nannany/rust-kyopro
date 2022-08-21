// ac
fn main() {
    proconio::input! {
        n : usize,
        l: i64,
        r: i64,
        mut p : [i64; n]
    }

    let mut dp: [i64; 3] = [0; 3];

    for p in p {
        let mut next = [i64::max_value(); 3];
        next[0] = dp[0] + l;
        next[1] = dp[0].min(dp[1]) + p;
        next[2] = dp[0].min(dp[1].min(dp[2])) + r;
        dp = next;
    }

    println!("{}", dp.iter().min().unwrap())
}
