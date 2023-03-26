
// https://atcoder.jp/contests/typical90/tasks/typical90_a
fn main() {
    proconio::input! {
        n: usize,
        l: u32,
        k: usize,
        a: [u32; n],
    }

    let mut elements: Vec<u32> = a.windows(2).map(|pair| pair[1] - pair[0]).collect();

    println!("elements: {:?}", elements);

    println!("Hello, world!");
}
