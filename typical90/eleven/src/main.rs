fn main() {
    proconio::input! {
        n: usize,
        dcs: [(usize, usize, i64); n],
    }

    let mut dcs = dcs.clone();
    // dcsをdでソート
    dcs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut answer = 0;
    // bit全探索
    for i in 0..1 << n {
        let money = hantei(i, dcs);

        answer = answer.max(money);
    }
}

fn hantei(mask: i32, dcs: ) -> i32 {
    let mut moneyCounter = 0;
    let mut time = 0;

    for i in 0..n {
        if
        if mask >> i & 1 == 1 {
            moneyCounter += dcs[i].1;
            time += dcs[i].0;
        }
    }

    return moneyCounter;
}


