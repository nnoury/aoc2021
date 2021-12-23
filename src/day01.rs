use itertools::Itertools;
use std::io;

lazy_static! {
    static ref DEPTHS: Vec<i32> = crate::read_lines("inputs/input_day1.txt")
        .unwrap()
        .map(|l| { l.unwrap().parse::<i32>().unwrap() })
        .collect_vec();
}
pub fn step1() -> io::Result<usize> {
    Ok(DEPTHS
        .iter()
        .tuple_windows()
        .filter(|(&a, &b)| b - a > 0)
        .count())
}

pub fn step2() -> io::Result<usize> {
    Ok(DEPTHS
        .iter()
        .tuple_windows()
        .tuple_windows()
        .map(|((a, b), (_, c))| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b - a > 0)
        .count())
}
