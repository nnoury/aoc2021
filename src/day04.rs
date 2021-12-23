use itertools::Itertools;

fn parse(filename: &str) -> (Vec<u32>, Vec<Vec<(u32, bool)>>) {
    let mut lines = crate::read_lines(filename).unwrap();
    let numbers = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect_vec();
    lines.next();

    let boards = lines
        .map(|l| l.unwrap())
        .join("  ")
        .split("    ")
        .map(|l| {
            l.split_whitespace()
                .map(|n| (n.parse::<u32>().unwrap(), false))
                .collect_vec()
        })
        .collect_vec();
    (numbers, boards)
}

lazy_static! {
    static ref BINGO: (Vec<u32>, Vec<Vec<(u32, bool)>>) = {
        let (mut numbers, boards) = parse("inputs/day04_input.txt");
        numbers.reverse();
        (numbers, boards)
    };
}
pub(crate) fn step1() -> u32 {
    let (numbers, boards) = BINGO.clone();
    play1(numbers, boards)
}

pub(crate) fn step2() -> u32 {
    let (numbers, boards) = BINGO.clone();
    play2(numbers, boards, 0)
}

#[test]
fn test() {
    let (mut numbers, boards) = parse("inputs/day04_test_input.txt");
    boards.iter().for_each(|b| assert_eq!(b.len(), 25));
    println!("{:?}", (&numbers, &boards));
    numbers.reverse();
    println!("{}", play1(numbers, boards));
}

fn victory(board: &[(u32, bool)]) -> Option<u32> {
    let mut columns = vec![(0u32, false); 25];
    transpose::transpose(board, &mut *columns, 5, 5);
    if board.chunks_exact(5).any(|l| l.iter().all(|(_, b)| *b))
        || columns.chunks_exact(5).any(|l| l.iter().all(|(_, b)| *b))
    {
        return Some(board.iter().filter(|(_, b)| !(*b)).map(|(a, _)| a).sum());
    }
    None
}

fn play1(mut numbers: Vec<u32>, mut boards: Vec<Vec<(u32, bool)>>) -> u32 {
    let number = numbers.pop().unwrap();
    update_boards(&mut boards, number);
    for board in &boards {
        if let Some(sum) = victory(board) {
            return sum * number;
        }
    }
    play1(numbers, boards)
}

fn update_boards(boards: &mut Vec<Vec<(u32, bool)>>, number: u32) {
    boards.iter_mut().for_each(|b| {
        b.iter_mut().for_each(|(a, b)| {
            if *a == number {
                *b = true
            }
        })
    });
}

fn play2(mut numbers: Vec<u32>, mut boards: Vec<Vec<(u32, bool)>>, mut score: u32) -> u32 {
    if numbers.is_empty() {
        return score;
    }
    let number = numbers.pop().unwrap();
    update_boards(&mut boards, number);
    boards.retain(|board| {
        if let Some(sum) = victory(board) {
            score = sum * number;
            false
        } else {
            true
        }
    });
    play2(numbers, boards, score)
}
