use regex::Regex;

struct Instruction {
    count: i32,
    from: i32,
    to: i32
}

fn main() {
    let data = include_str!("input");
    let stacks = parse_initial_state(data);
    let instructions = parse_instructions(data);

    println!("Part 1: {}", process_stacks(&mut stacks.clone(), &instructions, false)); // SPFMVDTZT
    println!("Part 2: {}", process_stacks(&mut stacks.clone(), &instructions, true)); // ZFSJBPRFP
}

fn process_stacks(stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>, multi_move: bool) -> String {
    let mut result = String::new();
    for instruction in instructions {
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..instruction.count {
            temp.push(stacks[(instruction.from - 1) as usize].pop().unwrap());
        }
        if multi_move {
            temp.reverse();
        }
        stacks[(instruction.to - 1) as usize].extend(temp);
    }

    for stack in stacks {
        result.push(*stack.last().unwrap());
    }
    return result;
}

fn parse_initial_state(data: &str) -> Vec<Vec<char>> {
    let state_idx: Vec<usize> = vec![1, 5, 9, 13, 17, 21, 25, 29, 33];
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 1..10 {
        let stack: Vec<char> = Vec::new();
        stacks.push(stack);
    }


    for line in data.split("\n") {
        if line.is_empty() {
            continue;
        }
        
        if line.contains("[") {
            let mut i = 0;
            for idx in &state_idx {
                let c = line.as_bytes()[*idx] as char;
                if c != ' ' {
                    stacks[i].push(c);
                }
                i += 1;
            }
        }

        if line.starts_with("move") {
            break;
        }
    }

    for stack in &mut stacks {
        stack.reverse(); // TODO: This is a massive hack, should build a proper stack structure.
    }
    
    return stacks;
}

fn parse_instructions(data: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in data.split("\n") {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("move") {
            let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
            for cap in re.captures(line) {
                let instruction = Instruction{
                    count: cap[1].parse::<i32>().unwrap(),
                    from: cap[2].parse::<i32>().unwrap(),
                    to: cap[3].parse::<i32>().unwrap()
                };
                instructions.push(instruction);
            }
        }
    }
    return instructions;
}