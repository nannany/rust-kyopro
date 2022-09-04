use proconio::*;

// ac
fn main() {
    input! {
        n: usize,
        c: [[u64; n]; 1 << n]
    }

    let mut dp = vec![vec![0; 1 << n]; n + 1];
    let mut max = vec![vec![0; 1 << n]; n + 1];

    for i in 1..=n { // i回戦目を表す
        for j in 0..(1 << n) { // j番目のノードを表す
            let unit = 1 << (i - 1);
            if (j / unit) % 2 == 0 { // 右側と対戦
                let counter = unit * ((j / unit) + 1);
                dp[i][j] = max[i - 1][counter] + dp[i - 1][j];
            } else { // 左側と対戦
                let counter = unit * ((j / unit) - 1);
                dp[i][j] = max[i - 1][counter] + dp[i - 1][j];
            }

            // maxの更新
            let belong_unit = 1 << i;
            let belong = belong_unit * (j / belong_unit);
            if max[i][belong] < dp[i][j] + c[j][i - 1] {
                max[i][belong] = dp[i][j] + c[j][i - 1];
            }
        }
    }

    println!("{}", max[n].iter().max().unwrap());
}
