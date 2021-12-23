use itertools::Itertools;

fn parse(filename: &str) -> Vec<i32> {
    let line = crate::read_lines(filename)
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    line.split(',').map(|s| s.parse::<i32>().unwrap()).collect()
}

pub(crate) fn step1() -> i32 {
    innerstep("inputs/day07_input.txt", |n| n)
}

pub(crate) fn step2() -> i32 {
    innerstep("inputs/day07_input.txt", sum_arithm)
}

fn innerstep(filename: &str, f: fn(i32) -> i32) -> i32 {
    let crabs = parse(filename);
    let (&min, &max) = crabs.iter().minmax().into_option().unwrap();
    let minmax: Vec<i32> = (min..=max).collect_vec();
    minmax
        .iter()
        .map(|i| crabs.iter().map(|n| f((n - i).abs())).sum())
        .min()
        .unwrap()
}

fn sum_arithm(n: i32) -> i32 {
    n * (n + 1) / 2
}

#[test]
fn test() {
    let fuel = innerstep("inputs/day07_test_input.txt", |n| n);
    assert_eq!(fuel, 37);
    let fuel2 = innerstep("inputs/day07_test_input.txt", sum_arithm);
    assert_eq!(fuel2, 168);
}
