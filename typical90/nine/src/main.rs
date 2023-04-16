fn main() {
    proconio::input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut answer: f64 = 0.0;

    // xyをprintln
    for &(x_origin, y_origin) in &xy {
        for &(x, y) in &xy {
            if (x == x_origin) && (y == y_origin) {
                continue;
            }


        }
    }
}

//
fn calculate_angle(x1: i64, y1: i64, x2: i64, y2: i64) -> f64 {
    let x:f64 = (x2 - x1) as f64;
    let y:f64 = (y2 - y1) as f64;
    let rad = y.atan2(x);
    rad * 180.0 / std::f64::consts::PI
}


