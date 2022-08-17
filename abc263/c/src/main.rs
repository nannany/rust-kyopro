use proconio::input;

fn main() {
    input!(n: usize, m: usize);

    let ans: Vec<usize> = vec![1];

    let mut memo: Vec<bool> = initialize_memo(m);
    std::mem::replace(&mut memo[0], true);

    dfn(ans, memo, n, m);
}

pub fn dfn(ans: Vec<usize>, memo: Vec<bool>, n: usize, m: usize) {
    if ans.len() == n {
        let ans_str_vec: Vec<String> = ans.iter().map(|x| x.to_string()).collect();
        let ans_str: String = ans_str_vec.join(" ");
        println!("{}", ans_str);
    }

    let ans_top = ans.get(ans.len() - 1).unwrap();
    if *ans_top + 1 > m {
        return
    }
    if memo[*ans_top] == false {
        let mut ans_1 = ans.clone();
        ans_1.push(*ans_top + 1);
        let mut memo_clone_1 = memo.clone();
        std::mem::replace(&mut memo_clone_1[*ans_top], true);
        dfn(ans_1, memo_clone_1, n, m);

        let mut ans_2 = ans.clone();
        let mut memo_clone_2 = memo.clone();
        std::mem::replace(&mut memo_clone_2[*ans_top], true);
        dfn(ans_2, memo_clone_2, n, m);
    } else {
        let mut memo_clone_3 = memo.clone();
        let unreached_number_option: Option<usize> = memo_clone_3.iter().position(|x| *x != true);
        let unreached_number: usize = match unreached_number_option {
            Some(number) => number,
            _ => return
        };
        std::mem::replace(&mut memo_clone_3[unreached_number], true);

        let mut ans_3 = ans.clone();
        ans_3.push(unreached_number + 1);

        dfn(ans_3, memo_clone_3, n, m);
    }
}

pub fn initialize_memo(max_len: usize) -> Vec<bool> {
    let mut memo: Vec<bool> = Vec::new();

    for _ in 1..=max_len {
        memo.push(false);
    }

    return memo;
}
