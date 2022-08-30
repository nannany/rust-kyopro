use proconio::*;

fn main() {
    input! {
        n: usize,
        c: [[u64; n]; 1 << n]
    }

    let mut dp = vec![vec![0; 1 << n]; n + 1];
    let mut max = vec![vec![0; 1 << n]; n + 1];

    for i in 1..=n {
        for j in 0..1 << n {
            if (j / (1 << i - 1)) % 2 == 0 {
                // 右側と対戦
                dp[i][j] = max[i - 1][j] + c[j][i - 1]; //todo
            } else {
                // 左側と対戦
                dp[i][j] = max[i - 1][j] + c[j][i - 1]; //todo
            }

            // maxの更新
            if max[i][j] < max[i][j + 1] {
                max[i][j] = max[i][j + 1]; //todo
            }
        }
    }

    println!("{}", dp[n].iter().max().unwrap()))
}
