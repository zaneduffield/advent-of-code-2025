fn solve(line: &str, mut digits: u8) -> u64 {
    let line = line.as_bytes();

    let mut sum = 0u64;
    let mut largest_idx = 0;

    while digits > 0 {
        digits -= 1;

        let mut largest = 0;
        for (idx, &val) in line[..line.len() - digits as usize]
            .iter()
            .enumerate()
            .skip(largest_idx)
        {
            if val > largest {
                largest = val;
                largest_idx = idx;
            }
        }

        largest_idx += 1;
        sum = sum * 10 + (largest - b'0') as u64;
    }

    sum
}

pub fn part_1(input: &str) -> u64 {
    input.trim().lines().map(|line| solve(line, 2)).sum()
}

pub fn part_2(input: &str) -> u64 {
    input.trim().lines().map(|line| solve(line, 12)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = indoc! {
            "
            987654321111111
            811111111111119
            234234234234278
            818181911112111
            "
        };
        assert_eq!(part_1(input), 357);
        assert_eq!(part_2(input), 3121910778619);
    }

    #[cfg(input_exists)]
    #[test]
    fn test_my_input() {
        let input = include_str!("../../input/2025/day3.txt");
        assert_eq!(part_1(input), 17445);
        assert_eq!(part_2(input), 173229689350551);
    }
}
