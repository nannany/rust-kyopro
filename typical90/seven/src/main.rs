fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
        q: usize,
        b: [usize; q]
    }

    let mut a = a;
    a.sort();

    println!("{:?}", b);
}


fn find_nearest_by_binary_search(a: &[usize], x: usize) -> usize {
    let mut ng = 0;
    let mut ok = a.len();

    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if a[mid] <= x {
            ng = mid;
        } else {
            ok = mid;
        }
    }

    if (a[ng] - x).abs() < (a[ok] - x).abs() {
        a[ng]
    } else {
        a[ok]
    }
}
