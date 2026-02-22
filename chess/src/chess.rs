use std::collections::HashMap;

const RESET_COLOR: &str = "\x1b[0m";

static BACKGROUND_COLOR: [&str;8] = [
    "\x1b[48;2;151;147;204m", //light
    "\x1b[48;2;94;101;191m" , //dark
    "\x1b[48;2;182;173;142m", //light_selected
    "\x1b[48;2;143;141;133m", //dark_selected
    "\x1b[48;2;129;175;139m", //light_movement
    "\x1b[48;2;94;147;131m" , //dark_movement
    "\x1b[48;2;181;122;176m", //light_capture
    "\x1b[48;2;147;94;168m" , //dark_capture
];

static PIECE_COLOR: [&str; 2] = [
    "\x1b[38;2;255;255;255m", //white
    "\x1b[38;2;0;0;0m",       //black
];

static DIRECTIONS: [[i8; 2]; 5] = [
    [ 1, 0], //Horizontal
    [ 1, 1], //Diagonal
    [ 2, 1], //L shape    
    [-1,0], //Vertical Up
    [ 1, 0], //Vertical Down
];

//------ General functions ------
fn sign(num: i8) -> i8 {
    match num {
        num if num > 0 => 1,
        num if num < 0 => -1,
        _ => 0,
    }
}

pub fn decode_notation(s: &String) -> Result<i8, &'static str> {
    let err_msg: &'static str = "Wrong notation, try again";
    if s.trim().len() == 2 {
        let j: i8 = match s.trim()
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
             _  => return Err(err_msg),
        }; 
        let i: i8 = match s.trim()
                                .chars()
                                .nth(1)
                                .unwrap()
                                .to_digit(10) {
            Some(num) => num as i8,
            None => return Err(err_msg), 
        };
        if i < 9 && i > 0 {
            Ok(j + (8 - i) * 8)
        } else {
            return Err(err_msg);
        }
    } else {
        return Err(err_msg);
    }
}

//------ Piece Data structure ------
struct Piece {
    dir: &'static [i8; 2],
    n_rot: i8,
    range: i8,    
    graph: char,
    is_pawn: bool,
}

static PIECES: [Piece; 7] = [
    Piece { //King
        dir: &DIRECTIONS[0], n_rot: 8,
        range: 1, graph: '\u{265A}',
        is_pawn: false,
    },
    Piece { //Queen
        dir: &DIRECTIONS[0],n_rot: 8,
        range: 7, graph: '\u{265B}',
        is_pawn: false,
    },
    Piece { //Rook
        dir: &DIRECTIONS[0], n_rot: 4,
        range: 7, graph: '\u{265C}',
        is_pawn: false,
    },
    Piece { //Bishop
        dir: &DIRECTIONS[1], n_rot: 4,
        range: 7, graph: '\u{265D}',
        is_pawn: false,
    },
    Piece { //Knight
        dir: &DIRECTIONS[2], n_rot: 8,
        range: 1, graph: '\u{265E}',
        is_pawn: false,
    },
    Piece { //Pawn White
        dir: &DIRECTIONS[3], n_rot: 1,
        range: 1, graph: '\u{265F}',
        is_pawn: true,
    },
    Piece { //Pawn Black
        dir: &DIRECTIONS[4], n_rot: 1,
        range: 1, graph: '\u{265F}',
        is_pawn: true,
    },
];

//------ Board structue ------
pub struct Board {
    white: HashMap<i8, &'static Piece>,
    black: HashMap<i8, &'static Piece>,
    movement: Vec<i8>,
    capture: Vec<i8>,
    pawn_moved: [u8; 2],
    en_passant: i8,
    square: i8,
    white_turn: bool,
}

impl Board {
    pub fn new() -> Board {
        Board{
            white: HashMap::from([
                (48, &PIECES[5]), (49, &PIECES[5]),
                (50, &PIECES[5]), (51, &PIECES[5]),
                (52, &PIECES[5]), (53, &PIECES[5]),
                (54, &PIECES[5]), (55, &PIECES[5]),
                (56, &PIECES[2]), (63, &PIECES[2]),
                (57, &PIECES[4]), (62, &PIECES[4]),
                (58, &PIECES[3]), (61, &PIECES[3]),
                (59, &PIECES[1]), (60, &PIECES[0]),
            ]),
            black: HashMap::from([
                (15, &PIECES[6]), (14, &PIECES[6]),
                (13, &PIECES[6]), (12, &PIECES[6]),
                (11, &PIECES[6]), (10, &PIECES[6]),
                ( 9, &PIECES[6]), ( 8, &PIECES[6]),
                ( 7, &PIECES[2]), ( 0, &PIECES[2]),
                ( 6, &PIECES[4]), ( 1, &PIECES[4]),
                ( 5, &PIECES[3]), ( 2, &PIECES[3]),
                ( 4, &PIECES[0]), ( 3, &PIECES[1]),
            ]),         
            movement: Vec::with_capacity(27),
            capture: Vec::with_capacity(8),
            pawn_moved: [0; 2],
            en_passant: 64,
            square: 64,
            white_turn: true,
        }
    }

    pub fn draw(&self) {
        let mut indx: i8;
        let mut piece_color: &'static str;
        let mut piece_graph: char;
        println!("\n");
        for n in 0..64i8 {            
            if(n % 8) == 0 {
                print!("{:^2}", 8 - n / 8);
            }

            indx = (n + n / 8) % 2 + 
            if self.square == n {
                2
            } else if self.movement.contains(&(n)) {
                4
            } else if self.capture.contains(&(n)) {
                6
            } else {
                0
            };

            (piece_color, piece_graph) = if self.white.contains_key(&n) {
                (PIECE_COLOR[0], self.white[&n].graph)
            } else if self.black.contains_key(&n) {
                (PIECE_COLOR[1], self.black[&n].graph)
            } else {
                ("", ' ')
            };

            print!(
                "{}{}{:^2}{}",
                BACKGROUND_COLOR[indx as usize],
                piece_color,
                piece_graph,
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

    pub fn check_start(
        &mut self, indx: &i8
    ) -> Option<&'static str> {
        let err_msg: &'static str = "you don't have a piece in that square";
        if self.white_turn {
            if self.white.contains_key(indx) {
                self.square = *indx;
                None
            } else {
                Some(err_msg)
            }            
        } else {
            if self.black.contains_key(indx) {
                self.square = *indx;
                None
            } else {
                Some(err_msg)
            }
        }
    }

    pub fn check_end(
        &mut self, indx: &i8
    ) -> Option<&'static str> {
        if self.movement.contains(indx) ||
           self.capture.contains(indx) {
            None
        } else {
            Some("Invalid movement for your piece")
        }
    }

    pub fn eval_reach(&mut self) -> Option<&'static str> {
        let (mut i, mut j) : (i8, i8); 
        let mut indx: i8;
        let (
            current, 
            oposite,
            pawn_moved,
        ): (
            &HashMap<i8,&Piece>,
            &HashMap<i8,&Piece>,
            &u8,
        ) = if self.white_turn {
            (&self.white, &self.black, &self.pawn_moved[0])
        } else {
            (&self.black, &self.white, &self.pawn_moved[1])
        };
        let piece: &Piece = current[&self.square];
        let mut dir: [i8; 2] = *piece.dir;
        if piece.is_pawn {
            (i, j) = (
                self.square / 8 + dir[0],
                self.square % 8
            );
            indx = j + i * 8;
            if !current.contains_key(&indx) &&
               !oposite.contains_key(&indx) {
                self.movement.push(indx);
                if (*pawn_moved >> j) & 1 == 0 &&
                   !current.contains_key(&(indx + dir[0] * 8)) &&
                   !oposite.contains_key(&(indx + dir[0] * 8)) {
                    self.movement.push(indx + dir[0] * 8);
                }
            }
            for k in [-1, 1i8] {
                if oposite.contains_key(&(indx + k)) {
                    self.capture.push(indx + k);
                }
                if self.en_passant == indx - dir[0] * 8 + k && 
                   !current.contains_key(&(indx + k)) &&
                   !oposite.contains_key(&(indx + k)) {
                    self.capture.push(indx + k);
                }
            }       

        } else {
            for r in 0..piece.n_rot {
                if r == 4 {
                    dir = [
                        dir[0] - sign(dir[1]),
                        dir[1] + sign(dir[0])
                    ];
                }
                for l in 1..(piece.range + 1) {
                    (i, j) = (
                        self.square / 8 + l * dir[0],
                        self.square % 8 + l * dir[1]
                    );
                    if (i > -1 && i < 8) && (j > -1 && j < 8) {
                        indx = j + i * 8;
                        if current.contains_key(&indx) {
                            break;
                        } else if oposite.contains_key(&indx) {
                            self.capture.push(indx);
                            break;
                        } else {
                            self.movement.push(indx);
                        }
                    } else {
                        break;
                    }
                }
                dir = [-dir[1], dir[0]]; 
            }
        }
        if self.movement.len() != 0 || self.capture.len() != 0{
            None
        }else {
            self.square = 64;
            Some("The piece cannot move")
        }
    }

    pub fn move_piece(&mut self, indx: &i8) {
        let (
            current, 
            oposite,
            pawn_moved
        ): (
            &mut HashMap<i8,&Piece>,
            &mut HashMap<i8,&Piece>,
            &mut u8,
        ) = if self.white_turn {
            (
                &mut self.white,
                &mut self.black, 
                &mut self.pawn_moved[0],
            )
        } else {
            (
                &mut self.black,
                &mut self.white,
                &mut self.pawn_moved[1],
            )
        };
        let piece: &Piece = current.remove(&self.square).unwrap();
        current.insert(*indx, piece);
        if self.capture.contains(indx) {
            if piece.is_pawn && 
               (self.en_passant == *indx - piece.dir[0] * 8) {
                oposite.remove(&(*indx - piece.dir[0] * 8));
            } else {
                oposite.remove(indx);
            }
        }
        if piece.is_pawn {
            if (self.square - indx).abs() == 16 {
                self.en_passant = *indx;
            }
            let j = self.square % 8;
            if (*pawn_moved >> j) & 1 == 0 {
                *pawn_moved |= 1 << j;
            } 
        }
        else {
            self.en_passant = 64;
        }
        self.movement.clear();
        self.capture.clear();
    }

    pub fn change_turn(&mut self) {
        self.white_turn = !self.white_turn;
        self.square = 64;
    }

    pub fn reset_select(&mut self) {
        self.square = 64;
        self.movement.clear();
        self.capture.clear();
    }
}