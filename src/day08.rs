use itertools::Itertools;
use std::collections::HashMap;

fn parse(filename: &str) -> Vec<(Vec<String>, Vec<String>)> {
    crate::read_lines(filename)
        .unwrap()
        .map(|l| {
            let line = l.unwrap();
            let (digits, numbers) = line.split('|').collect_tuple().unwrap();
            (
                digits
                    .split_whitespace()
                    .map(|s: &str| s.chars().sorted().collect::<String>())
                    .collect(),
                numbers
                    .split_whitespace()
                    .map(|s: &str| s.chars().sorted().collect::<String>())
                    .collect(),
            )
        })
        .collect()
}

fn is_one(digit: &str) -> bool {
    digit.len() == 2
}

fn is_four(digit: &str) -> bool {
    digit.len() == 4
}

fn is_seven(digit: &str) -> bool {
    digit.len() == 3
}

fn is_eight(digit: &str) -> bool {
    digit.len() == 7
}

pub(crate) fn step1() -> usize {
    let numbers = parse("inputs/day08_input.txt");
    numbers
        .iter()
        .map(|(_d, n)| n)
        .flatten()
        .filter(|n| is_one(n) || is_four(n) || is_seven(n) || is_eight(n))
        .count()
}

pub(crate) fn step2() -> u32 {
    let numbers = parse("inputs/day08_input.txt");
    numbers
        .into_iter()
        .map(|(d, n)| {
            let sorted_digits = identify_digits(d);
            compute_numbers(sorted_digits, n)
        })
        .sum()
}

/*
For step 2, one will use this schema for identifying digits

one     len 2   unique                              first
seven   len 3   unique                              first
four    len 4   unique                              first
two     len 5   remaining one of len 5              last
three   len 5   contains letters from seven         second
five    len 5   contained by six                    third
zero    len 6   remaining one of len 6              third
six     len 6   !contains letters from seven        second
nine    len 6   contains letters from four          second
eight   len 7   unique                              first
 */
fn identify_digits(digits: Vec<String>) -> HashMap<String, u32> {
    // first pass
    let one = digits.iter().find(|s| is_one(s)).unwrap();
    let seven = digits.iter().find(|s| is_seven(s)).unwrap();
    let four = digits.iter().find(|s| is_four(s)).unwrap();
    let eight = digits.iter().find(|s| is_eight(s)).unwrap();
    // second pass
    let len6 = digits
        .to_owned()
        .into_iter()
        .filter(|s| s.len() == 6)
        .collect_vec();
    let nine = len6
        .iter()
        .find(|s| four.chars().all(|c| s.contains(c)))
        .unwrap();
    let six = len6
        .iter()
        .find(|s| !seven.chars().all(|c| s.contains(c)))
        .unwrap();
    let zero = len6.iter().find(|s| s != &six && s != &nine).unwrap();

    // third pass
    let len5 = digits
        .to_owned()
        .into_iter()
        .filter(|s| s.len() == 5)
        .collect_vec();
    let three = len5
        .iter()
        .find(|s| seven.chars().all(|c| s.contains(c)))
        .unwrap();
    let five = len5
        .iter()
        .find(|&s| s.chars().all(|c| six.as_str().contains(c)))
        .unwrap();
    let two = len5.iter().find(|s| s != &three && s != &five).unwrap();

    let mut sorted_numbers = HashMap::new();
    sorted_numbers.insert(zero.to_owned(), 0);
    sorted_numbers.insert(one.to_owned(), 1);
    sorted_numbers.insert(two.to_owned(), 2);
    sorted_numbers.insert(three.to_owned(), 3);
    sorted_numbers.insert(four.to_owned(), 4);
    sorted_numbers.insert(five.to_owned(), 5);
    sorted_numbers.insert(six.to_owned(), 6);
    sorted_numbers.insert(seven.to_owned(), 7);
    sorted_numbers.insert(eight.to_owned(), 8);
    sorted_numbers.insert(nine.to_owned(), 9);
    sorted_numbers
}

fn compute_numbers(sorted_digits: HashMap<String, u32>, number: Vec<String>) -> u32 {
    1000 * sorted_digits[&number[0]]
        + 100 * sorted_digits[&number[1]]
        + 10 * sorted_digits[&number[2]]
        + sorted_digits[&number[3]]
}

#[test]
fn test() {
    let numbers = parse("inputs/day08_test_input.txt");
    println!("{:?}", numbers);
}

#[test]
fn test_step2() {
    let numbers = parse("inputs/day08_test_input.txt");
    let sum: u32 = numbers
        .into_iter()
        .map(|(d, n)| {
            let sorted_digits = identify_digits(d);
            println!("{:?} {:?}", sorted_digits, n);
            compute_numbers(sorted_digits, n)
        })
        .sum();
    println!("sum is: {:?}", sum);
    assert_eq!(sum, 61229);
}
