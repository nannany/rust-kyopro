fn main() {
    proconio::input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut answer: f64 = 0.0;

    // xyをprintln
    for &(x_origin, y_origin) in &xy {
        let mut angles: Vec<f64> = Vec::new();

        for &(x, y) in &xy {
            if (x == x_origin) && (y == y_origin) {
                continue;
            }

            let angle = calculate_angle(x_origin, y_origin, x, y);
            angles.push(angle);
        }

        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for angle in &angles {
            // 該当のangleの理想の相手の角度を求める


            binary_search()
        }
    }
}

fn calculate_angle(x1: i64, y1: i64, x2: i64, y2: i64) -> f64 {
    let x: f64 = (x2 - x1) as f64;
    let y: f64 = (y2 - y1) as f64;
    let rad = y.atan2(x);
    rad * 180.0 / std::f64::consts::PI
}

// 二分探索
fn binary_search(angles: Vec<f64>, org_angle: f64) -> f64 {
    let mut left: usize = 0;
    let mut right: usize = angles.len() - 1;
    let mut angle: f64 = 0.0;

    while left <= right {
        let mid: usize = (left + right) / 2;



    }

    0.9
}

// calc angle

