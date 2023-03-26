// https://atcoder.jp/contests/typical90/tasks/typical90_a
fn main() {
    proconio::input! {
        n: usize,
        l: u32,
        k: usize,
        a: [u32; n],
    }

    let mut elements: Vec<u32> = a.windows(2).map(|pair| pair[1] - pair[0]).collect();

    let mut target = n/2;

    let mut max = elements.iter().min();
    while target < 1 {

        for ele in elements {
            // let mut bucket

        }
    }

    println!("elements: {:?}", elements);
    println!("max: {:?}", max);

    println!("Hello, world!");
}
