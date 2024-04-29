use std::collections::HashSet;

use itertools::Itertools;

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<(i32, i32)> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| {
                    c == '#'
                })
                .map(|(x, _)| (x as i32, y as i32))
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .collect()
}


#[aoc(day10, part1)]
pub fn solve_part1(asteroids: &Vec<(i32, i32)>) -> usize {
    get_best_asteroid_w_max(asteroids.clone()).1
}

fn get_best_asteroid_w_max(asteroids: Vec<(i32, i32)>) -> (
    (i32, i32),
    usize,
 ) {
    let mut max = 0;
    let mut best: Option<(i32, i32)> = None;
    for &asteroid in asteroids.iter() {
        let angles: HashSet<(i32, i32)> = HashSet::from_iter(
            asteroids.clone()
                .iter()
                .filter(|&a| a != &asteroid)
                .map(|(x, y)| {
                    let (x_rel, y_rel) = (x - asteroid.0, y - asteroid.1);

                    if x_rel == 0 || y_rel == 0 {
                        return (x_rel.signum(), y_rel.signum());
                    }

                    let _gcd = gcd(x_rel.abs(), y_rel.abs());
                    (x_rel / _gcd, y_rel / _gcd)
                })
            );

        if angles.len() > max {
            max = angles.len();
            best = Some(asteroid);
        }
    }

    (best.unwrap(), max)
}

fn gcd(mut n: i32, mut m: i32) -> i32 {
    while m != 0 {
      if m < n {
        std::mem::swap(&mut m, &mut n);
      }
      if n == 0 {
      }
      m %= n;
    }
    n
  }

#[aoc(day10, part2)]
pub fn solve_part2(asteroids: &Vec<(i32, i32)>) -> i32 {
    let base: (i32, i32) = get_best_asteroid_w_max(asteroids.clone()).0;

    let mut asteroids_by_angle: Vec<(f64, i32, (i32, i32))> = get_asteroid_vals(asteroids.clone(), base);
    let mut last_asteroid = asteroids_by_angle.remove(0);

    'outer: for _ in 0..199 {
        for (idx, asteroid) in asteroids_by_angle.iter().enumerate() {
            if asteroid.0 < last_asteroid.0 {
                last_asteroid = asteroids_by_angle.remove(idx);
                continue 'outer;
            }
        }
        last_asteroid = asteroids_by_angle.remove(0);
    }

    last_asteroid.2.0 * 100 + last_asteroid.2.1
}

fn get_asteroid_vals(asteroids: Vec<(i32, i32)>, base: (i32, i32)) -> Vec<(f64, i32, (i32, i32))> {
    asteroids
        .iter()
        .filter(|&a| a != &base)
        .into_iter()
        .map(|&a| {
            let (rel_x, rel_y) = (a.0 - base.0, a.1 - base.1);
            let angle = f64::atan2(rel_x as f64, rel_y as f64);

            (angle, (rel_x.abs() + rel_y.abs()), a) // angle, manhatten dist, a
        })
        .sorted_by(|&a, &b| {
            a.0.partial_cmp(&b.0).unwrap().reverse().then(a.1.cmp(&b.1))
        })
        .collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    // Part 1
    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(
                &input_generator(
                    ".#..#\
                    \n.....\
                    \n#####\
                    \n....#\
                    \n...##"
                )
            ),
            8
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part1(
                &input_generator(
                    "......#.#.\
                    \n#..#.#....\
                    \n..#######.\
                    \n.#.#.###..\
                    \n.#..#.....\
                    \n..#....#.#\
                    \n#..#....#.\
                    \n.##.#..###\
                    \n##...#..#.\
                    \n.#....####"
                )
            ),
            33
        )
    }

    #[test]
    fn example3() {
        assert_eq!(
            solve_part1(
                &input_generator(
                    "#.#...#.#.\
                    \n.###....#.\
                    \n.#....#...\
                    \n##.#.#.#.#\
                    \n....#.#.#.\
                    \n.##..###.#\
                    \n..#...##..\
                    \n..##....##\
                    \n......#...\
                    \n.####.###."
                )
            ),
            35
        )
    }

    #[test]
    fn example4() {
        assert_eq!(
            solve_part1(
                &input_generator(
                    ".#..#..###\
                    \n####.###.#\
                    \n....###.#.\
                    \n..###.##.#\
                    \n##.##.#.#.\
                    \n....###..#\
                    \n..#.#..#.#\
                    \n#..#.#.###\
                    \n.##...##.#\
                    \n.....#.#.."
                )
            ),
            41
        )
    }

    #[test]
    fn example5() {
        assert_eq!(
            solve_part1(
                &input_generator(
                    ".#..##.###...#######\
                    \n##.############..##.\
                    \n.#.######.########.#\
                    \n.###.#######.####.#.\
                    \n#####.##.#.##.###.##\
                    \n..#####..#.#########\
                    \n####################\
                    \n#.####....###.#.#.##\
                    \n##.#################\
                    \n#####.##.###..####..\
                    \n..######..##.#######\
                    \n####.##.####...##..#\
                    \n.#####..#.######.###\
                    \n##...#.##########...\
                    \n#.##########.#######\
                    \n.####.#.###.###.#.##\
                    \n....##.##.###..#####\
                    \n.#.#.###########.###\
                    \n#.#.#.#####.####.###\
                    \n###.##.####.##.#..##"
                )
            ),
            210
        )
    }

    // Part 2
    #[test]
    fn example6() {
        let asteroids = input_generator(
            ".#....#####...#..\
            \n##...##.#####..##\
            \n##...#...#.#####.\
            \n..#.....#...###..\
            \n..#.#.....#....##"
        );
        let best = get_best_asteroid_w_max(asteroids.clone()).0;
        println!("{:?}", best);
        println!(
            "{:?}",
            get_asteroid_vals(
                asteroids.clone(),
                best
            )
        );
    }

    #[test]
    fn example7() {
        let asteroids = input_generator(
            ".#..##.###...#######\
            \n##.############..##.\
            \n.#.######.########.#\
            \n.###.#######.####.#.\
            \n#####.##.#.##.###.##\
            \n..#####..#.#########\
            \n####################\
            \n#.####....###.#.#.##\
            \n##.#################\
            \n#####.##.###..####..\
            \n..######..##.#######\
            \n####.##.####...##..#\
            \n.#####..#.######.###\
            \n##...#.##########...\
            \n#.##########.#######\
            \n.####.#.###.###.#.##\
            \n....##.##.###..#####\
            \n.#.#.###########.###\
            \n#.#.#.#####.####.###\
            \n###.##.####.##.#..##"
        );

        println!("{:?}", solve_part2(&asteroids));
    }
}