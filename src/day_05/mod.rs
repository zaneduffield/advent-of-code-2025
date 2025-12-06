use std::ops::RangeInclusive;

use nom::{Parser, character::complete::*, multi::*, sequence::separated_pair};

pub struct Input {
    sorted_fresh_ranges: Vec<RangeInclusive<u64>>,
    sorted_ingredients: Vec<u64>,
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let (input, mut fresh_ranges) = separated_list1(
        line_ending,
        separated_pair(u64, char('-'), u64).map(|(start, end)| start..=end),
    )
    .parse(input)?;

    let (input, mut ingredients) = separated_list1(line_ending, u64).parse(input.trim())?;

    fresh_ranges.sort_by_key(|r| *r.start());
    ingredients.sort();

    Ok((
        input,
        Input {
            sorted_fresh_ranges: fresh_ranges,
            sorted_ingredients: ingredients,
        },
    ))
}

pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

pub fn part_1(input: &Input) -> u32 {
    // a simple loop with a .contains check is simpler, but this is faster

    let mut total = 0;
    let mut min_idx = 0;
    for ingredient in &input.sorted_ingredients {
        while input
            .sorted_fresh_ranges
            .get(min_idx)
            .is_some_and(|r| *r.end() < *ingredient)
        {
            min_idx += 1;
        }

        for r in &input.sorted_fresh_ranges[min_idx..] {
            if *r.start() > *ingredient {
                break;
            }
            if *r.end() >= *ingredient {
                total += 1;
                break;
            }
        }
    }

    total
}

pub fn part_2(input: &Input) -> u64 {
    let mut total = 0u64;
    let mut last_included = 0u64;
    for r in &input.sorted_fresh_ranges {
        let size = 1 + *r.end() - *r.start();
        total += size;
        if *r.start() <= last_included {
            total -= (1 + last_included - *r.start()).min(size)
        }
        last_included = last_included.max(*r.end());
    }
    total
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
