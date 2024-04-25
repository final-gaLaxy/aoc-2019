use std::{cmp, collections::VecDeque};
use itertools::Itertools;

struct Amplifier {
    int_code: Vec<i32>,
    pointer: usize,
    inputs: VecDeque<i32>,
}

impl Amplifier {
    fn new(int_code: &Vec<i32>, phase: i32) -> Amplifier {
        Amplifier {
            int_code: int_code.clone(),
            pointer: 0,
            inputs: VecDeque::from([phase]),
        }
    }

    fn mode_to_pointer(&self, mode: i32, offset: usize) -> usize {
        match mode {
            0 => self.int_code[self.pointer + offset] as usize,
            1 => self.pointer + offset,
            _ => usize::MAX,
        }
    }

    fn next_output(&mut self, input: i32) -> Option<i32> {
        self.inputs.push_back(input);
        self.solve_with_inputs()
    }

    fn solve_with_inputs(&mut self) -> Option<i32> {
        while self.pointer < self.int_code.len() {
            let mut inst = self.int_code[self.pointer];

            let op_code = inst % 100;
            inst /= 100;

            match op_code {
                99 => break,
                1 => { // Addition
                    let p1: usize = self.mode_to_pointer(inst%10, 1);
                    inst /= 10;
                    let p2: usize = self.mode_to_pointer(inst%10, 2);
                    let p3: usize = self.mode_to_pointer(0, 3);
                    self.int_code[p3] = self.int_code[p1] + self.int_code[p2];
                    self.pointer += 4;
                },
                2 => { // Multiplication
                    let p1: usize = self.mode_to_pointer(inst%10, 1);
                    inst /= 10;
                    let p2: usize = self.mode_to_pointer(inst%10, 2);
                    let p3: usize = self.mode_to_pointer(0, 3);
                    self.int_code[p3] = self.int_code[p1] * self.int_code[p2];
                    self.pointer += 4;
                },
                3 => { // Input
                    let addr: usize = self.mode_to_pointer(inst%10, 1);
                    self.int_code[addr] = self.inputs.pop_front().unwrap();
                    self.pointer += 2;
                },
                4 => { // Output
                    let addr: usize = self.mode_to_pointer(inst%10, 1);
                    self.pointer += 2;
                    return Some(self.int_code[addr])
                },
                5 => { // jump-if-true
                    let p1: usize = self.mode_to_pointer(inst%10, 1);
                    inst /= 10;
                    let p2: usize = self.mode_to_pointer(inst%10, 2);

                    if self.int_code[p1] != 0 {
                        self.pointer = self.int_code[p2] as usize;
                    } else {
                        self.pointer += 3;
                    }
                },
                6 => { // jump-if-false
                    let p1: usize = self.mode_to_pointer(inst%10, 1);
                    inst /= 10;
                    let p2: usize = self.mode_to_pointer(inst%10, 2);

                    if self.int_code[p1] == 0 {
                        self.pointer = self.int_code[p2] as usize;
                    } else {
                        self.pointer += 3;
                    }
                },
                7 => { // less than
                    let p1: usize = self.mode_to_pointer(inst%10, 1);
                    inst /= 10;
                    let p2: usize = self.mode_to_pointer(inst%10, 2);
                    let p3: usize = self.mode_to_pointer(0, 3);

                    self.int_code[p3] = if self.int_code[p1] < self.int_code[p2] { 1 } else { 0 };

                    self.pointer += 4;
                },
                8 => { // equal to
                    let p1: usize = self.mode_to_pointer(inst%10, 1);
                    inst /= 10;
                    let p2: usize = self.mode_to_pointer(inst%10, 2);
                    let p3: usize = self.mode_to_pointer(0, 3);

                    self.int_code[p3] = if self.int_code[p1] == self.int_code[p2] { 1 } else { 0 };

                    self.pointer += 4;
                }
                _ => {}
            }
        }

        None
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
        for phase in perm {
            amplifiers.push(Amplifier::new(int_code, phase));
        }

        let mut output = 0;
        for amp in amplifiers.into_iter() {
            output = amp
                .next_output(output)
                .unwrap();
        }
        max = cmp::max(max, output);
    }

    max
}

#[aoc(day7, part2)]
pub fn solve_part2(int_code: &Vec<i32>) -> i32 {
    let mut max = 0;
    for perm in (5..=9).permutations(5) {
        let amplifiers: &mut Vec<Amplifier> = &mut vec![];
        for phase in perm {
            amplifiers.push(Amplifier::new(int_code, phase));
        }

        let mut amp_in = Some(0);
        let mut output = 0;

        while !amp_in.is_none() {
            for amp in amplifiers.into_iter() {
                amp_in = amp
                    .next_output(amp_in.unwrap());

                if amp_in.is_none() { break };
            }

            output = amp_in.unwrap_or(output);
        }

        max = cmp::max(max, output);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    // Part 1
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
            solve_part1(&input_generator("3,23,3,24,1002,24,10,24,1002,23,-1,23,\
            101,5,23,23,1,24,23,23,4,23,99,0,0")),
            54321
        )
    }

    #[test]
    fn example3() {
        assert_eq!(
            solve_part1(&input_generator("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,\
            1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0")),
            65210
        )
    }

    // Part 2
    #[test]
    fn example4() {
        assert_eq!(
            solve_part2(&input_generator("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
            27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5")),
            139629729
        )
    }

    #[test]
    fn example5() {
        assert_eq!(
            solve_part2(&input_generator("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
            -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
            53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10")),
            18216
        )
    }
}