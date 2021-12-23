use itertools::Itertools;
use std::io;

#[derive(Debug, Clone, Copy)]
enum SubActions {
    Forward(i64),
    UpDown(i64),
}

fn parse(filename: &str) -> io::Result<Vec<SubActions>> {
    let lines = crate::read_lines(filename)?;
    Ok(lines
        .filter_map(|l| {
            let line = l.unwrap();
            let (sub_action, value) = line.split_whitespace().collect_tuple().unwrap();
            let val = value.parse::<i64>().unwrap();
            match sub_action {
                "forward" => Some(SubActions::Forward(val)),
                "down" => Some(SubActions::UpDown(val)),
                "up" => Some(SubActions::UpDown(-val)),
                _ => None,
            }
        })
        .collect_vec())
}

lazy_static! {
    static ref SUB_PATH: Vec<SubActions> = parse("inputs/input_day2.txt").unwrap();
}

pub fn step1() -> io::Result<i64> {
    let (len, depth) = SUB_PATH.iter().fold((0, 0), |(len, depth), &l| match l {
        SubActions::Forward(l) => (len + l, depth),
        SubActions::UpDown(l) => (len, depth + l),
    });
    Ok(len * depth)
}

pub fn step2() -> io::Result<i64> {
    let (len, _aim, depth) = SUB_PATH
        .iter()
        .fold((0, 0, 0), |(len, aim, depth), &l| match l {
            SubActions::Forward(l) => (len + l, aim, depth + l * aim),
            SubActions::UpDown(l) => (len, aim + l, depth),
        });
    Ok(len * depth)
}
