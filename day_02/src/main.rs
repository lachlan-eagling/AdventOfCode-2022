fn main() {
    let data = include_bytes!("input").split(|b| *b == b'\n');
    let mut score: u32 = 0;
    let mut part_2_score: u32 = 0;

    for line in data {
        let their_move: u8 = line[0];
        let our_move: u8 = line[2];

        // A (Rock) = 65
        // B (Paper) = 66
        // C (Scissors) = 67

        // X (Rock) = 88
        // Y (Paper) = 89
        // Z (Scissors) = 90

        // Part 1
        let mut our_score = match (their_move, our_move) {
            (65, 90) | (66, 88) | (67, 89) => 0,
            (65, 89) | (66, 90) | (67, 88) => 6,
            _ => 3,
        };
       
        our_score += our_move - 88 + 1;
        score += our_score as u32;


        // Part 2
        let mut our_new_move = our_move;
        if our_move == 88 {
            // We need to lose
            our_new_move = match their_move {
                65 => 90,
                66 => 88,
                _ => 89
            };
        } else if our_move == 89 {
            // We need to draw
            our_new_move = match their_move {
                65 => 88,
                66 => 89,
                _ => 90
            };
        } else if our_move == 90 {
            // We need to win
            our_new_move = match their_move {
                65 => 89,
                66 => 90,
                _ => 88
            };
        }

        let mut our_new_score = match (their_move, our_new_move) {
            (65, 90) | (66, 88) | (67, 89) => 0,
            (65, 89) | (66, 90) | (67, 88) => 6,
            _ => 3,
        };
       
        our_new_score += our_new_move - 88 + 1;
        part_2_score += our_new_score as u32;
    }

    println!("Part 1: {}", score); // 11063
    println!("Part 2: {}", part_2_score); // 10349
}
