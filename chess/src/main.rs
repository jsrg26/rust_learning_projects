use std::io;
use crate::chess::Board;

pub mod chess;

enum State {
    Close,
    Undo,
    Next
}

fn main() {
    let mut board: Board = Board::new();    
    let mut buf: String = String::new();
    let mut start_indx:i8 = 0;
    let mut end_indx:i8 = 0;
    let mut err_msg: &str = "";

    'main: loop {
        loop {   
            print!("\x1bc"); 
            board.draw();
            println!("\x1b[38;2;255;0;0m{}\x1b[0m", err_msg);
            println!("[q] to quit game");
            println!("type the square of the piece you want to move:");   
            match read_index(&mut buf, &mut start_indx, false) {
                Ok(val) => match val {
                    State::Close => {
                        println!("\x1bc");
                        return;
                    },
                    _ => (),
                },
                Err(msg) => {
                    err_msg = msg;
                    continue;
                }
            } 
            err_msg = match board.check_start(&start_indx) {
                Ok(_) => match board.eval_reach() {
                    Ok(_) => break,
                    Err(msg) => msg, 
                },
                Err(msg) => msg,
            }
        }
        err_msg = "";
        loop {
            print!("\x1bc"); 
            board.draw();
            println!("\x1b[38;2;255;0;0m{}\x1b[0m", err_msg);
            println!("[q] to quit game / [u] to undo the move");
            println!("type the square you want to move your pice to:");
            match read_index(&mut buf, &mut end_indx, true) {
                Ok(val) => match val {
                    State::Close => {
                        println!("\x1bc");
                        return;
                    },
                    State::Undo => {
                        board.reset_select();
                        continue 'main;
                    },
                    State::Next => (),
                },
                Err(msg) => {
                    err_msg = msg;
                    continue;
                }
            }
            err_msg = match board.check_end(&end_indx) {
                Ok(_) => {
                    println!("\x1bc");
                    board.move_piece(&end_indx);
                    board.draw();
                    break;
                },
                Err(msg) => msg,
            }
        }
        err_msg = "";
        board.change_turn();
    }
}

fn read_index(
        buf: &mut String, indx: &mut i8, in_turn: bool
    ) -> Result<State, &'static str>{
    *buf = String::new();
    io::stdin()
        .read_line(buf)
        .expect("Failed to read the line");
    if buf.trim() == "q" {
        Ok(State::Close)
    } else if buf.trim() == "u" && in_turn {
        Ok(State::Undo)
    } else {
        *indx = match chess::decode_notation(&buf) {
            Ok(num) => num,
            Err(msg) => {
                return Err(msg);
            },
        };
        Ok(State::Next)
    }
}