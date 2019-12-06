use std::fs;

fn main() {
    let input = fs::read_to_string("src/02/input.txt").expect("error reading input");

    println!("part 1: {}", intcode(&input, (12, 2))); // 2692315

    let mut params: (usize, usize) = (0, 0);
    loop {
        if intcode(&input, params) == 19690720 {
            break;
        }
        params = match params {
            (99, 99) => break,
            (99, b) => (0, b + 1),
            (a, b) => (a + 1, b),
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
    loop {
        let (op1, op2, target) = (program[i + 1], program[i + 2], program[i + 3]);
        match program[i] {
            1 => {
                program[target] = program[op1] + program[op2];
                i += 4;
            }
            2 => {
                program[target] = program[op1] * program[op2];
                i += 4;
            }
            99 => break,
            _ => panic!("wtf? {}", program[i]),
        }
        i += 4;
    }
    return program[0];
}

#[test]
fn test_parse() {
    println!("{:?}", parse(String::from("1002,0,0,1")));
}

fn parse(input: String) -> Vec<Instruction> {
    let mut data: Vec<_> = input.split(',').collect();

    let opcode = format!("{:0>5}", data[0]);
    let mut i = opcode.chars();
    let pm3 = mode(i.next().unwrap());
    let pm2 = mode(i.next().unwrap());
    let pm1 = mode(i.next().unwrap());
    let op = i.collect::<String>();

    println!("pm1:{:?},pm2:{:?},pm3:{:?}", pm1, pm2, pm3);
    println!("opcode:{:?}", opcode.chars().skip(3).collect::<String>());

    let ret = Vec::new();
    ret
}

#[test]
fn test_instruction() {
    println!("{:?}", instruction(String::from("1002,11,22,33")));
}

fn instruction(input: String) -> Instruction {
    let data = format!("{:0>5}", input);
    let mut chars = data.chars();
    let pm3 = mode(chars.next().unwrap());
    let pm2 = mode(chars.next().unwrap());
    let pm1 = mode(chars.next().unwrap());
    let opcode = chars.collect::<String>().clone();
    println!("opcode:{}", &opcode);
    match opcode.as_str() {
        "01" => Instruction::ADD(
            Parameter {
                mode: pm1,
                value: 0,
            },
            Parameter {
                mode: pm2,
                value: 0,
            },
            Parameter {
                mode: pm3,
                value: 0,
            },
        ),
        "02" => Instruction::MULTIPLY(
            Parameter {
                mode: pm1,
                value: 0,
            },
            Parameter {
                mode: pm2,
                value: 0,
            },
            Parameter {
                mode: pm3,
                value: 0,
            },
        ),
        "03" => Instruction::INPUT(Parameter {
            mode: ParameterMode::POSITION,
            value: 0,
        }),
        "04" => Instruction::OUTPUT(Parameter {
            mode: ParameterMode::POSITION,
            value: 0,
        }),
        _ => panic!("unrecognized opcode"),
    }
}

fn mode(c: char) -> ParameterMode {
    match c {
        '0' => ParameterMode::IMMEDIATE,
        '1' => ParameterMode::POSITION,
        _ => panic!("unknown parameter mode"),
    }
}

#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    value: usize,
}

#[derive(Debug)]
enum Instruction {
    ADD(Parameter, Parameter, Parameter),
    MULTIPLY(Parameter, Parameter, Parameter),
    INPUT(Parameter),
    OUTPUT(Parameter),
}

#[derive(Debug)]
enum ParameterMode {
    POSITION,
    IMMEDIATE,
}
