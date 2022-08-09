use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        n : usize,
        mut p : [Usize1; n-1]
    }
    p.insert(0, 0);
    let mut ans = 0;
    let mut n = n - 1;

    while n > 0 {
        ans += 1;
        n = p[n];
    }

    println!("{}", ans);
}
