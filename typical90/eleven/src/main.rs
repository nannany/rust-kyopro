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

    if dcs[0].0 >= dcs[0].1 {
        dp[0][dcs[0].1] = dcs[0].2;
    }

}


