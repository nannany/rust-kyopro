fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h]
    }

    // get row sums
    let mut row_sums: Vec<i64> = vec![0; h];
    for i in 0..h {
        for j in 0..w {
            row_sums[i] += a[i][j];
        }
    }

    // get column sums
    let mut column_sums: Vec<i64> = vec![0; w];
    for i in 0..w {
        for j in 0..h {
            column_sums[i] += a[j][i];
        }
    }

    // print answer
    for i in 0..h {
        for j in 0..w {
            print!("{} ", row_sums[i] + column_sums[j] - a[i][j]);
        }
        println!();
    }
}
