use std::collections::VecDeque;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|d| d.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(int_code: &Vec<i32>) -> String {
    let outputs = solve_with_inputs(int_code.clone(), VecDeque::from([1]));
    format!("{:?}", outputs)
}

fn solve_with_inputs(mut program: Vec<i32>, mut inputs: VecDeque<i32>) -> Vec<i32> {
    let mut outputs: Vec<i32> = Vec::new();
    let mut p = 0;
    while p < program.len() {
        let mut inst = program[p];

        let op_code = inst % 100;
        inst /= 100;

        match op_code {
            99 => break,
            1 => {
                let p1: usize = if inst%10 == 1 { p+1 } else { program[p+1] as usize };
                inst /= 10;
                let p2: usize = if inst%10 == 1 { p+2 } else { program[p+2] as usize };
                let p3: usize = program[p+3] as usize;
                program[p3] = program[p1] + program[p2];
                p += 4;
            },
            2 => {
                let p1: usize = if inst%10 == 1 { p+1 } else { program[p+1] as usize };
                inst /= 10;
                let p2: usize = if inst%10 == 1 { p+2 } else { program[p+2] as usize };
                let p3: usize = program[p+3] as usize;
                program[p3] = program[p1] * program[p2];
                p += 4;
            },
            3 => {
                let addr: usize = program[p+1] as usize;
                program[addr] = inputs.pop_front().unwrap();
                p += 2;
            },
            4 => {
                let addr: usize = if inst%10 == 1 { p+1 } else { program[p+1] as usize };
                outputs.push(program[addr]);
                p += 2;
            },
            _ => {}
        }
    }

    outputs
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    // Part 1
    #[test]
    fn example1() {
        let mut rng = rand::thread_rng();
        for _ in 1..5 {
            let n: i32 = rng.gen();
            let output = solve_with_inputs(
                input_generator("3,0,4,0,99"),
                VecDeque::from([n])
            );
            assert_eq!(
                format!("{:?}",output),
                format!("[{}]", n)
            );
        }
    }

    #[test]
    fn example2() {
        let output = solve_with_inputs(
            input_generator("1002,6,3,6,4,6,33"),
            VecDeque::from([])
        );
        assert_eq!(
            format!("{:?}",output),
            "[99]"
        );
    }
}