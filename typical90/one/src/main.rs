use std::collections::HashSet;

// https://atcoder.jp/contests/typical90/tasks/typical90_a
fn main() {
    proconio::input! {
        n: usize,
        l: u32,
        k: usize,
        a: [u32; n],
    }

    let mut elements: Vec<u32> = a.windows(2).map(|pair| pair[1] - pair[0]).collect();
    elements.push(l - a[n - 1]);
    elements.insert(0, a[0]);

    let mut range = l / 2;
    let mut before_target = l;
    let mut target = range;
    let mut target_set: HashSet<u32> = HashSet::new();

    let mut max = *elements.iter().min().unwrap() as u32;
    loop {
        let mut count = 0;
        let mut bucket = 0;
        for ele in &elements {
            bucket += ele;

            if bucket >= target as u32 {
                bucket = 0;
                count += 1;
            }

            if count == k + 1 {
                break;
            }
        }

        range = before_target.abs_diff(target) / 2;
        if range == 0 {
            range += 1;
        }

        if count == k + 1 {
            max = target;
            before_target = target;
            target += range;
        } else {
            before_target = target;
            target -= range;
        }
        if target_set.contains(&target) {
            break;
        }
        target_set.insert(target);
    }

    println!("{:?}", max);
}
