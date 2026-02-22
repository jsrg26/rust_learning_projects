use std::io;
use crate::chess::Board;

pub mod chess;

fn main() {
    let mut board: Board = Board::new();    
    let mut buf: String = String::new();
    let mut start_indx:i8 = 0;
    let mut end_indx:i8 = 0;
    let mut err_msg: &str = "";

    loop {
        loop {   
            print!("\x1bc"); 
            board.draw();
            println!("\x1b[38;2;255;0;0m{}\x1b[0m", err_msg);
            println!("type the square of the piece you want to move or 'q' to close:");   
            match read_index(&mut buf, &mut start_indx) {
                Ok(val) => if val {
                    print!("\x1bc"); 
                    return;
                },
                Err(msg) => {
                    err_msg = msg;
                    continue;
                }
            } 
            err_msg = match board.check_start(&start_indx) {
                None => match board.eval_reach() {
                    None  => break,
                    Some(msg) => msg, 
                },
                Some(msg) => msg,
            }
        }
        err_msg = "";
        loop {
            print!("\x1bc"); 
            board.draw();
            println!("\x1b[38;2;255;0;0m{}\x1b[0m", err_msg);
            println!("type the square you want to go to or 'q' to close:");
            match read_index(&mut buf, &mut end_indx) {
                Ok(val) => if val {
                    print!("\x1bc"); 
                    return;
                },
                Err(msg) => {
                    err_msg = msg;
                    continue;
                }
            }
            err_msg = match board.check_end(&end_indx) {
                None => {
                    println!("\x1bc");
                    board.move_piece(&end_indx);
                    board.draw();
                    break;
                },
                Some(msg) => msg,
            }
        }
        err_msg = "";
        board.change_turn();
    }
}

fn read_index(
        buf: &mut String, indx: &mut i8
    ) -> Result<bool, &'static str>{
    // loop {
        *buf = String::new();
        io::stdin()
            .read_line(buf)
            .expect("Failed to read the line");
        if buf.trim() == "q" {
            return Ok(true);
        }
        *indx = match chess::decode_notation(&buf) {
            Ok(num) => num,
            Err(msg) => {
                return Err(msg);
                // *buf = String::new();
                // continue;
            },
        };
        // break;
    // }
    return Ok(false);
}