use std::collections::HashSet;

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut asteroids: Vec<(i32, i32)> = Vec::new();
    for (y, l) in input.trim().lines().enumerate() {
        for (x, char) in l.chars().enumerate() {
            if char == '#' {
                asteroids.push((x as i32, y as i32));
            }
        }
    }

    println!("{:?}", asteroids.len());

    let mut max = 0;
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
        }
    }

    max
}

pub fn gcd(mut n: i32, mut m: i32) -> i32 {
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
pub fn solve_part2(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(
                ".#..#\
                \n.....\
                \n#####\
                \n....#\
                \n...##"
            ),
            8
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part1(
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
            ),
            33
        )
    }

    #[test]
    fn example3() {
        assert_eq!(
            solve_part1(
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
            ),
            35
        )
    }

    #[test]
    fn example4() {
        assert_eq!(
            solve_part1(
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
            ),
            41
        )
    }

    #[test]
    fn example5() {
        assert_eq!(
            solve_part1(
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
            ),
            210
        )
    }
}