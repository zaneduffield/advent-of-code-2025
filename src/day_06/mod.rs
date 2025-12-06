#[derive(Copy, Clone)]
enum Op {
    Plus,
    Times,
}

fn parse_ops(last_line: &str) -> impl Iterator<Item = Op> {
    last_line.split_ascii_whitespace().map(|op| match op {
        "*" => Op::Times,
        "+" => Op::Plus,
        _ => panic!("unexpected operator: {op}"),
    })
}

pub fn part_1(input: &str) -> u64 {
    let last_line = input.lines().next_back().unwrap();
    let ops: Vec<Op> = parse_ops(last_line).collect();
    let mut calcs: Vec<u64> = ops
        .iter()
        .map(|op| match op {
            Op::Plus => 0,
            Op::Times => 1,
        })
        .collect();

    for line in input.lines() {
        if line.starts_with(['*', '+']) {
            break;
        }
        for (idx, num) in line.split_ascii_whitespace().enumerate() {
            let num: u64 = num.parse().unwrap();
            calcs[idx] = match ops[idx] {
                Op::Plus => calcs[idx] + num,
                Op::Times => calcs[idx] * num,
            };
        }
    }

    calcs.iter().sum()
}

pub fn part_2(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();

    let last_line = input.lines().next_back().unwrap();
    let ops = parse_ops(last_line);

    let mut total = 0;
    let mut col = 0;
    for op in ops {
        let mut calc = match op {
            Op::Plus => 0,
            Op::Times => 1,
        };

        // find non-empty column
        'outer: loop {
            for line in &lines {
                match line.get(col) {
                    Some(b' ') => continue,
                    _ => break 'outer,
                }
            }
            col += 1;
        }

        loop {
            let mut num = 0;
            for line in &lines {
                if let Some(b @ b'0'..=b'9') = line.get(col) {
                    num = num * 10 + (b - b'0') as u64;
                }
            }

            col += 1;
            if num == 0 {
                break;
            }

            calc = match op {
                Op::Plus => calc + num,
                Op::Times => calc * num,
            };
        }

        total += calc;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = indoc! {
            "
            123 328  51 64
             45 64  387 23
              6 98  215 314
            *   +   *   +
            "
        };
        assert_eq!(part_1(input), 4277556);
        assert_eq!(part_2(input), 3263827);
    }

    #[cfg(input_exists)]
    #[test]
    fn test_my_input() {
        let input = include_str!("../../input/2025/day6.txt");
        assert_eq!(part_1(input), 6209956042374);
        assert_eq!(part_2(input), 12608160008022);
    }
}
