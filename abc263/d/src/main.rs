use proconio::*;

fn main() {
    input! {
        n : usize,
        l: i32,
        r: i32,
        mut p : [i32; n]
    }

    let mut from_start: Vec<i32> = vec![];
    from_start.push(0);

    for i in 0..n {
        if from_start[i] + p[i] < l * (i as i32 + 1) {
            from_start.push(from_start[i] + p[i]);
        } else {
            from_start.push(l * (i as i32 + 1));
        }
    }

    let mut from_end: Vec<i32> = vec![];
    from_end.push(0);

    for i in 0..n {
        if from_end[i] + p[n - i - 1] < r * ((i as i32) + 1) {
            from_end.push(from_end[i] + p[n - i - 1]);
        } else {
            from_end.push(r * ((i as i32) + 1));
        }
    }

    let mut ans: i32 = i32::max_value();
    for i in 0..=n {
        ans = std::cmp::min(from_start[i] + from_end[n - i], ans);
    }

    println!("{}", ans);
}
