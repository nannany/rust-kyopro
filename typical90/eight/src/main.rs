fn main() {
    proconio::input! {
        _n: usize,
        s: String,
    }

    // dp
    let mut dp = vec![0; 8]; // dp[0]はダミーで、dp[1]がaまでの通り数、dp[2]がtまでのとおりすう
    dp[0] = 1;

    let atcoder = "atcoder";

    for c in s.chars() {
        if let Some(x) = atcoder.find(c) { // xは位置
            dp[x + 1] = (dp[x] + dp[x + 1]) % 1_000_000_007;
        }
    }

    println!("{}", dp[7] % 1_000_000_007);
}
