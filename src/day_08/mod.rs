use std::cmp::Reverse;

pub type Point3 = (u64, u64, u64);
pub type Input = Vec<Point3>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut nums = line.split(',').map(|num| num.parse::<u64>().unwrap());
            (
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            )
        })
        .collect()
}

fn square(n: u64) -> u64 {
    n * n
}

pub fn solve1(input: &Input, limit: u32) -> (u64, u64) {
    let mut circuits: Vec<Vec<&Point3>> = vec![];
    let mut dists: Vec<(u64, &Point3, &Point3)> = input
        .iter()
        .enumerate()
        .flat_map(|(idx, point)| {
            input.iter().skip(idx + 1).map(move |p2| {
                (
                    square(point.0.abs_diff(p2.0))
                        + square(point.1.abs_diff(p2.1))
                        + square(point.2.abs_diff(p2.2)),
                    point,
                    p2,
                )
            })
        })
        .collect();

    dists.sort_unstable_by_key(|x| Reverse(x.0));

    let mut last_joined = None;

    'outer: for _ in 0..limit {
        let Some((_, p1, p2)) = dists.pop() else {
            break;
        };

        for (c_idx, c) in circuits.iter_mut().enumerate() {
            let mut to_merge = None;
            if c.contains(&p1) {
                to_merge = Some(p2);
            } else if c.contains(&p2) {
                to_merge = Some(p1);
            }

            if let Some(to_merge) = to_merge {
                if !c.contains(&to_merge) {
                    let mut other_circuit = None;
                    for (c2_idx, c2) in circuits.iter().enumerate() {
                        if c2.contains(&to_merge) {
                            other_circuit = Some(c2_idx);
                            break;
                        }
                    }

                    if let Some(c2_idx) = other_circuit {
                        let [c1, c2] = circuits.get_disjoint_mut([c_idx, c2_idx]).unwrap();
                        c1.append(c2);
                    } else {
                        circuits[c_idx].push(to_merge);
                    }
                    last_joined = Some((p1, p2));
                }
                continue 'outer;
            }
        }

        last_joined = Some((p1, p2));
        circuits.push(vec![p1, p2]);
    }

    circuits.sort_unstable_by_key(|c| Reverse(c.len()));
    let sol1 = circuits.iter().take(3).map(|c| c.len() as u64).product();
    let sol2 = last_joined.map(|(p1, p2)| p1.0 * p2.0).unwrap();

    (sol1, sol2)
}

pub fn part_1(input: &Input) -> u64 {
    solve1(input, 1000).0
}

pub fn part_2(input: &Input) -> u64 {
    solve1(input, u32::MAX).1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            162,817,812
            57,618,57
            906,360,560
            592,479,940
            352,342,300
            466,668,158
            542,29,236
            431,825,988
            739,650,466
            52,470,668
            216,146,977
            819,987,18
            117,168,530
            805,96,715
            346,949,466
            970,615,88
            941,993,340
            862,61,35
            984,92,344
            425,690,689
            "
        });
        assert_eq!(solve1(&input, 10).0, 40);
        assert_eq!(part_2(&input), 25272);
    }

    #[cfg(input_exists)]
    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2025/day8.txt"));
        assert_eq!(part_1(&input), 121770);
        assert_eq!(part_2(&input), 7893123992);
    }
}
