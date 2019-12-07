use fnv::FnvHashMap as HashMap;

static INPUT: &str = include_str!("input");

fn main() {
    let mut orbits: HashMap<String, String> = HashMap::default();
    for line in INPUT.lines() {
        let mut iter = line.trim().split(')');
        let body = iter.next().expect("no body in line").to_string();
        let satellite = iter.next().expect("no satellite orbiting body").to_string();
        orbits.insert(satellite, body);
    }

    let mut santa_bodies = vec![];
    let mut key = "SAN";
    loop {
        match orbits.get(key) {
            Some(body) => {
                santa_bodies.push(body);
                key = body
            }
            None => break,
        }
    }

    let mut dist = 0;
    let mut key = "YOU";
    loop {
        match orbits.get(key) {
            Some(body) => {
                key = body;
                match santa_bodies.iter().position(|&b| b == body) {
                    Some(idx) => {
                        dist += idx;
                        break;
                    }
                    None => {}
                }
                dist += 1;
            }
            None => panic!("what"),
        }
    }

    println!("{}", dist);
}
