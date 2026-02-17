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

pub fn decode_notation(s: &String) -> Option<usize> {
    if s.trim().len() == 2 {
        let j: usize = match s.trim()
                                .chars()
                                .next()
                                .unwrap() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
             _  => 8,
        }; 
        let i: usize = match s.trim()
                                .chars()
                                .nth(1)
                                .unwrap()
                                .to_digit(10) {
            Some(num) => num as usize,
            None => 8, 
        };
        if i < 8 && j < 8{
            Some(j + (8 - i) * 8)
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Clone, Copy)]
pub struct Board {
    id: [u8; 64],
    color: [u8; 64],
}

impl Board {
    pub fn new() -> Board {
        Board { 
            id: [
                2, 4, 3, 1, 0, 3, 4, 2,
                5, 5, 5, 5, 5, 5, 5, 5,
                6, 6, 6, 6, 6, 6, 6, 6,
                6, 6, 6, 6, 6, 6, 6, 6,
                6, 6, 6, 6, 6, 6, 6, 6,
                6, 6, 6, 6, 6, 6, 6, 6,
                5, 5, 5, 5, 5, 5, 5, 5,
                2, 4, 3, 1, 0, 3, 4, 2,
            ],
            color: [
                1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1,
                2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ],
        }
    }

    pub fn draw(&self) {
        print!("\n");
        for n in 0..64 {
            if(n % 8) == 0 {
                print!("{:^2}", 8 - n / 8);
            }
            let index:usize = (n + n / 8) % 2;
            print!(
                "{}{}{:^2}{}",
                BACKGROUND_COLOR[index],
                PIECE_COLOR[self.color[n] as usize],
                PIECE_GRAPH[self.id[n] as usize],
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
        print!("\n");
    }

    pub fn move_piece(
            &mut self, start: &usize, end: &usize
        ) {
            if self.id[*start] == 6 {
                println!("the starting tile is empty");
                return;
            }
            if self.id[*end] != 6 {
                println!("the finishing tile is not empty");
                return;
            }
            self.id[*end] = self.id[*start];
            self.color[*end] = self.color[*start];
            self.id[*start] = 6;
            self.color[*start] = 2;
    }
}