use std::io;

const PIECE: [char; 6] = [
    '\u{265A}', //king
    '\u{265B}', //queen
    '\u{265C}', //rock
    '\u{265D}', //bishop
    '\u{265E}', //knight
    '\u{265F}', //pawn
];

const PIECE_COLOR: [&str; 2] = [
    "\x1b[38;2;255;255;255m", //white
    "\x1b[38;2;0;0;0m", //black
];

const INITIAL_BOARD: [char; 64] = [
    PIECE[2], PIECE[4], PIECE[3], PIECE[1],
    PIECE[0], PIECE[3], PIECE[4], PIECE[2],
    PIECE[5], PIECE[5], PIECE[5], PIECE[5],
    PIECE[5], PIECE[5], PIECE[5], PIECE[5],
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    PIECE[5], PIECE[5], PIECE[5], PIECE[5],
    PIECE[5], PIECE[5], PIECE[5], PIECE[5], 
    PIECE[2], PIECE[4], PIECE[3], PIECE[1],
    PIECE[0], PIECE[3], PIECE[4], PIECE[2],
];


const INITIAL_COLOR: [&str; 64] = [
    PIECE_COLOR[1], PIECE_COLOR[1], PIECE_COLOR[1], PIECE_COLOR[1],
    PIECE_COLOR[1], PIECE_COLOR[1], PIECE_COLOR[1], PIECE_COLOR[1],
    PIECE_COLOR[1], PIECE_COLOR[1], PIECE_COLOR[1], PIECE_COLOR[1],
    PIECE_COLOR[1], PIECE_COLOR[1], PIECE_COLOR[1], PIECE_COLOR[1],
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    PIECE_COLOR[0], PIECE_COLOR[0], PIECE_COLOR[0], PIECE_COLOR[0],
    PIECE_COLOR[0], PIECE_COLOR[0], PIECE_COLOR[0], PIECE_COLOR[0],
    PIECE_COLOR[0], PIECE_COLOR[0], PIECE_COLOR[0], PIECE_COLOR[0],
    PIECE_COLOR[0], PIECE_COLOR[0], PIECE_COLOR[0], PIECE_COLOR[0],
];

const BACKGROUND_COLOR: [&str;2] = [
    "\x1b[48;2;151;147;204m", //light
    "\x1b[48;2;75;81;152m", //dark
];

const RESET_COLOR: &str = "\x1b[0m";

// fn start_board(board: &mut [char; 64], color: &mut [&str; 64]) {
//     *board = initial_board;
//     *color = initial_color;
// }

fn main() {
    let board: [char; 64] = INITIAL_BOARD;
    let color: [&str; 64] = INITIAL_COLOR;
    
    for n in 0..64 {
        let index:usize = (n + n / 8) % 2;
        print!(
            "{}{}{:^2}{}",
            BACKGROUND_COLOR[index],
            color[n],
            board[n],
            RESET_COLOR,
        );
        if (n + 1) % 8 == 0 {
            print!("\n");
        }        
    }
    println!("\nType anything to close: ");
    let mut buffer:String = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("cannot read the line");
}
