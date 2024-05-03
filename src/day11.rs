use std::collections::{HashSet, VecDeque};

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|d| d.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(int_code: &Vec<i64>) -> usize {
    let dirs = [(0,1), (1,0), (0,-1), (-1,0)];
    let mut painted: HashSet<(i64, i64)> = HashSet::new();
    let mut is_white: HashSet<(i64, i64)> = HashSet::new();
    let mut dir = 0;
    let mut pos: (i64, i64) = (0, 0);
    let mut runner = IntcodeRunner::new(int_code);
    while let Some(colour) = runner.next_output( if is_white.contains(&pos) { Some(1) } else { Some(0) }) {
        if colour == 1 {
            is_white.insert(pos);
        } else {
            is_white.remove(&pos);
        }
        painted.insert(pos);

        let turn_right = runner.next_output(None).unwrap();

        dir = (turn_right*2-1 + dir).rem_euclid(dirs.len() as i64);
        pos = (pos.0 + dirs[dir as usize].0, pos.1 + dirs[dir as usize].1);
    }

    painted.len()
}

#[aoc(day11, part2)]
pub fn solve_part2(_input: &Vec<i64>) -> i32 {
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

    fn next_output(&mut self, input: Option<i64>) -> Option<i64> {
        if input.is_some() {
            self.inputs.push_back(input.unwrap());
        }
        self.solve_with_inputs()
    }

    fn solve_with_inputs(&mut self) -> Option<i64> {
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

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}