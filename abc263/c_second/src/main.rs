use itertools::*;
use proconio::*;

fn main() {
    input! {
        n: usize,
        m: usize
    }

    dfs(n, m, &mut vec![], 1)
}

pub fn dfs(n: usize, m: usize, ans: &mut Vec<usize>, site: usize) {
    if ans.len() == n {
        println!("{}", ans.iter().join(" "));
        return;
    }

    for i in site..=m {
        ans.push(i);
        dfs(n, m, ans, i + 1);
        ans.pop();
    }
}
