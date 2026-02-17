use std::io;
use crate::chess::Board;

pub mod chess;

fn main() {
    let board: Board = Board::new();
    board.draw();

    loop {
        println!("type the square of the piece you want to move");
        let mut buf: String = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read the line");

        let index:usize = match chess::decode_notation(&buf) {
            Some(num) => num,
            None => {
                println!("Wrong notation, try agian");
                continue;
            },
        };
        println!("{}", index);   
    }
}
