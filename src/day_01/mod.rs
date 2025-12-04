use nom::{character::complete::*, multi::*, Parser};

pub struct Input {
    instructions: Vec<(Dir, u16)>,
}

enum Dir {
    Left,
    Right,
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let (input, instructions) = separated_list0(
        line_ending,
        (one_of("LR"), u16)
            .map(|(dir, len)| (if dir == 'L' { Dir::Left } else { Dir::Right }, len)),
    )
    .parse(input)?;

    Ok((input, Input { instructions }))
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

const INIT: i16 = 50;

#[aoc(day1, part1)]
pub fn part_1(input: &Input) -> u32 {
    input
        .instructions
        .iter()
        .fold((INIT, 0), |(mut dial, mut count), inst| {
            dial = match inst.0 {
                Dir::Left => dial.wrapping_sub_unsigned(inst.1),
                Dir::Right => dial.wrapping_add_unsigned(inst.1),
            };

            dial = dial.rem_euclid(100);

            count += (dial == 0) as u32;
            (dial, count)
        })
        .1
}

#[aoc(day1, part2)]
pub fn part_2(input: &Input) -> u32 {
    input
        .instructions
        .iter()
        .fold((INIT, 0), |(mut dial, mut count), inst| {
            dial = match inst.0 {
                Dir::Left => {
                    if dial == 0 {
                        count -= 1;
                    }

                    dial.wrapping_sub_unsigned(inst.1)
                }
                Dir::Right => dial.wrapping_add_unsigned(inst.1),
            };

            count += dial.div_euclid(100).unsigned_abs() as u32;
            dial = dial.rem_euclid(100);
            count += (matches!(inst.0, Dir::Left) && dial == 0) as u32;

            (dial, count)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
            "
        });
        assert_eq!(part_1(&input), 3);
        assert_eq!(part_2(&input), 6);

        let input = input_generator(indoc! {
            "
            L150
            "
        });
        assert_eq!(part_2(&input), 2);
    }

    #[cfg(input_exists)]
    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2025/day1.txt"));
        assert_eq!(part_1(&input), 1018);
        assert_eq!(part_2(&input), 5815);
    }
}
