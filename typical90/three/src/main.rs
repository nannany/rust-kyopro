use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: usize,
        e: [(usize, usize); n-1]
    }

    let mut adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new();

    for (a, b) in e {
        adjacency_list.entry(a).or_insert(Vec::new()).push(b);
        adjacency_list.entry(b).or_insert(Vec::new()).push(a);
    }

    let (longest_point, _) = get_longest_point_and_distance(&adjacency_list, 1, 1);

    let (_, longest_distance) = get_longest_point_and_distance(&adjacency_list, longest_point, longest_point);

    println!("{}", longest_distance + 1);
}

fn get_longest_point_and_distance(
    adjacency_list: &HashMap<usize, Vec<usize>>,
    start_point: usize,
    parent_point: usize,
) -> (usize, usize) {
    let mut longest_point = start_point;
    let mut longest_distance = 0;

    for &next_point in adjacency_list.get(&start_point).unwrap() {
        if next_point == parent_point {
            continue;
        }

        let (next_longest_point, next_longest_distance) =
            get_longest_point_and_distance(adjacency_list, next_point, start_point);

        if longest_distance < next_longest_distance + 1 {
            longest_distance = next_longest_distance + 1;
            longest_point = next_longest_point;
        }
    }

    (longest_point, longest_distance)
}
