// chess_rs.rs — Шахматы (2D) на Rust

use std::io::{self, Write, BufRead};

const SIZE: usize = 8;

struct Board {
    cells: [[char; SIZE]; SIZE],
    white_turn: bool,
}

impl Board {
    fn new() -> Self {
        let setup = [
            "RNBQKBNR",
            "PPPPPPPP",
            "........",
            "........",
            "........",
            "........",
            "pppppppp",
            "rnbqkbnr",
        ];
        let mut cells = [['.'; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in 0..SIZE {
                cells[i][j] = setup[i].chars().nth(j).unwrap();
            }
        }
        Board { cells, white_turn: true }
    }

    fn print(&self) {
        println!("  a b c d e f g h");
        for i in 0..SIZE {
            print!("{} ", 8 - i);
            for j in 0..SIZE {
                print!("{} ", self.cells[i][j]);
            }
            println!();
        }
        println!("  a b c d e f g h");
    }

    fn move_piece(&mut self, from: &str, to: &str) -> bool {
        let fx = (from.chars().nth(0).unwrap() as u8 - b'a') as usize;
        let fy = (b'8' - from.chars().nth(1).unwrap() as u8) as usize;
        let tx = (to.chars().nth(0).unwrap() as u8 - b'a') as usize;
        let ty = (b'8' - to.chars().nth(1).unwrap() as u8) as usize;
        if fx >= SIZE || fy >= SIZE || tx >= SIZE || ty >= SIZE {
            return false;
        }
        let piece = self.cells[fy][fx];
        if piece == '.' {
            return false;
        }
        let target = self.cells[ty][tx];
        if target != '.' && (target.is_uppercase() == piece.is_uppercase()) {
            return false;
        }
        self.cells[ty][tx] = piece;
        self.cells[fy][fx] = '.';
        true
    }
}

fn main() {
    let mut board = Board::new();
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        board.print();
        let turn = if board.white_turn { "Белые" } else { "Чёрные" };
        print!("{} ходят. Введите ход (e2 e4): ", turn);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" { break; }
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Неверный формат");
            continue;
        }
        if board.move_piece(parts[0], parts[1]) {
            board.white_turn = !board.white_turn;
        } else {
            println!("Неверный ход.");
        }
    }
}
