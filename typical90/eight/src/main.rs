fn main() {
    proconio::input! {
        _n: usize,
        s: String,
    }

    // sからa,t,c,o,d,e,rの文字を取り出す
    let mut s = s.chars().filter(|&c| "atcoder".contains(c)).collect::<Vec<_>>();

    // sを整形する
    let mut tmp_s = "".to_string();

    let mut bools_from_first = [false; 7];

    for i in 0..s.len() {
        if s[i] == 'a' {
            bools_from_first[0] = true;
            tmp_s.push('a')
        } else if s[i] == 't' {
            if bools_from_first[0] == true {
                bools_from_first[1] = true;
                tmp_s.push('t')
            }
        } else if s[i] == 'c' {
            if bools_from_first[1] == true {
                bools_from_first[2] = true;
                tmp_s.push('c')
            }
        } else if s[i] == 'o' {
            if bools_from_first[2] == true {
                bools_from_first[3] = true;
                tmp_s.push('o')
            }
        } else if s[i] == 'd' {
            if bools_from_first[3] == true {
                bools_from_first[4] = true;
                tmp_s.push('d')
            }
        } else if s[i] == 'e' {
            if bools_from_first[4] == true {
                bools_from_first[5] = true;
                tmp_s.push('e')
            }
        } else if s[i] == 'r' {
            if bools_from_first[5] == true {
                bools_from_first[6] = true;
                tmp_s.push('r')
            }
        }
    }

    let mut bools_from_last = [false; 7];
    let mut arranged_s = "".to_string();

    let mut tmp_chars = tmp_s.chars().collect::<Vec<_>>();

    for i in (0..tmp_chars.len()).rev() {
        if tmp_chars[i] == 'r' {
            bools_from_last[6] = true;
            arranged_s.insert(0, 'r')
        } else if tmp_chars[i] == 'e' {
            if bools_from_last[6] == true {
                bools_from_last[5] = true;
                arranged_s.insert(0, 'e')
            }
        } else if tmp_chars[i] == 'd' {
            if bools_from_last[5] == true {
                bools_from_last[4] = true;
                arranged_s.insert(0, 'd')
            }
        } else if tmp_chars[i] == 'o' {
            if bools_from_last[4] == true {
                bools_from_last[3] = true;
                arranged_s.insert(0, 'o')
            }
        } else if tmp_chars[i] == 'c' {
            if bools_from_last[3] == true {
                bools_from_last[2] = true;
                arranged_s.insert(0, 'c')
            }
        } else if tmp_chars[i] == 't' {
            if bools_from_last[2] == true {
                bools_from_last[1] = true;
                arranged_s.insert(0, 't')
            }
        } else if tmp_chars[i] == 'a' {
            if bools_from_last[1] == true {
                bools_from_last[0] = true;
                arranged_s.insert(0, 'a')
            }
        }
    }

    let mut set = std::collections::HashSet::new();
    for c in arranged_s.chars() {
        set.insert(c);
    }
    if set.len() != 7 {
        println!("0");
        return;
    } else {
        let arranged_chars = arranged_s.chars().collect::<Vec<_>>();

        let mut t_count = vec![0; arranged_s.len()];
        let mut c_count = vec![0; arranged_s.len()];
        let mut o_count = vec![0; arranged_s.len()];
        let mut d_count = vec![0; arranged_s.len()];
        let mut e_count = vec![0; arranged_s.len()];
        let mut r_count = vec![0; arranged_s.len()];

        for i in (0..arranged_s.len()).rev() {
            if i == arranged_s.len() - 1 {
                r_count[i] += 1;
                continue;
            }

            if arranged_chars[i] == 'r' {
                r_count[i] = r_count[i + 1] + 1;
                e_count[i] = e_count[i + 1];
                d_count[i] = d_count[i + 1];
                o_count[i] = o_count[i + 1];
                c_count[i] = c_count[i + 1];
                t_count[i] = t_count[i + 1];
            } else if arranged_chars[i] == 'e' {
                e_count[i] = e_count[i + 1] + 1;
                r_count[i] = r_count[i + 1];
                d_count[i] = d_count[i + 1];
                o_count[i] = o_count[i + 1];
                c_count[i] = c_count[i + 1];
                t_count[i] = t_count[i + 1];
            } else if arranged_chars[i] == 'd' {
                d_count[i] = d_count[i + 1] + 1;
                r_count[i] = r_count[i + 1];
                e_count[i] = e_count[i + 1];
                o_count[i] = o_count[i + 1];
                c_count[i] = c_count[i + 1];
                t_count[i] = t_count[i + 1];
            } else if arranged_chars[i] == 'o' {
                o_count[i] = o_count[i + 1] + 1;
                r_count[i] = r_count[i + 1];
                e_count[i] = e_count[i + 1];
                d_count[i] = d_count[i + 1];
                c_count[i] = c_count[i + 1];
                t_count[i] = t_count[i + 1];
            } else if arranged_chars[i] == 'c' {
                c_count[i] = c_count[i + 1] + 1;
                r_count[i] = r_count[i + 1];
                e_count[i] = e_count[i + 1];
                d_count[i] = d_count[i + 1];
                o_count[i] = o_count[i + 1];
                t_count[i] = t_count[i + 1];
            } else if arranged_chars[i] == 't' {
                t_count[i] = t_count[i + 1] + 1;
                r_count[i] = r_count[i + 1];
                e_count[i] = e_count[i + 1];
                d_count[i] = d_count[i + 1];
                o_count[i] = o_count[i + 1];
                c_count[i] = c_count[i + 1];
            } else if arranged_chars[i] == 'a' {
                r_count[i] = r_count[i + 1];
                e_count[i] = e_count[i + 1];
                d_count[i] = d_count[i + 1];
                o_count[i] = o_count[i + 1];
                c_count[i] = c_count[i + 1];
                t_count[i] = t_count[i + 1];
            }
        }

        let mut answer = 0;
        for i in 0..arranged_s.len() {
            if arranged_chars[i] == 'a' {
                answer += r_count[i] * e_count[i] * d_count[i] * o_count[i] * c_count[i] * t_count[i];
                answer %= 1000000007;
            }
        }

        println!("{}", answer);
    }
}
