use proconio::*;

fn main() {
    input! {
        n : usize,
        l: i64,
        r: i64,
        mut p : [i64; n]
    }

    let mut from_start: Vec<i64> = vec![];
    from_start.push(0);

    for i in 0..n {
        if from_start[i] + p[i] < l * (i as i64 + 1) {
            from_start.push(from_start[i] + p[i]);
        } else {
            from_start.push(l * (i as i64 + 1));
        }
    }

    let mut from_end: Vec<i64> = vec![];
    from_end.push(0);

    for i in 0..n {
        if from_end[i] + p[n - i - 1] < r * (i as i64 + 1) {
            from_end.push(from_end[i] + p[n - i - 1]);
        } else {
            from_end.push(r * (i as i64 + 1));
        }
    }

    let mut ans: i64 = i64::max_value();
    for i in 0..=n {
        ans = std::cmp::min(from_start[i] + from_end[n - i], ans);
    }

    println!("{}", ans);
}
