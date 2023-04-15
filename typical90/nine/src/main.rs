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

            println!("({}, {}) -> ({}, {})", x_origin, y_origin, x, y);
        }
    }
}

//
fn calculate_angle(x1: i64, y1: i64, x2: i64, y2: i64) -> f64 {
    let x = x2 - x1;
    let y = y2 - y1;
    let rad = y.atan2(x);
    let deg = rad * 180.0 / std::f64::consts::PI;

    if deg < 0.0 {
        deg + 360.0
    } else {
        deg
    }
}

