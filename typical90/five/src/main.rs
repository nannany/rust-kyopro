use itertools::*;

fn main() {
    proconio::input! {
        n: usize,
        b: usize,
        k: usize,
        c: [usize; k]
    }

    // matrix を0で初期化
    let mut matrix: SquareMatrix = SquareMatrix::new(b, vec![vec![0; b]; b]);
    // matrix を計算する
    for i in 0..b {
        for &c in &c {
            matrix.data[i][(i * 10 + c) % b] += 1;
        }
    }

    // matrix を n
    let n_matrix = matrix_pow(matrix, n);

    println!("{}", n_matrix.data[0][0] % 1000000007);
}

/// 引数に受け取ったmatrix を n 乗したものを返す
fn matrix_pow(matrix: SquareMatrix, n: usize) -> SquareMatrix {
    let mut ans: SquareMatrix = SquareMatrix::new(matrix.sx, vec![vec![0; matrix.sx]; matrix.sx]);

    // 単位行列として初期化
    for i in 0..matrix.sx {
        ans.data[i][i] = 1;
    }

    let mut tmp = matrix;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            ans = ans.matrix_mul_from_right(&tmp.clone());
        }
        tmp = tmp.clone().matrix_mul_from_right(&tmp.clone());
        n >>= 1;
    }
    ans
}

/// 1辺のながさがsxの正方行列
struct SquareMatrix {
    sx: usize,
    data: Vec<Vec<usize>>,
}

impl SquareMatrix {
    fn new(sx: usize, data: Vec<Vec<usize>>) -> Self {
        Self { sx, data }
    }

    fn matrix_mul_from_right(&self, other: &SquareMatrix) -> SquareMatrix {
        let mut ans: SquareMatrix = SquareMatrix::new(self.sx, vec![vec![0; self.sx]; self.sx]);
        for i in 0..self.sx {
            for j in 0..self.sx {
                for k in 0..self.sx {
                    ans.data[i][j] += (self.data[i][k] % 1000000007) * (other.data[k][j] % 1000000007) % 1000000007;
                }
            }
        }
        ans
    }

    fn clone(&self) -> Self {
        let mut data: Vec<Vec<usize>> = vec![vec![0; self.sx]; self.sx];
        for i in 0..self.sx {
            for j in 0..self.sx {
                data[i][j] = self.data[i][j];
            }
        }
        Self::new(self.sx, data)
    }
}

