fn main() {
    let (min, max) = (193651, 649729);

    let mut valid_part1 = 0;
    let mut valid_part2 = 0;

    for n in min..=max {
        if validate_part1(n) {
            valid_part1 += 1;
        }
        if validate_part2(n) {
            valid_part2 += 1;
        }
    }

    // part 1: 1605
    // part 2: 1102
    println!("part 1: {}, part 2: {}", valid_part1, valid_part2);
}

fn validate_part1(x: i32) -> bool {
    let value = to_array(x);

    let mut double: bool = false;
    for n in 0..=4 {
        if value[n] > value[n + 1] {
            return false;
        }
        // check double
        if value[n] == value[n + 1] {
            double = true;
        }
    }

    double
}

#[test]
fn test_to_array() {
    assert_eq!([6, 4, 9, 7, 2, 9], to_array(649729));
}

fn to_array(x: i32) -> [i32; 6] {
    let mut result = [0; 6];

    for n in 0..=5 {
        result[n] = x % 10_i32.pow(6-n as u32) / 10_i32.pow(5-n as u32);
    }

    result
}

fn validate_part2(x: i32) -> bool {
    let value = to_array(x);

    let mut double = false;
    for n in 0..=4 {
        if value[n] > value[n + 1] {
            return false;
        }
        // check double
        if value[n] == value[n + 1] {
            if n < 4 && value[n] == value[n + 2] {
                continue;
            }
            if n > 0 && value[n] == value[n - 1] {
                continue;
            }
            double = true;
        }
    }
    double
}

#[test]
fn test_part2() {
    type Test = (i32, bool);

    let tests: [Test; 3] = [(112233, true), (123444, false), (111122, true)];

    for t in &tests {
        assert_eq!(t.1, validate_part2(t.0), "{} should be {}", t.1, t.0);
    }
}

