fn main() {
    proconio::input! {
        n: usize,
        cp: [(i64, i64); n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut cpn = cp.clone();
    cpn.insert(0, (0, 0));

    let mut c1 = vec![0; n + 1];
    let mut c2 = vec![0; n + 1];


    for i in 1..n + 1 {
        if cpn[i].0 == 1 {
            // 1組の話
            c1[i] = c1[i - 1] + cpn[i].1;
            c2[i] = c2[i - 1];
        } else {
            c1[i] = c1[i - 1];
            c2[i] = c2[i - 1] + cpn[i].1;
        }
    }

    for (l, r) in &lr {
        println!("{} {}", c1[*r] - c1[*l - 1], c2[*r] - c2[*l - 1]);
    }
}
