use std::io;
use crate::chess::Board;

pub mod chess;

enum State {
    Close,
    Undo,
    Next,
}

fn main() {
    let mut board: Board = Board::new();    
    let mut buf: String = String::new();
    let mut start_indx:i8 = 0;
    let mut end_indx:i8 = 0;
    let mut err_msg: &str = "";

    'main: loop {
        match board.calc_valid_moves() {
            chess::State::Mate(val) => {
                print!("\x1bc");
                err_msg = if val {
                    "Checkmate for Black, White loses"
                } else {
                    "Checkmate for White, Black loses"
                };
                board.draw();
                println!("\x1b[38;2;0;128;255m{}\x1b[0m", err_msg);
                println!("\nPress any key to close");
                buf = String::new();
                match io::stdin().read_line(&mut buf) {
                    Ok(_) => {
                        print!("\x1bc");
                        break;
                    },
                    Err(_) => println!("Failed to read the line")
                }
            },
            chess::State::Stale(val) => {
                print!("\x1bc");
                err_msg = if val {
                    "White is in Stalemate, is a Tie"
                } else {
                    "Black is in Stalemate, is a Tie"
                };
                board.draw();
                println!("\x1b[38;2;0;128;255m{}\x1b[0m", err_msg);
                println!("Press any key to close");
                buf = String::new();
                match io::stdin().read_line(&mut buf) {
                    Ok(_) => {
                        print!("\x1bc");
                        break;
                    },
                    Err(_) => println!("Failed to read the line")
                }
            },
            _ => (),
        }
        loop {   
            print!("\x1bc");
            board.draw();
            println!("\x1b[38;2;255;0;0m{}\x1b[0m", err_msg);
            println!("[\x1b[38;2;255;0;0mq\x1b[0m]: Quit game");
            println!("Which piece do you want to move?:");   
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
                Ok(_) => break,
                Err(msg) => msg,
            }
        }
        err_msg = "";
        loop {
            print!("\x1bc"); 
            board.draw();
            println!("\x1b[38;2;255;0;0m{}\x1b[0m", err_msg);
            print!("[\x1b[38;2;255;0;0mq\x1b[0m]: Quit game /");
            println!(" [\x1b[38;2;0;255;255mu\x1b[0m]: Unselect");
            println!("Where do you want to move it?:");
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
                    board.move_piece(&end_indx);
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