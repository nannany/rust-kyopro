use proconio::input;

fn main() {
    input!(n: usize, m: usize);

    let ans: Vec<usize> = Vec::new();

    let mut memo: Vec<bool> = initialize_memo(m);

    dfn(ans, memo, n, m);
}

pub fn dfn(ans: Vec<usize>, memo: Vec<bool>, n: usize, m: usize) {
    if ans.len() == n {
        let ans_str_vec: Vec<String> = ans.iter().map(|x| x.to_string()).collect();
        let ans_str: String = ans_str_vec.join(" ");
        println!("{}", ans_str);
    }

    let unreached_number_option: Option<usize> = memo.iter().position(|x| *x != true);
    let unreached_number: usize = match unreached_number_option {
        Some(number) => number,
        _ => return
    };

    for i in unreached_number..m {
        let mut ans_tmp = ans.to_vec();
        let mut memo_tmp = memo.to_vec();
        ans_tmp.push(i + 1);
        for j in 0..=i {
            std::mem::replace(&mut memo_tmp[j], true);
        }
        dfn(ans_tmp, memo_tmp, n, m);
    }
}

pub fn initialize_memo(max_len: usize) -> Vec<bool> {
    let mut memo: Vec<bool> = Vec::new();

    for _ in 1..=max_len {
        memo.push(false);
    }

    return memo;
}
