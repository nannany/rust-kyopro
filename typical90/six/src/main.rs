const ALPHABETS: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        s: String
    }

    // 各アルファベットについて、該当の順番から右のどの位置に出現するかを格納した配列
    let mut position: Vec<Vec<i32>> = vec![vec![-1; n]; 26];

    for i in (0..n).rev() {
        let c = s.chars().nth(i).unwrap();
        let index = ALPHABETS.iter().position(|&x| x == c).unwrap();
        position[index][i] = i as i32;

        if i != n - 1 {
            for j in 0..26 {
                if j != index {
                    position[j][i] = position[j][i + 1];
                }
            }
        }
    }

    let mut count = 0;

    let mut answer = String::new();

    while answer.len() != k {
        for i in 0..26 {
            // if position[i][count] + k as i32 - answer.len() as i32 == n as i32 {
            //     answer.push_str(s.chars().skip(position[i][count] as usize - 1).collect::<String>().as_str());
            //     return;
            // }
            if position[i][count] != -1 && position[i][count] as usize + k - answer.len() <= n {
                answer.push(ALPHABETS[i]);
                count = (position[i][count] + 1) as usize;
                break;
            }
        }
    }

    println!("{}", answer);
}
