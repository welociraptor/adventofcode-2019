use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("error reading input");
    let data = input.lines();
    let mut result_part1: f32 = 0.0;
    let mut result_part2: f32 = 0.0;
    for s in data {
        let mass: f32 = s.parse().unwrap();

        let f = fuel(mass);

        result_part1 += f;
        result_part2 += f;

        let mut ef = fuel(f);
        while ef > 0.0 {
            result_part2 += ef;
            ef = fuel(ef);
        }
    }
    println!("Answer for part one: {}", result_part1); // 3317970
    println!("Answer for part two: {}", result_part2); // 4974073
}

fn fuel(mass: f32) -> f32 {
    (mass / 3.0).floor() - 2.0
}
