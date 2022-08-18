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

    let mut ans_top: usize = 0;
    if ans.is_empty() {
    } else {
        ans_top = *ans.get(ans.len() - 1).unwrap();
    }

    if ans_top + 1 > m {
        return
    }
    if memo[ans_top] == false {

        for i in ans_top..m {
            let mut ans_tmp = ans.to_vec();
            let mut memo_tmp = memo.to_vec();
            ans_tmp.push(i + 1);
            for j in 0..=i {
                std::mem::replace(&mut memo_tmp[j], true);
            }
            dfn(ans_tmp, memo_tmp, n, m);
        }

        for k in ans_top..m {
            let mut ans_tmp = ans.to_vec();
            let mut memo_tmp = memo.to_vec();
            for l in 0..=k {
                std::mem::replace(&mut memo_tmp[k], true);
            }
            dfn(ans_tmp, memo_tmp, n, m);
        }
    } else {
        let mut memo_clone_3 = memo.to_vec();
        let unreached_number_option: Option<usize> = memo_clone_3.iter().position(|x| *x != true);
        let unreached_number: usize = match unreached_number_option {
            Some(number) => number,
            _ => return
        };
        std::mem::replace(&mut memo_clone_3[unreached_number], true);

        let mut ans_3 = ans.to_vec();
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
