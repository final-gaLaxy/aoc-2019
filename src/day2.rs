#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|d| d.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<usize>) -> i32 {
    let mut p = input.clone();

    // poor way of determining if input is a test or not
    if p.len() > 100 {
        p[1] = 12; p[2] = 2;
    }

    let r = std::ops::Range {start: 0, end: p.len()}
        .step_by(4);
    for i in r {
        match p[i] {
            99 => break,
            1 => {
                let (a, b, c) = (p[i+1], p[i+2], p[i+3]);
                p[c] = p[a] + p[b];
            },
            2 => {
                let (a, b, c) = (p[i+1], p[i+2], p[i+3]);
                p[c] = p[a] * p[b];
            },
            _ => {}
        }
    }

    p[0] as i32
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<usize>) -> i32 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut p = input.clone();

            // update input
            p[1] = noun; p[2] = verb;

            let r = std::ops::Range {start: 0, end: p.len()}
                .step_by(4);
            for i in r {
                match p[i] {
                    99 => break,
                    1 => {
                        let (a, b, c) = (p[i+1], p[i+2], p[i+3]);
                        p[c] = p[a] + p[b];
                    },
                    2 => {
                        let (a, b, c) = (p[i+1], p[i+2], p[i+3]);
                        p[c] = p[a] * p[b];
                    },
                    _ => {}
                }
            }
            if p[0] == 19690720 {
                return (100 * noun + verb) as i32;
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    // Part 1
    #[test]
    fn example1() {
        assert_eq!(solve_part1(&input_generator("1,0,0,0,99")), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part1(&input_generator("2,3,0,3,99")), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(solve_part1(&input_generator("2,4,4,5,99,0")), 2);
    }

    #[test]
    fn example4() {
        assert_eq!(solve_part1(&input_generator("1,1,1,4,99,5,6,0,99")), 30);
    }
}