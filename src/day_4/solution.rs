use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (u64, u64) {
    let game = read_one_per_line::<String>("./src/day_4/input.txt").unwrap();
    let numbers: Vec<u64> = build_numbers(game[0].clone());

    let mut boards = build_boards(&game[2..]);
    let mut completed_boards = vec![false; boards.len()];

    // read numbers until a winner occurs
    let (winner_board, number, winner_board_idx) = play(&mut boards, &numbers, &completed_boards);
    let part1 = Board::score(&winner_board, &number);
    completed_boards[winner_board_idx] = true;

    let mut part2 = 0;
    while completed_boards
        .clone()
        .into_iter()
        .any(|completed| !completed)
    {
        let (winner_board, number, winner_board_idx) =
            play(&mut boards, &numbers, &completed_boards);
        part2 = Board::score(&winner_board, &number);
        completed_boards[winner_board_idx] = true;
    }

    (part1, part2)
}

fn play(
    boards: &mut Vec<Vec<Vec<Cell>>>,
    numbers: &Vec<u64>,
    completed_boards: &Vec<bool>,
) -> (Vec<Vec<Cell>>, u64, usize) {
    for number in numbers {
        for (idx, board) in (&mut *boards).into_iter().enumerate() {
            if completed_boards[idx] {
                continue;
            }

            Board::check_by_num(board, number);
            if Board::is_completed(&board) {
                return (board.clone(), number.clone(), idx);
            }
        }
    }

    // unreachable
    return (vec![], 0, 0);
}

fn build_numbers(row: String) -> Vec<u64> {
    row.split(",")
        .map(|num| num.parse::<u64>().unwrap())
        .collect()
}

fn build_boards(raw_boards: &[String]) -> Vec<Vec<Vec<Cell>>> {
    let mut boards: Vec<Vec<Vec<Cell>>> = vec![];
    let rows = Board::build(raw_boards);

    for board_rows in rows.chunks(5) {
        let mut board: Vec<Vec<Cell>> = vec![];
        for row in board_rows.clone() {
            board.push(row.clone());
        }
        boards.push(board);
    }

    boards
}

#[derive(Clone)]
struct Board {}

impl Board {
    pub fn build(raw_boards: &[String]) -> Vec<Vec<Cell>> {
        let mut rows = vec![];

        for row in raw_boards {
            let parsed_row: Vec<Cell> = row
                .split(" ")
                .filter(|num| num != &"")
                .map(|num| Cell::new(num.parse::<u64>().unwrap()))
                .collect();

            if parsed_row.is_empty() {
                continue;
            }

            rows.push(parsed_row);
        }

        rows
    }

    pub fn check_by_num(board: &mut Vec<Vec<Cell>>, number: &u64) {
        for row in &mut *board {
            for cell in &mut *row {
                if cell.num == *number {
                    cell.is_marked = true;
                    return;
                }
            }
        }
    }

    pub fn is_completed(board: &Vec<Vec<Cell>>) -> bool {
        // read rows
        for row in board {
            if row.into_iter().all(|cell| cell.is_marked) {
                return true;
            }
        }

        // read cols
        for col_idx in 0..board.len() {
            let col = board.into_iter().map(|row| &row[col_idx]);
            if col.into_iter().all(|cell| cell.is_marked) {
                return true;
            }
        }

        false
    }

    pub fn score(board: &Vec<Vec<Cell>>, number: &u64) -> u64 {
        let mut sum = 0;

        for row in board {
            for cell in row {
                if !cell.is_marked {
                    sum += cell.num;
                }
            }
        }

        sum * number
    }
}

#[derive(Clone)]
struct Cell {
    num: u64,
    is_marked: bool,
}

impl Cell {
    pub fn new(num: u64) -> Self {
        Self {
            num,
            is_marked: false,
        }
    }
}
