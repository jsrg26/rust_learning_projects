use std::io;
use crate::chess::Board;

pub mod chess;

fn main() {
    let mut board: Board = Board::new();
    board.draw();

    
    let mut buf: String = String::new();
    let mut start_indx:usize = 0;
    let mut end_indx:usize = 0;

    read_index(&mut buf, &mut start_indx);
    read_index(&mut buf, &mut end_indx);

    board.move_piece(&start_indx, &end_indx);
    board.draw();
}

fn read_index(buf: &mut String, indx: &mut usize) {
    loop {
        *buf = String::new();
        io::stdin()
            .read_line(buf)
            .expect("Failed to read the line");

        *indx = match chess::decode_notation(&buf) {
            Some(num) => num,
            None => {
                println!("Wrong notation, try again");
                *buf = String::new();
                continue;
            },
        };
        break;
    }
}