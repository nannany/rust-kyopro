use std::collections::HashMap;
use proconio::marker::*;

fn main() {
    proconio::input! {
        n: usize,
        e: [(Usize1, Usize1); n-1]
    }

    let mut adjacency_list:HashMap<usize, Vec<usize>> = HashMap::new();

    for (a, b) in e {
        adjacency_list.entry(a).or_insert(Vec::new()).push(b);
    }

    println!("Hello, world!");
}
