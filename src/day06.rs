use itertools::Itertools;
use std::collections::VecDeque;

fn parse(filename: &str) -> VecDeque<(usize, i32)> {
    let line = crate::read_lines(filename)
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    line.split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .sorted()
        .dedup_with_count()
        .collect_vec()
        .into()
}

pub(crate) fn aoc(day: i32) -> usize {
    let mut lanternfishs = parse("inputs/day06_input.txt");
    for _ in 0..day {
        iter(&mut lanternfishs);
    }
    lanternfishs.iter().map(|(c, _)| c).sum::<usize>()
}

fn iter(lanternfishs: &mut VecDeque<(usize, i32)>) {
    let mut count = 0;
    let mut seven_found = false;
    let mut eight_found = false;
    lanternfishs.iter_mut().for_each(|(c, i)| {
        if *i == 0 {
            count += *c;
        } else if *i == 7 {
            *c += count;
            seven_found = true;
        } else if *i == 8 {
            eight_found = true;
        }
        *i -= 1;
    });
    if count > 0 {
        lanternfishs.pop_front();
        if !seven_found {
            lanternfishs.push_back((count, 6));
            if eight_found {
                lanternfishs.swap(lanternfishs.len() - 1, lanternfishs.len() - 2);
            }
        }
        lanternfishs.push_back((count, 8));
    }
}

#[test]
fn test() {
    let mut lanternfishs = parse("inputs/day06_test_input.txt");
    println!("{:?}", lanternfishs);
    for i in 0..80 {
        iter(&mut lanternfishs);
        println!("after {} days, {:?}", i, lanternfishs);
    }
    let len = lanternfishs.iter().map(|(c, _)| c).sum::<usize>();
    println!("{:?}", len);
    assert_eq!(len, 5934);
}
