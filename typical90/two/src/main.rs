fn main() {
    proconio::input! {
        n: usize
    }

    if n % 2 == 1 {
        println!();
    } else {
        resolve(&String::from(""), n / 2, n / 2);
    }
}

fn resolve(letters: &str, l: usize, r: usize) {
    if l == 0 && r == 0 {
        println!("{}", letters);
    } else if l == r {
        resolve(&format!("{}{}", letters, "("), l - 1, r);
    } else if l < r {
        if l != 0 {
            resolve(&format!("{}{}", letters, "("), l - 1, r);
        }
        resolve(&format!("{}{}", letters, ")"), l, r - 1);
    }
}
