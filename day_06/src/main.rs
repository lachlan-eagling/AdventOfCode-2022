use std::collections::HashSet;

fn main() {
    let data: Vec<char> = include_str!("input").chars().collect();
    println!("Part 1: {}", find_in_window(&data, 4)); // 1651
    println!("Part 2: {}", find_in_window(&data, 14)); // 3837
}

fn find_in_window(data: &Vec<char>, window_size: usize) -> usize {
    for i in 0..data.len() - window_size + 1 {
        let window = &data[i..i + window_size];
        let mut unique: HashSet<char> = HashSet::new();
        unique.extend(window);
        if unique.len() == window_size {
            return i + window_size;
        }
    }
    return 0;
}
