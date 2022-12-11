use std::collections::HashSet;

fn main() {
    let data = include_str!("input");
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    // Part 1
    let mut part1_total = 0;
    let mut part2_total = 0;
    for line in data.split("\n") {
        let parts = line.split_at(line.len() / 2);
        let a: HashSet<char> = HashSet::from_iter(parts.0.chars());
        let b: HashSet<char> = HashSet::from_iter(parts.1.chars());

        let common  = a.intersection(&b).collect::<Vec<&char>>();

        part1_total += alphabet.find(|c: char| c == *common[0]).unwrap() + 1;
    }

    // Part 2
    let lines: Vec<&str> = data.split("\n").collect();
    let chunks: Vec<&[&str]> = lines.chunks(3).collect();

    for chunk in chunks {
        let a: Vec<char> = chunk[0].chars().collect();
        let b: Vec<char> = chunk[1].chars().collect();
        let c: Vec<char> = chunk[2].chars().collect();

        // TODO: Replace this brute force with a three-way set intersection somehow.
        let mut intersction: Vec<char> = Vec::new();
        for k in c {
            if a.contains(&k) && b.contains(&k) {
                intersction.push(k);
            }
        }
        
        part2_total += alphabet.find(|c: char| c == intersction[0]).unwrap() + 1;
    }
    println!("Part 1: {}", part1_total); // 8202
    println!("Part 2: {}", part2_total); // 2864
}
