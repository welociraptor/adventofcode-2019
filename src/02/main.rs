use std::fs;

fn main() {
    let input = fs::read_to_string("src/02/input.txt").expect("error reading input");

    println!("part 1: {}", intcode(&input, (12, 2))); // 2692315

    let mut params: (usize, usize) = (0, 0);
    let mut output: usize = 0;
    loop {
        output = intcode(&input, params); // brute force like a boss
        if output == 19690720 {
            break;
        }
        match params {
            (a, b) if a < 99 && b < 99 => params.0 += 1,
            (a, _) if a == 99 => {
                params.0 = 0;
                params.1 += 1
            }
            _ => panic!("wtf?"),
        }
    }

    println!("part 2: {}", (100 * params.0) + params.1); // 9507
}

fn intcode(input: &String, params: (usize, usize)) -> usize {
    let mut program = input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    program[1] = params.0;
    program[2] = params.1;

    let mut i = 0;
    while i < program.len() {
        let op1 = program[i + 1];
        let op2 = program[i + 2];
        let target = program[i + 3];
        match program[i] {
            1 => program[target] = program[op1] + program[op2],
            2 => program[target] = program[op1] * program[op2],
            99 => break,
            _ => panic!("wtf? {}", program[i]),
        }
        i += 4;
    }
    return program[0]
}