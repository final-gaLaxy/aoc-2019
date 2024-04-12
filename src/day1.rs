#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|tok| {
            let module = tok.trim().parse::<i32>().unwrap();
            module / 3 - 2
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .map(|tok| {
            let mass = tok.trim().parse::<i32>().unwrap();
            let mut fuel = mass / 3 - 2;
            let mut sum = 0;
            while fuel >= 0 {
                sum += fuel;
                fuel = fuel / 3 - 2;
            }
            sum
        })
        .sum()
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

    // Part 2
    #[test]
    fn example6() {
        assert_eq!(solve_part2("12"), 2)
    }

    #[test]
    fn example7() {
        assert_eq!(solve_part2("14"), 2)
    }

    #[test]
    fn example8() {
        assert_eq!(solve_part2("1969"), 966)
    }

    #[test]
    fn example9() {
        assert_eq!(solve_part2("100756"), 50346)
    }

    #[test]
    fn example10() {
        assert_eq!(solve_part2("12\n14\n1969\n100756"), [2,2,966,50346].iter().sum())
    }
}