use console::{Style, Term};
use rand::Rng;

fn main() {
    let term = Term::stdout();
    let mut board: Vec<Vec<i32>> = create_board(10, 10);
    let sleep_duration = std::time::Duration::from_millis(100);

    _ = term.clear_screen();
    let mut iterations = 0;
    while iterations < 500 {
        iterations += 1;

        board = mutate_board(&board);

        draw_board(&board, &term);

        if is_dead(&board) || always_alive(&board) {
            break;
        }

        std::thread::sleep(sleep_duration);

        if iterations != 500 {
            _ = term.clear_screen();
        }
    }
}

fn draw_board(board: &Vec<Vec<i32>>, term: &Term) {
    let mut line = 0;
    let blue = Style::new().blue();
    let red = Style::new().red();
    while line < board.len() {
        let mut chr = 0;
        while chr < board[line].len() {
            let val: i32 = board[line][chr];
            let str = val.to_string();
            if str == "0" {
                print!("{}", red.apply_to(str));
            } else {
                print!("{}", blue.apply_to(str));
            }
            chr += 1;
        }
        _ = term.write_line("");
        line += 1;
    }
}

fn mutate_board(board: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut line = 0;
    let mut new_board = board.to_vec();
    let len = board.len();
    while line < len {
        let mut cell = 0;
        while cell < board[line].len() {
            let iline = line as i32;
            let icell = cell as i32;
            let is_live = board[line][cell] == 1;
            let neighbors = [
                safe_access(&board, iline - 1, icell - 1),
                safe_access(&board, iline - 1, icell + 1),
                safe_access(&board, iline - 1, icell),
                safe_access(&board, iline, icell - 1),
                safe_access(&board, iline, icell + 1),
                safe_access(&board, iline + 1, icell),
                safe_access(&board, iline + 1, icell - 1),
                safe_access(&board, iline + 1, icell + 1),
            ];

            let live_count = neighbors.into_iter().filter(|&x| x == 1).count();

            if is_live {
                if live_count != 2 && live_count != 3 {
                    new_board[line][cell] = 0;
                } else {
                    new_board[line][cell] = 1;
                }
            } else {
                if live_count == 3 {
                    new_board[line][cell] = 1;
                }
            }

            cell += 1;
        }
        line += 1;
    }

    return new_board;
}

fn safe_access(board: &Vec<Vec<i32>>, row: i32, col: i32) -> i32 {
    let len = board.len() - 1;
    if row < 0 || row > len as i32 {
        return -1;
    }

    let slice: &Vec<i32> = &board[row as usize];
    let slice_lengh = slice.len() - 1;

    if col < 0 || col > slice_lengh as i32 {
        return -1;
    }

    return slice[col as usize];
}

fn create_board(rows: i32, cols: i32) -> Vec<Vec<i32>> {
    let mut rng = rand::thread_rng();
    let mut vec: Vec<Vec<i32>> = [].to_vec();

    let mut row_it = 0;
    while row_it < rows {
        row_it += 1;

        let mut col_it = 0;
        vec.push([].to_vec());
        while col_it < cols {
            vec[row_it as usize - 1].push(rng.gen_range(0..2));
            col_it += 1;
        }
    }

    return vec;
}

fn is_dead(board: &Vec<Vec<i32>>) -> bool {
    return board.into_iter().all(|f| f.into_iter().all(|&x| x == 0));
}

fn always_alive(board: &Vec<Vec<i32>>) -> bool {
    let mut line = 0;
    let mut new_board = board.to_vec();
    let len = board.len();
    let mut always = false;
    while line < len {
        let mut cell = 0;
        while cell < board[line].len() {
            let iline = line as i32;
            let icell = cell as i32;
            let is_live = board[line][cell] == 1;
            let neighbors = [
                safe_access(&board, iline - 1, icell),
                safe_access(&board, iline + 1, icell),
                safe_access(&board, iline + 1, icell - 1),
                safe_access(&board, iline + 1, icell + 1),
            ];

            let live_count = neighbors.into_iter().filter(|&x| x == 1).count();

            if is_live && live_count == 3 {
                always = true;
            }
            cell += 1;
        }
        line += 1;
    }

    let liveCount: usize = board
        .into_iter()
        .map(|f| f.into_iter().filter(|&x| x == &1).count())
        .sum();
    return always && liveCount == 3;
}
