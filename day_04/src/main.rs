fn main() {
    let data = include_str!("input");

    let mut part_1_count: i32 = 0;
    let mut part_2_count: i32 = 0;
    for line in data.split("\n") {
        let parts: Vec<&str> = line.split(",").collect();

        let a: Vec<i32> = parts[0].split("-").collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect();
        let b: Vec<i32> = parts[1].split("-").collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect();

        if (a[0] >= b[0] && a[1] <= b[1]) || (b[0] >= a[0] && b[1] <= a[1]) {
            part_1_count += 1;
        }

        if a[0] <= b[1] && a[1] >= b[0] {
            part_2_count += 1;
        }
    }
    println!("{:?}", part_1_count);
    println!("{:?}", part_2_count);
}
