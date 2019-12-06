use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let orbits = parse_orbits(read_input());
    let mut total = 0;
    for mut o in orbits.keys() {
        //println!("{:?}", m.keys());
        loop {
            //println!("{}", m.get(k).unwrap());
            o = match orbits.get(o) {
                Some(oc) => {
                    total += 1;
                    oc
                }
                None => break,
            }
        }
    }
    println!("Answer to part 1: {}", total); // 130681

    let (mut ym, mut sm) = (HashSet::new(), HashSet::new());
    let mut yl = orbits.get("YOU").unwrap().clone();
    let mut sl = orbits.get("SAN").unwrap().clone();
    let mut youpath = String::new();
    youpath.push_str(yl.as_str());

    let mut sanpath = String::new();
    sanpath.push_str(sl.as_str());
    ym.insert(yl.clone());
    sm.insert(sl.clone());
    loop {
        match orbits.get(yl.clone().as_str()) {
            Some(o) => {
                yl = o.clone();
                youpath.push_str(yl.as_str());
                ym.insert(yl.clone());
            }
            None => (),
        }
        if check(&ym, &sm) {
            break;
        }
        match orbits.get(sl.clone().as_str()) {
            Some(o) => {
                sl = o.clone();
                sanpath.push_str(sl.as_str());
                sm.insert(sl.clone());
            }
            None => (),
        }
        if check(&ym, &sm) {
            break;
        }
    }
    println!("{}", youpath);
    println!("{}", sanpath);

    // Not 382, 380 or 378
    println!("Answer to part 2: {}", (ym.len() + sm.len()) - 4);
}

#[test]
fn test_check() {
    let a: HashSet<String> = ["SAN".to_owned(), "AAA".to_owned(), "BBB".to_owned()].iter().cloned().collect();
    let b: HashSet<String> = vec!["XXX", "YYY", "BBB"].into_iter().map(|x|x.to_owned()).collect();
    assert_eq!(true, check(&a, &b));
    println!("jumps: {}", a.len() + b.len());
}

fn check(a: &HashSet<String>, b: &HashSet<String>) -> bool {
    for aa in a.iter() {
        if b.contains(aa) {
            return true;
        }
    }
    false
}

fn read_input() -> String {
    fs::read_to_string("src/06/input.txt").expect("error reading input")
}

fn parse_orbits(input: String) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for a in input.lines() {
        let mut i = a.split(")");
        let c = i.next().unwrap().to_string();
        let o = i.next().unwrap().to_string();
        //println!("{} orbits {}", o, c);
        map.insert(o, c);
    }
    map
}
