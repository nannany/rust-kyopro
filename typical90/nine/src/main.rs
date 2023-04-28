use std::f64::consts::PI;

fn main() {
    proconio::input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut answer: f64 = 0.0;
    for (ox, oy) in &xy {
        let mut angles: Vec<f64> = Vec::new();

        for (tx, ty) in &xy {
            if ox == tx && oy == ty {
                continue;
            }
            let dx = (tx - ox) as f64;
            let dy = (ty - oy) as f64;
            let angle = ((dy.atan2(dx) / PI) * 180.0);
            if angle < 0.0 {
                angles.push(angle + 360.0);
            } else {
                angles.push(angle);
            }
        }

        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for &angle in &angles {
            let mut ideal = 0.0;
            if angle < 180.0 {
                ideal = angle + 180.0;
            } else {
                ideal = angle - 180.0;
            }

            let candidate = bin_search(&angles, ideal);

            answer = answer.max((candidate - angle).abs().);
        }
    }

    println!("{}", answer);
}

fn bin_search(angles: &Vec<f64>, ideal: f64) -> f64 {
    let mut left = 0;
    let mut right = angles.len() - 1;
    while left < right {
        let mid = (left + right) / 2;
        if angles[mid] < ideal {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    angles[left]
}


