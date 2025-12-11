use std::ops::RangeInclusive;

pub struct Input {
    tiles: Vec<(u32, u32)>,
}

pub fn input_generator(input: &str) -> Input {
    let tiles = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();
    Input { tiles }
}

fn rects(input: &Input) -> impl Iterator<Item = (&(u32, u32), &(u32, u32))> {
    input
        .tiles
        .iter()
        .enumerate()
        .flat_map(|(idx, t1)| input.tiles.iter().skip(idx + 1).map(move |t2| (t1, t2)))
}

fn rect_area((t1, t2): (&(u32, u32), &(u32, u32))) -> u64 {
    (1 + t1.0.abs_diff(t2.0) as u64) * (1 + t1.1.abs_diff(t2.1) as u64)
}

fn rect_sizes(input: &Input) -> impl Iterator<Item = u64> {
    rects(input).map(rect_area)
}

pub fn part_1(input: &Input) -> u64 {
    rect_sizes(input).max().unwrap_or(0)
}

fn minmax_range(a: u32, b: u32) -> RangeInclusive<u32> {
    a.min(b)..=a.max(b)
}

// idea: cast rays to the edge and count intersections to determine being inside the shape
pub fn part_2(input: &Input) -> u64 {
    let mut vert_edges = vec![];
    let mut hori_edges = vec![];
    for (p1, p2) in input
        .tiles
        .iter()
        .zip(input.tiles.iter().skip(1).chain(input.tiles.first()))
    {
        if p1.0 == p2.0 {
            vert_edges.push((p1.0, minmax_range(p1.1, p2.1)))
        } else {
            hori_edges.push((p1.1, minmax_range(p1.0, p2.0)))
        }
    }

    vert_edges.sort_by_key(|e| e.0);
    hori_edges.sort_by_key(|e| e.0);

    let vert_max = vert_edges
        .iter()
        .max_by_key(|e| e.1.clone().count())
        .unwrap();
    let hori_max = hori_edges
        .iter()
        .max_by_key(|e| e.1.clone().count())
        .unwrap();

    let rects: Vec<_> = rects(input)
        .map(|(p1, p2)| (rect_area((p1, p2)), p1, p2))
        .collect();

    let mut max_area = 0;
    'outer: for (area, p1, p2) in rects.iter().rev() {
        if *area <= max_area {
            continue;
        }
        let xrange = p1.0.min(p2.0)..=p1.0.max(p2.0);
        let yrange = p1.1.min(p2.1)..=p1.1.max(p2.1);

        // optimise by checking for intersection with the longest edges first
        if (vert_max.1.contains(yrange.start()) || vert_max.1.contains(yrange.end()))
            && *xrange.start() < vert_max.0
            && vert_max.0 < *xrange.end()
        {
            continue;
        }
        if (hori_max.1.contains(xrange.start()) || hori_max.1.contains(xrange.end()))
            && *yrange.start() < hori_max.0
            && hori_max.0 < *yrange.end()
        {
            continue;
        }

        let edge_pos = vert_edges.partition_point(|edge| edge.0 <= *xrange.start());
        let end_edge_pos = vert_edges[edge_pos..].partition_point(|edge| edge.0 < *xrange.end());

        for v_edge in &vert_edges[edge_pos..edge_pos + end_edge_pos] {
            if (v_edge.1.start() < yrange.end() && v_edge.1.end() > yrange.start())
                || (v_edge.1.end() > yrange.start() && v_edge.1.start() < yrange.end())
            {
                continue 'outer;
            }
        }

        let edge_pos = hori_edges.partition_point(|edge| edge.0 <= *yrange.start());
        let end_edge_pos = hori_edges[edge_pos..].partition_point(|edge| edge.0 < *yrange.end());

        for h_edge in &hori_edges[edge_pos..edge_pos + end_edge_pos] {
            if (h_edge.1.start() < xrange.end() && h_edge.1.end() > xrange.start())
                || (h_edge.1.end() > xrange.start() && h_edge.1.start() < xrange.end())
            {
                continue 'outer;
            }
        }

        max_area = *area;
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3
            "
        });
        assert_eq!(part_1(&input), 50);
        assert_eq!(part_2(&input), 24);
    }

    #[cfg(input_exists)]
    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2025/day9.txt"));
        assert_eq!(part_1(&input), 4759420470);
        assert_eq!(part_2(&input), 1603439684);
    }
}
