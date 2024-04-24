use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> HashMap<String, String> {
    input
        .trim()
        .lines()
        .map(|line|
            line.split_at(line.find(')').unwrap())
        )
        .map(|(object, orbiter)| (&orbiter[1..], object))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

pub fn solve(
    name: &String,
    satellites: &HashMap<String, String>,
    orbits: &mut HashMap<String, i32>
) -> i32 {
    match orbits.get(name) {
        Some(child) => *child,
        None => {
            if !satellites.contains_key(name) {
                0
            } else {
                let value = 1 + solve(
                    satellites.get(name).unwrap(),
                        satellites,
                        orbits
                );
                orbits.insert(name.to_string(), value);
                *orbits.get(name).unwrap()
            }
        }
    }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &HashMap<String, String>) -> i32 {
    let satellites: HashMap<String, String> = input.clone();
    let mut orbits: HashMap<String, i32> = HashMap::new();
    let mut sum = 0;

    for k in satellites.keys() {
        sum += solve(k, &satellites, &mut orbits);
    }

    sum
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &HashMap<String, String>) -> i32 {
    let satellites: HashMap<String, String> = input.clone();
    let mut orbits: HashMap<String, i32> = HashMap::new();
    orbits.insert(String::from("COM"), 0);

    for k in satellites.keys() {
        solve(k, &satellites, &mut orbits);
    }

    let mut l: &String = &satellites
        .get(&String::from("YOU"))
        .unwrap();
    let mut r: &String = &satellites
        .get(&String::from("SAN"))
        .unwrap();
    let mut steps = 0;
    while l != r {
        if orbits.get(l).unwrap() < orbits.get(r).unwrap() {
            r = satellites.get(r).unwrap();
        } else {
            l = satellites.get(l).unwrap();
        }

        steps += 1
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    // Part 1
    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(&input_generator("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L")),
            42
        );
    }

    // Part 2
    #[test]
    fn example2() {
        assert_eq!(
            solve_part2(&input_generator("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN")),
            4
        );
    }
}