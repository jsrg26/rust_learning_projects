use std::io;

const RESET_COLOR: &str = "\x1b[0m";

const BACKGROUND_COLOR: [&str;2] = [
    "\x1b[48;2;151;147;204m", //light
    "\x1b[48;2;94;101;191m", //dark
];

const PIECE_COLOR: [&str; 3] = [
    "\x1b[38;2;255;255;255m", //white
    "\x1b[38;2;0;0;0m", //black
    "", //no color
];


const PIECE_GRAPH: [char; 7] = [
    '\u{265A}', //king
    '\u{265B}', //queen
    '\u{265C}', //rook
    '\u{265D}', //bishop
    '\u{265E}', //knight
    '\u{265F}', //pawn
    ' ', //blank
];

#[derive(Clone, Copy)]
struct Tile {
    id: u8,
    color: u8,
}

impl Tile {
    pub const fn new(
            id: u8, color: u8,
        ) -> Tile {
        Tile { 
            id: id,
            color: color,
        }
    }
} 

const INITIAL_CONFIG: [Tile; 64] = [
    Tile::new(2, 1), Tile::new(4, 1),
    Tile::new(3, 1), Tile::new(1, 1),
    Tile::new(0, 1), Tile::new(3, 1),
    Tile::new(4, 1), Tile::new(2, 1),

    Tile::new(5, 1), Tile::new(5, 1),
    Tile::new(5, 1), Tile::new(5, 1),
    Tile::new(5, 1), Tile::new(5, 1),
    Tile::new(5, 1), Tile::new(5, 1),

    Tile::new(6, 2), Tile::new(6, 2),
    Tile::new(6, 2), Tile::new(6, 2),
    Tile::new(6, 2), Tile::new(6, 2),
    Tile::new(6, 2), Tile::new(6, 2),
    Tile::new(6, 2), Tile::new(6, 2),
    Tile::new(6, 2), Tile::new(6, 2),
    Tile::new(6, 2), Tile::new(6, 2),
    Tile::new(6, 2), Tile::new(6, 2),
    Tile::new(6, 2), Tile::new(6, 2),
    Tile::new(6, 2), Tile::new(6, 2),
    Tile::new(6, 2), Tile::new(6, 2),
    Tile::new(6, 2), Tile::new(6, 2),
    Tile::new(6, 2), Tile::new(6, 2),
    Tile::new(6, 2), Tile::new(6, 2),
    Tile::new(6, 2), Tile::new(6, 2),
    Tile::new(6, 2), Tile::new(6, 2),

    Tile::new(5, 0), Tile::new(5, 0),
    Tile::new(5, 0), Tile::new(5, 0),
    Tile::new(5, 0), Tile::new(5, 0),
    Tile::new(5, 0), Tile::new(5, 0),

    Tile::new(2, 0), Tile::new(4, 0),
    Tile::new(3, 0), Tile::new(0, 0),
    Tile::new(0, 0), Tile::new(3, 0),
    Tile::new(4, 0), Tile::new(2, 0),
];

fn move_piece(
        board: &mut [Tile; 64],
        indx_start: u8,
        indx_finish: u8
    ) {
    let start: usize = indx_start as usize;
    let finish: usize = indx_finish as usize;
    if board[start].id == 6 {
        println!("the starting tile is empty");
        return;
    }
    if board[finish].id != 6 {
        println!("the finishing tile is not empty");
        return;
    }
    board[finish] = board[start];
    board[start].id = 6;
    board[start].color = 2;
}

fn main() {
    let mut board: [Tile; 64] = INITIAL_CONFIG;
    
    for n in 0..64 {
        if(n % 8) == 0 {
            print!("{:^2}", 8 - n / 8);
        }
        let index:usize = (n + n / 8) % 2;
        print!(
            "{}{}{:^2}{}",
            BACKGROUND_COLOR[index],
            PIECE_COLOR[board[n].color as usize],
            PIECE_GRAPH[board[n].id as usize],
            RESET_COLOR,
        );
        if (n + 1) % 8 == 0 {
            print!("\n");
        }        
    }

    print!("  ");
    for n in 'a'..'i' {
        print!("{:^2}", n);
    }

    println!("\nType the piece to move: ");
    let mut buffer:String = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("cannot read the line");
    let start_poss: u8 = buffer.trim().parse()
                                .expect("type a number!");

    println!("\nType the destination tile: ");
    buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("cannot read the line");
    let end_poss: u8 = buffer.trim().parse()
                                .expect("type a number!");

    move_piece(&mut board,start_poss, end_poss);

    for n in 0..64 {
        if(n % 8) == 0 {
            print!("{:^2}", 8 - n / 8);
        }
        let index:usize = (n + n / 8) % 2;
        print!(
            "{}{}{:^2}{}",
            BACKGROUND_COLOR[index],
            PIECE_COLOR[board[n].color as usize],
            PIECE_GRAPH[board[n].id as usize],
            RESET_COLOR,
        );
        if (n + 1) % 8 == 0 {
            print!("\n");
        }        
    }

    print!("  ");
    for n in 'a'..'i' {
        print!("{:^2}", n);
    }
}
