use std::{cmp, collections::VecDeque};
use itertools::Itertools;

struct Amplifier {
    int_code: Vec<i32>,
    halted: bool,
}

impl Amplifier {
    fn new(int_code: &Vec<i32>) -> Amplifier {
        Amplifier {
            int_code: int_code.clone(),
            halted: false
        }
    }

    fn solve_with_inputs(&mut self, mut inputs: VecDeque<i32>) -> Vec<i32> {
        let mut outputs: Vec<i32> = Vec::new();
        let mut p = 0;
        while p < self.int_code.len() {
            let mut inst = self.int_code[p];

            let op_code = inst % 100;
            inst /= 100;

            match op_code {
                99 => break,
                1 => { // Addition
                    let p1: usize = if inst%10 == 1 { p+1 } else { self.int_code[p+1] as usize };
                    inst /= 10;
                    let p2: usize = if inst%10 == 1 { p+2 } else { self.int_code[p+2] as usize };
                    let p3: usize = self.int_code[p+3] as usize;
                    self.int_code[p3] = self.int_code[p1] + self.int_code[p2];
                    p += 4;
                },
                2 => { // Multiplication
                    let p1: usize = if inst%10 == 1 { p+1 } else { self.int_code[p+1] as usize };
                    inst /= 10;
                    let p2: usize = if inst%10 == 1 { p+2 } else { self.int_code[p+2] as usize };
                    let p3: usize = self.int_code[p+3] as usize;
                    self.int_code[p3] = self.int_code[p1] * self.int_code[p2];
                    p += 4;
                },
                3 => { // Input
                    let addr: usize = self.int_code[p+1] as usize;
                    self.int_code[addr] = inputs.pop_front().unwrap();
                    p += 2;
                },
                4 => { // Output
                    let addr: usize = if inst%10 == 1 { p+1 } else { self.int_code[p+1] as usize };
                    outputs.push(self.int_code[addr]);
                    p += 2;
                },
                5 => { // jump-if-true
                    let p1: usize = if inst%10 == 1 { p+1 } else { self.int_code[p+1] as usize };
                    inst /= 10;
                    let p2: usize = if inst%10 == 1 { p+2 } else { self.int_code[p+2] as usize };

                    if self.int_code[p1] != 0 {
                        p = self.int_code[p2] as usize;
                    } else {
                        p += 3;
                    }
                },
                6 => { // jump-if-false
                    let p1: usize = if inst%10 == 1 { p+1 } else { self.int_code[p+1] as usize };
                    inst /= 10;
                    let p2: usize = if inst%10 == 1 { p+2 } else { self.int_code[p+2] as usize };

                    if self.int_code[p1] == 0 {
                        p = self.int_code[p2] as usize;
                    } else {
                        p += 3;
                    }
                },
                7 => { // less than
                    let p1: usize = if inst%10 == 1 { p+1 } else { self.int_code[p+1] as usize };
                    inst /= 10;
                    let p2: usize = if inst%10 == 1 { p+2 } else { self.int_code[p+2] as usize };
                    let p3: usize = self.int_code[p+3] as usize;

                    self.int_code[p3] = if self.int_code[p1] < self.int_code[p2] { 1 } else { 0 };

                    p += 4;
                },
                8 => { // equal to
                    let p1: usize = if inst%10 == 1 { p+1 } else { self.int_code[p+1] as usize };
                    inst /= 10;
                    let p2: usize = if inst%10 == 1 { p+2 } else { self.int_code[p+2] as usize };
                    let p3: usize = self.int_code[p+3] as usize;

                    self.int_code[p3] = if self.int_code[p1] == self.int_code[p2] { 1 } else { 0 };

                    p += 4;
                }
                _ => {}
            }
        }

        outputs
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|d| d.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(int_code: &Vec<i32>) -> i32 {
    let mut max = 0;
    for perm in (0..=4).permutations(5) {
        let amplifiers: &mut Vec<Amplifier> = &mut vec![];
        for _ in 0..5 {
            amplifiers.push(Amplifier::new(int_code));
        }

        let mut output = 0;
        for (amp, input) in perm.iter().enumerate() {
            output = amplifiers
                .get_mut(amp)
                .unwrap()
                .solve_with_inputs(VecDeque::from([*input, output]))[0];
        }
        max = cmp::max(max, output);
    }

    max
}

#[aoc(day7, part2)]
pub fn solve_part2(_input: &Vec<i32>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(&input_generator("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")),
            43210
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part1(&input_generator("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")),
            54321
        )
    }

    #[test]
    fn example3() {
        assert_eq!(
            solve_part1(&input_generator("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0")),
            65210
        )
    }
}