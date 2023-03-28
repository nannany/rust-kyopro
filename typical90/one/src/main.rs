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
    let mut target = range;
    let mut target_set :HashSet<u32>= HashSet::new();

    let mut max = *elements.iter().min().unwrap() as u32;
    while range >= 1 {
        println!("target: {:?}", target);
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

        if range == 2 {
            range /= 2
        } else {
            range = range / 2 + 1;
        }

        if count == k + 1 {
            max = target;
            target += range;
        } else {
            target -= range;
        }
        if target_set.contains(&target) {
            break;
        }
        target_set.insert(target);

    }

    println!("max: {:?}", max);
}
