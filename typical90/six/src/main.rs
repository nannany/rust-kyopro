fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        s: String
    }

    let mut ans = String::new();

    let mut deque = std::collections::VecDeque::new();

    for (i, s) in s.char_indices() {
        while deque.back().map_or(false, |&c| c > s) {
            deque.pop_back();
        }
        deque.push_back(s);
        if i + k >= n {
            ans.push(deque.pop_front().unwrap());
        }
    }

    println!("{}", ans);
}
