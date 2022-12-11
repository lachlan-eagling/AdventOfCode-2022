fn main() {
    let data = include_str!("input");

    let mut totals = Vec::new();
    let mut current_total = 0;
    for line in data.split("\n") {
        if line == "" {
            totals.push(current_total);
            current_total = 0;
            continue;
        }
        let calories: i32 = line.parse().unwrap();
        current_total += calories;
    }

    totals.sort_by(|a, b| b.cmp(a));
    
    let part_1 = totals[0];
    let part_2 = totals[0..3].iter().fold(0, |sum, x| sum + x);
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
