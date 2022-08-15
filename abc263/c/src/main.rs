use proconio::input;

fn main() {
    input!(n: usize, m: usize);

    let mut num_vec: Vec<bool> = Vec::new();

    for _ in 1..=n {
        num_vec.push(false);
    }

    dfn(1, Vec::new(), initialize_memo(n), n);
}

pub fn dfn(first: usize, mut ans: Vec<usize>, mut memo: Vec<bool>, max_len: usize) {
    if ans.len() == max_len {
        let ans_str_vec: Vec<String> = ans.iter().map(|x| x.to_string()).collect();
        let ans_str: String = ans_str_vec.join(" ");
        println!("{}", ans_str);
        ans = Vec::new();
        memo[first] = true;
        dfn(first + 1, ans, memo, max_len);
    }

    ans.push(first);

    let mut ans: Vec<usize> = Vec::new();
    ans.push(first);

    // while
}


pub fn initialize_memo(max_len: usize) -> Vec<bool> {
    let mut memo: Vec<bool> = Vec::new();

    for _ in 1..=max_len {
        memo.push(false);
    }

    return memo;
}
