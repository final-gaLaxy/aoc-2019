use std::fmt::Write;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    input.lines()
         .map(|tok| {
            let module = tok.trim().parse::<u32>().unwrap();
            module / 3 - 2
         })
         .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> String {
    let mut result: String = String::new();
    write!(&mut result, "{}", input).unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // Part 1
    #[test]
    fn example1() {
        assert_eq!(solve_part1("12"), 2)
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part1("14"), 2)
    }

    #[test]
    fn example3() {
        assert_eq!(solve_part1("1969"), 654)
    }

    #[test]
    fn example4() {
        assert_eq!(solve_part1("100756"), 33583)
    }

    #[test]
    fn example5() {
        assert_eq!(solve_part1("12\n14\n1969\n100756"), [2,2,654,33583].iter().sum())
    }
}