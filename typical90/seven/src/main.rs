use std::cmp::min;

fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n],
        q: usize,
        b: [i32; q]
    }

    let mut a = a;
    a.sort();

    for ele in b {
        println!("{}", find_nearest_by_binary_search(&a, ele));
    }
}


fn find_nearest_by_binary_search(a: &[i32], x: i32) -> i32 {
    let mut ng = 0;
    let mut ok = a.len() - 1;

    while ((ok - ng) as i32).abs() > 1 {
        let mid = (ok + ng) / 2;
        if a[mid] <= x {
            ng = mid;
        } else {
            ok = mid;
        }
    }

    return min((a[ng] - x).abs(), (a[ok] - x).abs());
}
