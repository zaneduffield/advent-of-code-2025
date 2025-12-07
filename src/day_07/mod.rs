pub struct Input {
    start: usize,
    splitters: Vec<Vec<u8>>,
}

pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let start = first.find('S').unwrap();

    let splitters: Vec<Vec<u8>> = lines.map(|line| line.bytes().collect()).collect();

    Input { start, splitters }
}

fn explore(splitters: &mut [Vec<u8>], col: usize, mut row: usize) -> u32 {
    let mut count = 0;
    while let Some(s) = splitters.get_mut(row).and_then(|r| r.get_mut(col)) {
        match *s {
            // already split and counted
            b'v' => break,
            // splitter, uncounted
            b'^' => {
                *s = b'v';
                count += 1;
                if col > 0 {
                    count += explore(splitters, col - 1, row + 1);
                }
                if col < splitters[0].len() - 1 {
                    count += explore(splitters, col + 1, row + 1);
                }
                break;
            }
            // empty, continue
            _ => {
                row += 1;
            }
        }
    }

    count
}

pub fn part_1(input: &Input) -> u32 {
    // we will switch each splitter from true to false when we explore it
    let mut splitters = input.splitters.clone();
    explore(&mut splitters, input.start, 0)
}

// returns num unique paths explored starting from col/row
fn explore2(splitters: &mut [Vec<u64>], col: usize, mut row: usize) -> u64 {
    while let Some(s) = splitters.get_mut(row).and_then(|r| r.get_mut(col)) {
        if *s == b'^' as u64 {
            // unexplored
            let mut count = 0;
            if col > 0 {
                count += explore2(splitters, col - 1, row + 1);
            }
            if col < splitters[0].len() - 1 {
                count += explore2(splitters, col + 1, row + 1);
            }

            // cache the path count
            splitters[row][col] = count + b'^' as u64;
            return count;
        } else if *s > b'^' as u64 {
            // cached path count
            return *s - b'^' as u64;
        } else {
            row += 1;
        }
    }

    1
}

pub fn part_2(input: &Input) -> u64 {
    let mut splitters: Vec<Vec<u64>> = input
        .splitters
        .iter()
        .map(|line| line.iter().map(|b| *b as u64).collect())
        .collect();
    explore2(&mut splitters, input.start, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            .......S.......
            ...............
            .......^.......
            ...............
            ......^.^......
            ...............
            .....^.^.^.....
            ...............
            ....^.^...^....
            ...............
            ...^.^...^.^...
            ...............
            ..^...^.....^..
            ...............
            .^.^.^.^.^...^.
            ...............
            "
        });
        assert_eq!(part_1(&input), 21);
        assert_eq!(part_2(&input), 40);
    }

    #[cfg(input_exists)]
    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2025/day7.txt"));
        assert_eq!(part_1(&input), 1581);
        assert_eq!(part_2(&input), 73007003089792);
    }
}
