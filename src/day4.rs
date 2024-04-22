fn is_valid(mut input: i32) -> bool {
    let mut prev_digit: i32 = input % 10;
    let mut has_double: bool = false;

    input = input / 10;

    while input > 0 {
        let cur_digit: i32 = input % 10;
        if cur_digit > prev_digit {
            return false;
        }
        if cur_digit == prev_digit {
            has_double = true;
        }
        prev_digit = cur_digit;
        input /= 10;
    }

    has_double
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let range: Vec<i32> = input
        .split('-')
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut c = 0;
    for n in range[0]..range[1] {
        c += if is_valid(n) { 1 } else { 0 };
    }

    c
}

fn is_valid_part2(mut input: i32) -> bool {
    let mut prev_digit: i32 = input % 10;
    let mut has_double: bool = false;

    input = input / 10;

    let mut digit_count = 1;

    while input > 0 {
        let cur_digit: i32 = input % 10;

        if cur_digit > prev_digit {
            return false;
        }

        if cur_digit == prev_digit {
            digit_count += 1;
        } else {
            has_double = if digit_count == 2 { true } else { has_double };
            digit_count = 1;
        }

        prev_digit = cur_digit;
        input /= 10;
    }

    has_double || digit_count == 2
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let range: Vec<i32> = input
        .split('-')
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut c = 0;
    for n in range[0]..range[1] {
        c += if is_valid_part2(n) { 1 } else { 0 };
    }

    c
}

#[cfg(test)]
mod tests {
    use super::*;

    // Part 1
    #[test]
    fn example1() {
        assert_eq!(is_valid(111111), true);
    }

    #[test]
    fn example2() {
        assert_eq!(is_valid(223450), false);
    }

    #[test]
    fn example3() {
        assert_eq!(is_valid(123789), false);
    }

    // Part 2
    #[test]
    fn example4() {
        assert_eq!(is_valid_part2(112233), true);
    }

    #[test]
    fn example5() {
        assert_eq!(is_valid_part2(123444), false);
    }

    #[test]
    fn example6() {
        assert_eq!(is_valid_part2(111122), true);
    }

    #[test]
    fn example7() {
        assert_eq!(is_valid_part2(112222), true);
    }
}