use itertools::Itertools;

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let layers = to_layers(input, 25, 6);
    let layer_check = layers
        .iter()
        .map(|layer| layer.concat())
        .reduce(|acc, layer| {
            if layer.iter().filter(|&n| *n == 0).count() <
                acc.iter().filter(|&n| *n == 0).count() {
                layer
            } else {
                acc
            }
        })
        .unwrap();

    let ones: i32 = layer_check.iter().filter(|&n| *n == 1).count() as i32;
    let twos: i32 = layer_check.iter().filter(|&n| *n == 2).count() as i32;

    ones * twos
}

fn to_layers(input: &str, width: i32, height: i32) -> Vec<Vec<Vec<i32>>> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect_vec()
        .chunks((width * height) as usize)
        .map(|layer| layer.to_vec())
        .into_iter()
        .map(|layer| {
            layer
                .chunks(width as usize)
                .map(|row| row.to_vec())
                .collect_vec()
        })
        .collect()
}

#[aoc(day8, part2)]
pub fn solve_part2(_input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let layers = to_layers("123456789012", 3, 2);
        let layer_check = layers
            .iter()
            .map(|layer| layer.concat())
            .reduce(|acc, layer| {
                if layer.iter().filter(|&n| *n == 0).count() <
                    acc.iter().filter(|&n| *n == 0).count() {
                    layer
                } else {
                    acc
                }
            })
            .unwrap();

        assert_eq!(layer_check, vec![1,2,3,4,5,6]);
    }
}