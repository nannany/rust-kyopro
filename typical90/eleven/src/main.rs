fn main() {
    proconio::input! {
        n: usize,
        dcs: [(usize, usize, usize); n],
    }

    let mut dcs = dcs.clone();

    // dcsをdでソート
    dcs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // dpでやる
    let mut dp = vec![[0; 5001]; n];

    let d_max = dcs[n - 1].0;

    if dcs[0].0 >= dcs[0].1 {
        dp[0][dcs[0].1] = dcs[0].2;
    }

    for i in 1..n {
        let (d, c, s) = dcs[i];
        for j in 0..=d_max {
            if j == 0 {
                if d >= c {
                    dp[i][c] = s;
                }
            } else {
                if dp[i - 1][j] == 0 {
                    continue;
                } else {
                    if j + c > d {
                        dp[i][j] = dp[i - 1][j].max(dp[i][j]);
                    } else {
                        dp[i][j] = dp[i - 1][j].max(dp[i][j]);
                        dp[i][j + c] = (dp[i - 1][j] + s).max(dp[i][j + c]);
                    }
                }
            }
        }
    }


    let answer = dp[n - 1].iter().max().unwrap();

    println!("{}", answer);
}


