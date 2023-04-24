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

        let mut canonical_angles: Vec<f64> = angles.iter().map(|&angle| {
            angle % 360.0
        }).collect();

        canonical_angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for angle in &canonical_angles {
            // 該当のangleの理想の相手の角度を求める
            let ideal_angle = (angle + 180.0) % 360.0;

            let candidate = binary_search(&canonical_angles, ideal_angle);

            println!("{}", candidate);

            // if candidate is larger than answer, then update answer
            answer = answer.max(candidate);
        }
    }
    println!("{}", answer);
}

fn calculate_angle(x1: i64, y1: i64, x2: i64, y2: i64) -> f64 {
    let x: f64 = (x2 - x1) as f64;
    let y: f64 = (y2 - y1) as f64;
    let rad = y.atan2(x);
    rad * 180.0 / std::f64::consts::PI
}

// 二分探索。anglesのなかで、ideal_angleに最も近いものを探す
fn binary_search(angles: &Vec<f64>, ideal_angle: f64) -> f64 {
    let mut left: usize = 0;
    let mut right: usize = angles.len() - 1;

    while left < right {
        let mid = (left + right) / 2;
        if angles[mid] == ideal_angle {
            return angles[mid];
        } else if angles[mid] < ideal_angle {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    // left == right
    angles[left]
}


