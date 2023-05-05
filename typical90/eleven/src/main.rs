fn main() {
    proconio::input! {
        n: usize,
        dcs: [(usize, usize, usize); n],
    }

    let mut dcs = dcs.clone();

    // dcsをdでソート
    dcs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut answer = 0;
    // bit全探索
    for i in 0..1 << n {
        let money = hantei(i, &dcs, n);

        answer = answer.max(money);
    }

    println!("{}", answer);
}

fn hantei(mask: i32, dcs: &Vec<(usize, usize, usize)>, n:usize) -> usize {
    let mut money_counter = 0;
    let mut time = 0;

    for i in 0..n {
        if mask >> i & 1 == 1 {
            money_counter += dcs[i].2;
            time += dcs[i].1;
        }
        if time > dcs[i].0 {
            return 0;
        }
    }

    return money_counter;
}


