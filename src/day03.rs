use itertools::Itertools;
use std::io;

lazy_static! {
    static ref REPORT: Vec<u32> = crate::read_lines("inputs/input_day3.txt")
        .unwrap()
        .map(|l| { u32::from_str_radix(&l.unwrap(), 2).unwrap() })
        .collect_vec();
}
pub fn step1() -> io::Result<usize> {
    let length = REPORT.len();
    let val = (0..12)
        .map(|i| {
            (
                i,
                REPORT
                    .iter()
                    .filter(|&&j| (j & (1u32 << i)) >> i & 1 == 1)
                    .count()
                    > length / 2,
            )
        })
        .fold(0, |acc, (i, b)| acc + if b { 1 << i } else { 0 });
    Ok((val * (!val & ((1u32 << 12) - 1))) as usize)
}

pub fn step2() -> io::Result<usize> {
    Ok((innerstep2(REPORT.clone(), 11, true) * innerstep2(REPORT.clone(), 11, false)) as usize)
}

fn innerstep2(mut report: Vec<u32>, i: i32, is_oxygen: bool) -> u32 {
    //println!("[{} ]", report.iter().fold(String::new(), |acc, i| acc + &*format!(" {:#b}", i)));
    let length = report.len();
    if length == 1 {
        return report[0];
    }
    let count = report
        .iter()
        .filter(|&j| (j & (1u32 << i)) >> i & 1 == is_oxygen.into())
        .count();
    let keep = count > length - count || (is_oxygen && count == length - count);
    //println!("count: {}, keep: {}, {:#b}", count, keep, 1u32 << i);
    report.retain(|j| ((j & (1u32 << i)) >> i & 1 == keep.into()));
    innerstep2(report, i - 1, is_oxygen)
}

#[test]
fn test() {
    let test_report: Vec<u32> = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
        .lines()
        .map(|l| u32::from_str_radix(&l, 2).unwrap())
        .collect_vec();

    let oxygen = innerstep2(test_report.clone(), 4, true);
    let co2 = innerstep2(test_report.clone(), 4, false);

    dbg!(oxygen, co2, oxygen * co2);
}
