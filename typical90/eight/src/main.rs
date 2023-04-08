fn main() {
    proconio::input! {
        n: usize,
        s: String,
    }

    // dp
    let mut dp = vec![vec![0; n + 1]; 8];

    let atcoder: Vec<char> = "atcoder".chars().collect();

    for i in 1..n + 1 {
        for j in 1..8 {
            if s.chars().nth(i - 1).unwrap() == atcoder[j - 1] {
                if j == 1 {
                    dp[j][i] = dp[j][i - 1] + 1;
                } else {
                    dp[j][i] = dp[j - 1][i - 1] + dp[j][i - 1];
                }
            } else {
                dp[j][i] = dp[j][i - 1];
            }

            dp[j][i] %= 1_000_000_007;
        }
    }

    println!("{}", dp[7][n] % 1_000_000_007);
}
