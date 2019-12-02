use std::fs;

fn main() {
    let input = fs::read_to_string("src/01/input.txt").expect("error reading input");

    let sum: f32 = input.lines().map(|v| fuel(v.parse().unwrap())).sum();
    println!("Answer for part one: {}", sum); // 3317970

    let mut result_part2: f32 = 0.0;
    for s in input.lines() {
        let f = fuel(s.parse().unwrap());

        result_part2 += f;

        let mut ef = fuel(f);
        while ef > 0.0 {
            result_part2 += ef;
            ef = fuel(ef);
        }
    }
    println!("Answer for part two: {}", result_part2); // 4974073
}

fn fuel(mass: f32) -> f32 {
    (mass / 3.0).floor() - 2.0
}
