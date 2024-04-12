use std::fmt::Write;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> String {
    let mut result: String = String::new();
    write!(&mut result, "{}", input).unwrap();

    result
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

    #[test]
    fn example1() {
        assert_eq!(solve_part1("Hello"), "Hello")
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part2("World"), "World")
    }
}