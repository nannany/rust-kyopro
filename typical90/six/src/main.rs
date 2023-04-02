const ALPHABETS: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        s: String
    }

    // 各アルファベットについて、該当の順番から右のどの位置に出現するかを格納した配列
    let mut position: Vec<Vec<usize>> = vec![vec![s.len(); n]; 26];

    for i in (0..n).rev() {
        let c = s.chars().nth(i).unwrap();
        let index = ALPHABETS.iter().position(|&x| x == c).unwrap();
        position[index][i] = i;

        if i != n - 1 {
            for j in 0..26 {
                if j != index {
                    position[j][i] = position[j][i + 1];
                }
            }
        }
    }

    let mut current_pos = 0;

    let mut answer = String::new();

    for i in 0..k {
        for j in 0..26 {
            let next_pos = position[j][current_pos] as i32;
            let max_possible_length: i32 = n as i32 - next_pos - 1 + i as i32 + 1;
            if max_possible_length >= k as i32 {
                answer.push(ALPHABETS[j]);
                current_pos = next_pos as usize + 1;
                break;
            }
        }
    }

    println!("{}", answer);
}
