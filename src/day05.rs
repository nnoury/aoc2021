use itertools::{zip, Itertools};

fn parse(filename: &str) -> Vec<((i32, i32), (i32, i32))> {
    crate::read_lines(filename)
        .unwrap()
        .map(|l| {
            l.unwrap()
                .split(" -> ")
                .map(|p| {
                    p.split(',')
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

lazy_static! {
    static ref VENTS: Vec<((i32, i32), (i32, i32))> = parse("inputs/day05_input.txt");
}

fn steps(start: i32, end: i32) -> Vec<i32> {
    if start < end {
        (start..=end).collect()
    } else {
        (end..=start).rev().collect()
    }
}

fn lines(line: ((i32, i32), (i32, i32))) -> Vec<(i32, i32)> {
    let ((x1, y1), (x2, y2)) = line;
    if x1 == x2 {
        steps(y1, y2).iter().map(|&i| (x1, i)).collect_vec()
    } else if y1 == y2 {
        steps(x1, x2).iter().map(|&i| (i, y1)).collect_vec()
    } else {
        let x = steps(x1, x2);
        let y = steps(y1, y2);
        zip(x, y).collect_vec()
    }
}

pub(crate) fn aoc(ortho: bool) -> usize {
    let mut vents = VENTS.clone();
    if ortho {
        vents.retain(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2);
    }
    let lines = vents.iter().map(|&l| lines(l)).flatten().collect_vec();
    let mut duplicates = lines.into_iter().sorted().dedup_with_count().collect_vec();
    duplicates.retain(|(i, _)| i >= &2usize);
    duplicates.len()
}

#[test]
fn test() {
    let mut vents = parse("inputs/day05_test_input.txt");
    println!("{:?}", vents);
    assert_eq!(vents.len(), 10);
    vents.retain(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2);
    let lines = vents.iter().map(|&l| lines(l)).flatten().collect_vec();
    println!("{:?}", lines);
}
