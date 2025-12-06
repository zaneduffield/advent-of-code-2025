use std::ops::RangeInclusive;

use nom::{Parser, character::complete::*, multi::*, sequence::separated_pair};

pub struct Input {
    fresh_ranges: Vec<RangeInclusive<u64>>,
    ingredients: Vec<u64>,
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let (input, fresh_ranges) = separated_list1(
        line_ending,
        separated_pair(u64, char('-'), u64).map(|(start, end)| start..=end),
    )
    .parse(input)?;

    let (input, ingredients) = separated_list1(line_ending, u64).parse(input.trim())?;

    Ok((
        input,
        Input {
            fresh_ranges,
            ingredients,
        },
    ))
}

pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

pub fn part_1(input: &Input) -> u32 {
    input
        .ingredients
        .iter()
        .filter(|i| input.fresh_ranges.iter().any(|r| r.contains(i)))
        .count() as u32
}

pub fn part_2(input: &Input) -> u64 {
    let mut disjoint_ranges: Vec<RangeInclusive<u64>> = vec![];
    for r in &input.fresh_ranges {
        if let Some((pos, r2)) = disjoint_ranges
            .iter_mut()
            .enumerate()
            .find(|(_, r2)| *r.start() <= *r2.end())
        {
            let (merge_left, merge_right) = if r.contains(r2.start()) && r.contains(r2.end()) {
                // new interval completely contains old interval; need to merge on both sides
                (true, true)
            } else if r2.contains(r.start()) && r2.contains(r.end()) {
                // old interval completely contains new interval; ignore
                continue;
            } else if *r.start() < *r2.start() {
                // new interval intersects with the start of an old interval.
                // need to merge on the left
                (true, false)
            } else {
                // new interval intersects with the end of an old interval.
                // need to merge on the right
                (false, true)
            };

            if merge_right {
                if *r2.end() >= *r.start() {
                    *r2 = *r2.start()..=*r.start() - 1;
                }

                for r3 in &mut disjoint_ranges[pos + 1..] {
                    if *r3.end() <= *r.end() {
                        *r3 = *r3.start()..=*r3.start() - 1;
                        continue;
                    } else if *r3.start() <= *r.end() {
                        *r3 = (*r.end() + 1).max(*r3.start())..=*r3.end();
                    }
                    break;
                }
            }

            // reborrow
            let r2 = &mut disjoint_ranges[pos];
            let mut insert_pos = pos + 1;

            if merge_left {
                insert_pos = pos;

                if *r2.start() <= *r.end() {
                    *r2 = *r.end() + 1..=*r2.end();
                }

                for r3 in disjoint_ranges[..pos].iter_mut().rev() {
                    if *r3.start() >= *r.start() {
                        *r3 = *r3.start()..=*r3.start() - 1;
                        continue;
                    } else if *r3.end() >= *r.start() {
                        *r3 = *r3.start()..=(*r.start() - 1).min(*r3.end());
                    }
                    break;
                }
            }

            disjoint_ranges.insert(insert_pos, r.clone());
        } else {
            disjoint_ranges.push(r.clone());
        }
    }

    disjoint_ranges.iter().map(|r| r.size_hint().0 as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
            "
        });
        assert_eq!(part_1(&input), 3);
        assert_eq!(part_2(&input), 14);

        let input = input_generator(indoc! {
            "
            1-3
            5-9
            10-12
            8-11
            2-6

            1
            "
        });
        assert_eq!(part_2(&input), 12);

        let input = input_generator(indoc! {
            "
            1-3
            5-9
            10-12
            8-11
            1-11
            11-15

            1
            "
        });
        assert_eq!(part_2(&input), 15);

        let input = input_generator(indoc! {
            "
            8-10
            1-3

            1
            "
        });
        assert_eq!(part_2(&input), 6);
    }

    #[cfg(input_exists)]
    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2025/day5.txt"));
        assert_eq!(part_1(&input), 737);
        assert_eq!(part_2(&input), 357485433193284);
    }
}
