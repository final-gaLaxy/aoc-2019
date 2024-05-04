use std::collections::{BTreeMap, VecDeque};

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
    let mut robot: PaintingRobot = PaintingRobot::new(IntcodeRunner::new(int_code));
    robot.run();

    robot.panel.len()
}

#[aoc(day11, part2)]
pub fn solve_part2(_input: &Vec<i64>) -> i32 {
    0
}

#[derive(Clone, Copy)]
enum Colour {
    Black,
    White,
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Position {
        Position { x: x, y: y }
    }

    fn walk(&mut self, dir: Direction){
        match dir {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        };
    }
}

struct PaintingRobot {
    panel: BTreeMap<Position, Colour>,
    dir: Direction,
    pos: Position,
    runner: IntcodeRunner,
}

impl PaintingRobot {
    fn new(runner: IntcodeRunner) -> PaintingRobot {
        PaintingRobot {
            panel: BTreeMap::new(),
            dir: Direction::Up,
            pos: Position::new(0,0),
            runner: runner,
        }
    }

    fn run(&mut self) {
        while let Some(colour) = self.output(self.input()) {
            self.panel.insert(self.pos, colour);

            self.dir = match self.runner.next_output(None) {
                Some(0) => self.dir.turn_left(),
                Some(1) => self.dir.turn_right(),
                _ => panic!("Unknown Direction!")
            };

            self.pos.walk(self.dir);
        }
    }

    fn input(&self) -> i64 {
        match self.panel.get(&self.pos).unwrap_or(&Colour::Black) {
            Colour::Black => 0,
            Colour::White => 1,
        }
    }

    fn output(&mut self, v: i64) -> Option<Colour> {
        match self.runner.next_output(Some(v)) {
            Some(0) => Some(Colour::Black),
            Some(1) => Some(Colour::White),
            None => None,
            _ => panic!("Unkown Colour Output!")
        }
    }
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