#![feature(drain_filter)]

type Board = Vec<Option<u32>>;
const SQUARES: usize = 5;

fn sum(board: &Board) -> u32 {
    board.iter().map(|num| num.unwrap_or(0)).sum()
}

fn horizontal_bingo(board: &Board) -> bool {
    (0..5).any(|r| board[r * 5..][..5].iter().all(Option::is_none))
}

fn vertical_bingo(board: &Board) -> bool {
    (0..5).any(|c| board.iter().skip(c).step_by(5).all(Option::is_none))
}

fn part_1(draws: &Vec<u32>, mut boards: Vec<Board>) -> u32 {
    draws
        .iter()
        .find_map(|&draw| {
            boards.iter_mut().find_map(|board| {
                board.iter_mut().for_each(|cell| *cell = cell.filter(|&num| num != draw));
                (horizontal_bingo(&board) || vertical_bingo(&board))
                .then(|| sum(&board) * draw)
            })
        })
        .unwrap()
}

fn part_2(draws: &Vec<u32>, mut boards: Vec<Board>) -> u32 {
    let (board, draw) = draws
        .iter()
        .filter_map(|&n| {
            boards
                .drain_filter(|board| {
                    board.iter_mut().for_each(|c| *c = c.filter(|v| *v != n));
                    horizontal_bingo(&board) || vertical_bingo(&board)
                })
                .map(|b| (b, n))
                .next()
        })
        .last()
        .unwrap();
    sum(&board) * draw
}

fn main() {
    let (draws, boards) = include_str!("input.txt").split_once("\n\n").unwrap();
    let draws: Vec<u32> = draws.split(",").map(|draw| draw.parse().unwrap()).collect();
    let boards: Vec<Board> = boards
        .split("\n\n")
        .map(|b| b.split_whitespace().map(|n| n.parse().ok()).collect())
        .collect();
    let part_1 = part_1(&draws, boards.clone());
    let part_2 = part_2(&draws, boards);
    println!("part 1 {}, part 2 {}", part_1, part_2);
}

// 31
