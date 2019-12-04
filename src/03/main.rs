use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/03/input.txt").expect("error reading input");
    let mut data = Vec::new();
    for i in input.lines() {
        data.push(i);
    }

    let wire0 = Wire::new(data[0]);
    let wire1 = Wire::new(data[1]);

    /* Solution to part one. I'm too tired to fix the nearest_intersection function to not have the
       borrow checker scream at me in part two.

    println!(
        "nearest crossing: {}",
        Wire::nearest_intersection(wire0, wire1)
    );
    */

    println!(
        // Solution to part two.
        "{}",
        Wire::intersections(&wire0, &wire1)
            .iter()
            .map(|p| wire0.distance_to(*p) + wire1.distance_to(*p))
            .min()
            .unwrap()
    )
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

type Point = (i32, i32);

struct Wire {
    path: Vec<(Direction, i32)>,
}

impl Wire {
    fn new(input: &str) -> Wire {
        Wire {
            path: input
                .split(',')
                .map(|x| {
                    (
                        match &x[..1] {
                            "U" => Direction::UP,
                            "D" => Direction::DOWN,
                            "L" => Direction::LEFT,
                            "R" => Direction::RIGHT,
                            _ => panic!("unknown direction"),
                        },
                        x[1..].parse::<i32>().unwrap(),
                    )
                })
                .collect::<Vec<(Direction, i32)>>(),
        }
    }
    fn points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        let mut location: Point = (0, 0);
        for instruction in self.path.iter() {
            let mut distance = 0;
            match instruction.0 {
                Direction::UP => {
                    while distance < instruction.1 {
                        location.1 += 1;
                        points.insert(location);
                        distance += 1;
                    }
                }
                Direction::DOWN => {
                    while distance < instruction.1 {
                        location.1 -= 1;
                        points.insert(location);
                        distance += 1;
                    }
                }
                Direction::LEFT => {
                    while distance < instruction.1 {
                        location.0 -= 1;
                        points.insert(location);
                        distance += 1;
                    }
                }
                Direction::RIGHT => {
                    while distance < instruction.1 {
                        location.0 += 1;
                        points.insert(location);
                        distance += 1;
                    }
                }
            }
        }
        points
    }
    fn nearest_intersection(wire1: Wire, wire2: Wire) -> i32 {
        let mut shortest = std::i32::MAX;
        for p in wire1.points().intersection(&wire2.points()) {
            let dist = p.0.abs() + p.1.abs();
            if dist < shortest {
                shortest = dist;
            }
        }
        shortest
    }

    fn intersections(wire1: &Wire, wire2: &Wire) -> Vec<Point> {
        let mut points = Vec::new();
        for p in wire1.points().intersection(&wire2.points()) {
            points.push((p.0, p.1));
        }
        points
    }

    fn distance_to(&self, p: Point) -> i32 {
        let mut location: Point = (0, 0);
        let mut total = 0;
        for instruction in self.path.iter() {
            let mut leg = 0;
            match instruction.0 {
                Direction::UP => {
                    while leg < instruction.1 {
                        total += 1;
                        location.1 += 1;
                        leg += 1;
                        if location == p {
                            return total;
                        }
                    }
                }
                Direction::DOWN => {
                    while leg < instruction.1 {
                        location.1 -= 1;
                        total += 1;
                        leg += 1;
                        if location == p {
                            return total;
                        }
                    }
                }
                Direction::LEFT => {
                    while leg < instruction.1 {
                        location.0 -= 1;
                        total += 1;
                        leg += 1;
                        if location == p {
                            return total;
                        }
                    }
                }
                Direction::RIGHT => {
                    while leg < instruction.1 {
                        location.0 += 1;
                        total += 1;
                        leg += 1;
                        if location == p {
                            return total;
                        }
                    }
                }
            }
        }
        total
    }
}

#[test]
fn test_part1_example1() {
    let wire1 = Wire::new("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let wire2 = Wire::new("U62,R66,U55,R34,D71,R55,D58,R83");
    assert_eq!(159, Wire::nearest_intersection(wire1, wire2));
}

#[test]
fn test_part1_example2() {
    let wire1 = Wire::new("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let wire2 = Wire::new("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    assert_eq!(135, Wire::nearest_intersection(wire1, wire2));
}

#[test]
fn test_part2_example1() {
    let wire1 = Wire::new("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let wire2 = Wire::new("U62,R66,U55,R34,D71,R55,D58,R83");
    assert_eq!(
        610,
        Wire::intersections(&wire1, &wire2)
            .iter()
            .map(|p| wire1.distance_to(*p) + wire2.distance_to(*p))
            .min()
            .unwrap()
    );
}
