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
pub fn solve_part2(input: &str) -> String {
    let image = to_layers(input, 25, 6)
        .iter()
        .fold(vec![], |img: Vec<i32>, layer| {
            let flat = layer
                .iter()
                .flatten()
                .map(|&x| x);

            if img.len() == 0 {
                flat
                    .collect_vec()
            } else {
                flat
                    .zip(img)
                    .map(|(p_layer, p_image)| {
                        if p_image == 2 {
                            p_layer
                        } else {
                            p_image
                        }
                    })
                    .collect_vec()
            }
        });

    image
        .iter()
        .map(|&n| if n == 1 { '#' } else { '.' })
        .enumerate()
        .flat_map(|(i, c)| {
            if i % 25 == 0 {
                Some('\n')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>()
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

    #[test]
    fn example2() {
        let image = to_layers("0222112222120000", 2, 2)
            .iter()
            .fold(vec![], |img: Vec<i32>, layer| {
                let flat = layer
                    .iter()
                    .flatten()
                    .map(|&x| x);

                if img.len() == 0 {
                    flat
                        .collect_vec()
                } else {
                    flat
                        .zip(img)
                        .map(|(p_layer, p_image)| {
                            if p_image == 2 {
                                p_layer
                            } else {
                                p_image
                            }
                        })
                        .collect_vec()
                }

            });

        assert_eq!(image.iter().map(|n| n.to_string()).join(""), "0110");
    }
}