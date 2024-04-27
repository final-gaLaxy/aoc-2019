use std::collections::VecDeque;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|d| d.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(int_code: &Vec<i64>) -> i64 {
    let mut runner = IntcodeRunner::new(int_code);
    runner.solve_with_inputs(VecDeque::from([1]))[0]
}

#[aoc(day9, part2)]
pub fn solve_part2(_input: &Vec<i64>) -> i64 {
    0
}

struct IntcodeRunner {
    int_code: Vec<i64>,
    pointer: usize,
    inputs: VecDeque<i64>,
    rel_base: i64,
}

impl IntcodeRunner {
    fn new(int_code: &Vec<i64>) -> IntcodeRunner {
        IntcodeRunner {
            int_code: int_code.clone(),
            pointer: 0,
            inputs: VecDeque::new(),
            rel_base: 0,
        }
    }

    fn ensure_size(&mut self, len: usize) {
        if len < self.int_code.len() {
            return
        }

        self.int_code.resize(len, 0);
    }

    fn mode_to_pointer(&mut self, mode: i64, offset: usize) -> usize {
        let p: usize = match mode {
            0 => self.int_code[self.pointer + offset] as usize,
            1 => self.pointer + offset,
            2 => (self.int_code[self.pointer + offset] + self.rel_base) as usize,
            _ => usize::MAX,
        };

        self.ensure_size(p + 1);
        p
    }

    fn solve_with_inputs(&mut self, inputs: VecDeque<i64>) -> Vec<i64> {
        self.inputs.extend(inputs);
        let mut outputs: Vec<i64> = Vec::new();
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
                    inst /= 10;
                    let p3: usize = self.mode_to_pointer(inst%10, 3);
                    self.int_code[p3] = self.int_code[p1] + self.int_code[p2];
                    self.pointer += 4;
                },
                2 => { // Multiplication
                    let p1: usize = self.mode_to_pointer(inst%10, 1);
                    inst /= 10;
                    let p2: usize = self.mode_to_pointer(inst%10, 2);
                    inst /= 10;
                    let p3: usize = self.mode_to_pointer(inst%10, 3);
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
                    outputs.push(self.int_code[addr]);
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
                    inst /= 10;
                    let p3: usize = self.mode_to_pointer(inst%10, 3);

                    self.int_code[p3] = if self.int_code[p1] < self.int_code[p2] { 1 } else { 0 };

                    self.pointer += 4;
                },
                8 => { // equal to
                    let p1: usize = self.mode_to_pointer(inst%10, 1);
                    inst /= 10;
                    let p2: usize = self.mode_to_pointer(inst%10, 2);
                    inst /= 10;
                    let p3: usize = self.mode_to_pointer(inst%10, 3);

                    self.int_code[p3] = if self.int_code[p1] == self.int_code[p2] { 1 } else { 0 };

                    self.pointer += 4;
                },
                9 => { // adjust rel base
                    let p1: usize = self.mode_to_pointer(inst%10, 1);

                    self.rel_base += self.int_code[p1];

                    self.pointer += 2;
                },
                _ => {}
            }
        }

        outputs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let int_code = input_generator("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let mut runner = IntcodeRunner::new(&int_code);

        assert_eq!(runner.solve_with_inputs(VecDeque::from([1])), int_code);
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part1(&input_generator("1102,34915192,34915192,7,4,7,99,0")).ilog10() + 1,
            16
        )
    }

    #[test]
    fn example3() {
        assert_eq!(
            solve_part1(&input_generator("104,1125899906842624,99")),
            1125899906842624
        )
    }
}