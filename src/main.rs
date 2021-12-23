#[macro_use]
extern crate lazy_static;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    println!("day1 step1 {:?}", day01::step1()?);
    println!("day1 step2 {:?}", day01::step2()?);

    println!("day2 step1 {:?}", day02::step1()?);
    println!("day2 step2 {:?}", day02::step2()?);

    println!("day3 step1 {:?}", day03::step1()?);
    println!("day3 step2 {:?}", day03::step2()?);

    println!("day4 step1 {:?}", day04::step1());
    println!("day4 step2 {:?}", day04::step2());

    println!("day5 step1 {:?}", day05::aoc(true));
    println!("day5 step2 {:?}", day05::aoc(false));

    println!("day6 step1 {:?}", day06::aoc(80));
    println!("day6 step2 {:?}", day06::aoc(256));

    println!("day7 step1 {:?}", day07::step1());
    println!("day7 step2 {:?}", day07::step2());

    println!("day8 step1 {:?}", day08::step1());
    println!("day8 step1 {:?}", day08::step2());
    Ok(())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
