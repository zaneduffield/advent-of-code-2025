use std::ops::RangeInclusive;

use nom::{Parser, character::complete::*, multi::*, sequence::separated_pair};

pub struct Input {
    ranges: Vec<RangeInclusive<u64>>,
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let (input, ranges) = separated_list0(
        char(','),
        separated_pair(u64, char('-'), u64).map(|(start, end)| start..=end),
    )
    .parse(input)?;
    Ok((input, Input { ranges }))
}

pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

fn parse_digits(mut id: u64) -> [u8; 32] {
    let mut digits = [0u8; 32];
    let mut idx = 0;
    while id != 0 {
        digits[idx] = (id % 10) as u8;
        idx += 1;
        id /= 10;
    }
    digits
}

fn is_invalid_id1(id: u64) -> bool {
    let num_digits = (id.ilog10() + 1) as usize;
    if !num_digits.is_multiple_of(2) {
        return false;
    }

    assert!(num_digits <= 32);
    let digits = parse_digits(id);
    digits[..num_digits / 2] == digits[num_digits / 2..num_digits]
}

pub fn part_1(input: &Input) -> u64 {
    input
        .ranges
        .iter()
        .flat_map(|r| r.clone())
        .filter(|id| is_invalid_id1(*id))
        .sum()
}

fn is_invalid_id2(id: u64) -> bool {
    let num_digits = (id.ilog10() + 1) as usize;
    assert!(num_digits <= 32);
    let digits = parse_digits(id);

    'outer: for width in 1..=num_digits / 2 {
        if !num_digits.is_multiple_of(width) {
            continue;
        }
        let pattern = &digits[..width];
        let mut start = width;
        while start < num_digits {
            if &digits[start..start + width] != pattern {
                continue 'outer;
            }

            start += width;
        }
        return true;
    }

    false
}

pub fn part_2(input: &Input) -> u64 {
    input
        .ranges
        .iter()
        .flat_map(|r| r.clone())
        .filter(|id| is_invalid_id2(*id))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        assert!(is_invalid_id1(1188511885));
        assert!(!is_invalid_id1(1188511886));
        assert!(!is_invalid_id1(1188521885));

        let input = input_generator(indoc! {
            "
            11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
            1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
            824824821-824824827,2121212118-2121212124
            "
        });
        assert_eq!(part_1(&input), 1227775554);
        assert_eq!(part_2(&input), 4174379265);
    }

    #[cfg(input_exists)]
    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2025/day2.txt"));
        assert_eq!(part_1(&input), 53420042388);
        assert_eq!(part_2(&input), 69553832684);
    }
}
