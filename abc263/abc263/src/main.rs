fn main() {
    proconio::input!(mut a: [u32; 5]);
    a.sort();
    let mut answer = "No";

    if a[0] == a[2] && a[3] == a[4] {
        answer = "Yes"
    }
    a.reverse();
    if a[0] == a[2] && a[3] == a[4] {
        answer = "Yes"
    }

    println!("{}", answer);

}
