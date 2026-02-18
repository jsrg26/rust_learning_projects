use std::io;
use crate::chess::Board;

pub mod chess;

fn main() {
    let mut board: Board = Board::new();    
    let mut buf: String = String::new();
    let mut start_indx:usize = 0;
    let mut end_indx:usize = 0;
    let mut err_msg: &str = "";

    loop {
        loop {   
            print!("\x1bc"); 
            board.draw();
            println!("\x1b[38;2;255;0;0m{}\x1b[0m", err_msg);
            println!("type the square of the piece you want to move or 'q' to close:");        
            if read_index(&mut buf, &mut start_indx) {
                print!("\x1bc"); 
                return;
            }
            err_msg = match board.check_start(&start_indx) {
                None => match board.eval_reach(&start_indx) {
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
            if read_index(&mut buf, &mut end_indx){
                print!("\x1bc"); 
                return;
            }
            err_msg = match board.check_end(&end_indx) {
                Ok(indx) => {
                    println!("\x1bc");
                    board.move_piece(&start_indx, &end_indx);
                    board.draw();
                    if indx != 6 {
                        println!("you capture a piece!!");
                    }
                    break;
                },
                Err(msg) => msg,
            }
        }
        err_msg = "";
        board.change_turn();
    }
}

fn read_index(buf: &mut String, indx: &mut usize) -> bool{
    loop {
        *buf = String::new();
        io::stdin()
            .read_line(buf)
            .expect("Failed to read the line");
        if buf.trim() == "q" {
            return true;
        }
        *indx = match chess::decode_notation(&buf) {
            Ok(num) => num,
            Err(msg) => {
                println!("{}", msg);
                *buf = String::new();
                continue;
            },
        };
        break;
    }
    false
}