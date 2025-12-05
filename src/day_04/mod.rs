pub struct Input<'a> {
    width: isize,
    height: isize,
    grid: &'a [u8],
}

fn parse_input(input: &str) -> nom::IResult<&str, Input<'_>> {
    let mut lines = input.lines();
    let width = lines.next().unwrap().len() as isize;
    let height = 1 + lines.count() as isize;

    Ok((
        "",
        Input {
            width,
            height,
            grid: input.as_bytes(),
        },
    ))
}

pub fn input_generator(input: &str) -> Input<'_> {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

pub fn part_1(input: &Input) -> u32 {
    let datum = |y, x| input.grid[(y * (input.width + 1) + x) as usize];

    let mut accessible = 0;
    for y in 0..input.height {
        for x in 0..input.width {
            if datum(y, x) != b'@' {
                continue;
            }

            let count = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .iter()
            .filter(|(dx, dy)| {
                let y = y + dy;
                let x = x + dx;
                (0..input.height).contains(&y)
                    && (0..input.width).contains(&x)
                    && datum(y, x) == b'@'
            })
            .count();

            accessible += (count < 4) as u32;
        }
    }

    accessible
}

pub fn part_2(input: &Input) -> u32 {
    let mut grid = input.grid.to_owned();

    let datum = |grid: &[u8], y, x| grid[(y * (input.width + 1) + x) as usize];
    let extract = |grid: &mut [u8], y, x| {
        let item = &mut grid[(y * (input.width + 1) + x) as usize];
        if *item == b'@' {
            *item = b'.';
        }
    };

    let mut changed = true;
    let mut accessible = 0;
    while changed {
        changed = false;
        for y in 0..input.height {
            for x in 0..input.width {
                if datum(&grid, y, x) != b'@' {
                    continue;
                }

                let count = [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .iter()
                .filter(|(dx, dy)| {
                    let y = y + dy;
                    let x = x + dx;
                    (0..input.height).contains(&y)
                        && (0..input.width).contains(&x)
                        && datum(&grid, y, x) == b'@'
                })
                .count();

                if count < 4 {
                    accessible += 1;
                    changed = true;
                    extract(&mut grid, y, x);
                }
            }
        }
    }

    accessible
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
            "
        });
        assert_eq!(part_1(&input), 13);
        assert_eq!(part_2(&input), 43);
    }

    #[cfg(input_exists)]
    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2025/day4.txt"));
        assert_eq!(part_1(&input), 1372);
        assert_eq!(part_2(&input), 7922);
    }
}
