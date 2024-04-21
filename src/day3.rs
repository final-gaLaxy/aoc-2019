use std::{
    cmp,
    collections::{HashMap, HashSet}
};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> [Vec<(char, i32)>; 2] {
    input
        .trim()
        .lines()
        .map(|line| {
            line
                .split(',')
                .map(|d| {
                    let (dir, dist) = d.split_at(1);
                    (dir.chars().next().unwrap(), dist.parse::<i32>().unwrap())
                })
                .collect::<Vec<(char, i32)>>()
        })
        .collect::<Vec<Vec<(char, i32)>>>()
        .try_into()
        .unwrap()
}

fn populate_path(paths: Vec<(char, i32)>) -> (HashSet<(i32, i32)>, HashMap<(i32, i32), i32>) {
    let mut coords: HashSet<(i32, i32)> = HashSet::<(i32, i32)>::new();
    let mut coord_step: HashMap<(i32, i32), i32> = HashMap::<(i32, i32), i32>::new();
    let mut cur = (0,0);
    let mut steps = 1;
    for (dir, dist) in paths.iter() {
        let mut new_point = (0,0);
        let mut r = *dist;
        match dir {
            'R' => new_point.0 += 1,
            'L' => new_point.0 -= 1,
            'U' => new_point.1 += 1,
            'D' => new_point.1 -= 1,
            _ => ()
        }

        while r > 0 {
            cur = (cur.0 + new_point.0, cur.1 + new_point.1);
            coords.insert(cur);
            coord_step.insert(cur, steps);

            r -= 1;
            steps += 1;
        }
    }


    (coords, coord_step)
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<(char, i32)>;2]) -> i32 {
    let (p1, _) = populate_path(input[0].clone());
    let (p2, _) = populate_path(input[1].clone());

    let intersection = p1.intersection(&p2);

    intersection
        .fold(i32::MAX, |min_dist, x|
            cmp::min(min_dist, x.0.abs() + x.1.abs())
        )
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<(char, i32)>;2]) -> i32 {
    let (p1, p1_steps) = populate_path(input[0].clone());
    let (p2, p2_steps) = populate_path(input[1].clone());

    let intersection = p1.intersection(&p2);

    intersection
        .fold(i32::MAX, |min_dist, x|
            cmp::min(
                min_dist,
                p1_steps.get(x).unwrap() +
                    p2_steps.get(x).unwrap()
            )
        )
}

mod tests {
    use super::*;

    // Part 1
    #[test]
    fn example1() {
        assert_eq!(solve_part1(
            &input_generator(
                "R8,U5,L5,D3\nU7,R6,D4,L4"
            )
        ),
        6);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part1(
            &input_generator(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )
        ),
        159);
    }

    #[test]
    fn example3() {
        assert_eq!(solve_part1(
            &input_generator(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
        ),
        135);
    }

    // Part 2
    #[test]
    fn example4() {
        assert_eq!(solve_part2(
            &input_generator(
                "R8,U5,L5,D3\nU7,R6,D4,L4"
            )
        ),
        30);
    }

    #[test]
    fn example5() {
        assert_eq!(solve_part2(
            &input_generator(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )
        ),
        610);
    }

    #[test]
    fn example6() {
        assert_eq!(solve_part2(
            &input_generator(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
        ),
        410);
    }
}