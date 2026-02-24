use std::collections::HashMap;

pub enum State {
    Open,
    Check,
    Stale(bool),
    Mate(bool),
}

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
    [-1, 0], //Vertical Up
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

fn detect_checks(
        indx: &i8,
        current: &HashMap<i8,(&'static Piece, bool)>,
        oposite: &HashMap<i8,(&'static Piece, bool)>,
        white_turn: &bool,
) -> (Vec<i8>, i8) {
    let mut paths = Vec::with_capacity(27);
    let mut count: i8 = 0;
    let val: i8 = if *white_turn {
        -1
    } else {
        1
    };

    let mut dir_0: [i8; 2] = [1, 0];
    let mut dir_1: [i8; 2] = [2, 1];

    let mut new_indx: i8;
    let (mut i, mut j): (i8, i8);

    i = indx / 8 + val;
    j = indx % 8;
    for k in [-1,1i8] {
        new_indx = j + i * 8 + k;
        match oposite.get(&new_indx) {
            Some(p) if p.0.id == 5 => {
                paths.push(new_indx);
                count += 1;
            },
            _ => (),
        }
    }

    for r in 0..8 {
        if r == 4 {
            dir_0 = [
                dir_0[0],
                dir_0[1] + 1
            ];
            dir_1 = [
                dir_1[0] - 1, 
                dir_1[1] + 1
            ]   
        }
        i = indx / 8 + dir_1[0];
        j = indx % 8 + dir_1[1];
        new_indx = j + i * 8;
        match oposite.get(&new_indx) {
            Some(p) if p.0.id == 4 => {
                paths.push(new_indx);
                count += 1;
            },
            _ => (),
        }
        dir_1 = [-dir_1[1], dir_1[0]];

        let mut path: Vec<i8> = Vec::with_capacity(7);
        for l in 1..=7i8 {
            i = indx / 8 + l * dir_0[0];
            j = indx % 8 + l * dir_0[1];
            if (i > -1 && i < 8) && (j > -1 && j < 8) {
                new_indx = j + i * 8;
                if current.contains_key(&new_indx) {
                    break;
                } else if oposite.contains_key(&new_indx) {
                    if oposite[&new_indx].0.id == 1
                        || (oposite[&new_indx].0.id == 2 && r < 4)
                        || (oposite[&new_indx].0.id == 3 && r > 3) {
                        path.push(new_indx);
                        paths.append(&mut path);
                        count += 1;
                    }
                    path.clear();
                    break;
                } else {
                    path.push(new_indx);
                }
            } else {
                path.clear();
                break;
            }
        }
        dir_0 = [-dir_0[1], dir_0[0]];
    }
    (paths, count)
}

fn check_king_moves(
    indx: &mut i8, i: &mut i8, j: &mut i8,
    square: &i8,
    current: &mut HashMap<i8,(&'static Piece, bool)>,
    oposite: &HashMap<i8,(&'static Piece, bool)>,
    piece: &'static Piece,
    dir: &mut [i8; 2],
    white_turn: &bool,
    king_state: &State,
) -> (Vec<i8>, Vec<i8>){
    let mut movement: Vec<i8> = Vec::with_capacity(8);
    let mut capture: Vec<i8> = Vec::with_capacity(8);
    let king:(&'static Piece, bool)
        = current.remove(&square).unwrap();
    for r in 0..piece.n_rot {
        if r == 4 {
            *dir = [dir[0], dir[1] + 1];
        }
        *i = square / 8 + dir[0];
        *j = square % 8 + dir[1];
        if (*i > -1 && *i < 8) && (*j > -1 && *j < 8) {
            *indx = *j + *i * 8;
            if current.contains_key(&indx) {
                *dir = [-dir[1], dir[0]]; 
                continue;
            } else {                            
                let(_, count): (
                    Vec<i8>, i8
                ) = detect_checks(
                    &indx,
                    current,
                    oposite,
                    white_turn,
                );
                if count == 0 {                                
                    if oposite.contains_key(indx) {
                        capture.push(*indx);
                    } else {
                        movement.push(*indx);
                    }
                }                            
            }
        }
        *dir = [-dir[1], dir[0]]; 
    }
    match king_state {
        State::Open if king.1 => for s in 0..=1 {
            let val: i8 = 1 - 2 * s;
            for n in 1..=3 + s {
                *j = *square % 8 + val * n;
                *indx = *square + val * n;
                if n < 3 {
                    let result:
                        (Vec<i8>, i8) = detect_checks(
                        indx, current,
                        oposite, white_turn,
                    );
                    if result.1 > 0 {
                        break;
                    }
                }
                let b_var: bool = ((*j < 7) & (s == 0))
                    | ((*j > 0) & (s == 1));
                if (current.contains_key(indx)
                    || oposite.contains_key(indx))
                    && b_var
                {
                    break;
                } else if current.contains_key(indx)
                    && *j == 7 * (1 - s)
                {
                    if current[indx].0.id == 2
                        && current[indx].1
                    {
                        movement.push(*square + val * 2);
                    }
                }
            } 
        },
        _ => (),
    }
    current.insert(*square, king);
    (movement, capture)
}

fn check_pawn_moves(
    indx: &mut i8, i: &mut i8, j: &mut i8,
    square: &i8,
    current: &mut HashMap<i8,(&'static Piece, bool)>,
    oposite: &HashMap<i8,(&'static Piece, bool)>,
    dir: &mut [i8; 2],
    en_passant: &i8,
) -> (Vec<i8>, Vec<i8>){
    let mut movement: Vec<i8> = Vec::with_capacity(3);
    let mut capture: Vec<i8> = Vec::with_capacity(2);
    *i = *square / 8 + dir[0];
    *j = *square % 8;
    *indx = *j + *i * 8;
    if !current.contains_key(indx)
        && !oposite.contains_key(indx) {
        movement.push(*indx);
        if current[&square].1 
            && !current.contains_key(&(*indx + dir[0] * 8))
            && !oposite.contains_key(&(*indx + dir[0] * 8)) {
            movement.push(*indx + dir[0] * 8);
        }
    }
    for k in [-1, 1i8] {
        if (*j + k) > -1 && (*j + k) < 8 {
            if oposite.contains_key(&(*indx + k)) {
                capture.push(*indx + k);
            }
            if *en_passant == *indx - dir[0] * 8 + k 
                && !current.contains_key(&(*indx + k))
                && !oposite.contains_key(&(*indx + k)) {
                capture.push(*indx + k);
            }
        }
    }
    (movement, capture)
}

fn check_piece_move(
    indx: &mut i8, i: &mut i8, j: &mut i8,
    square: &i8,
    current: &mut HashMap<i8,(&'static Piece, bool)>,
    oposite: &HashMap<i8,(&'static Piece, bool)>,
    piece: &'static Piece,
    dir: &mut [i8; 2],
) -> (Vec<i8>, Vec<i8>){
    let mut movement: Vec<i8> = Vec::with_capacity(27);
    let mut capture: Vec<i8> = Vec::with_capacity(8);
    for r in 0..piece.n_rot {
        if r == 4 {
            *dir = [
                dir[0] - sign(dir[1]),
                dir[1] + sign(dir[0])
            ];
        }
        for l in 1..=piece.range {
            *i = *square / 8 + l * dir[0];
            *j = *square % 8 + l * dir[1];
            if (*i > -1 && *i < 8)
                && (*j > -1 && *j < 8) {
                *indx = *j + *i * 8;
                if current.contains_key(indx) {
                    break;
                } else if oposite.contains_key(indx) {
                    capture.push(*indx);
                    break;
                } else {
                    movement.push(*indx);
                }
            } else {
                break;
            }
        }
        *dir = [-dir[1], dir[0]]; 
    }
    (movement, capture)
}
//------ Piece Data structure ------
struct Piece {
    dir: &'static [i8; 2],
    n_rot: i8,
    range: i8,    
    graph: char,
    id: u8,
}

static PIECES: [Piece; 7] = [
    Piece { //King
        dir: &DIRECTIONS[0], n_rot: 8,
        range: 1, graph: '\u{265A}',
        id:0,
    },
    Piece { //Queen
        dir: &DIRECTIONS[0],n_rot: 8,
        range: 7, graph: '\u{265B}',
        id: 1,
    },
    Piece { //Rook
        dir: &DIRECTIONS[0], n_rot: 4,
        range: 7, graph: '\u{265C}',
        id: 2,
    },
    Piece { //Bishop
        dir: &DIRECTIONS[1], n_rot: 4,
        range: 7, graph: '\u{265D}',
        id: 3,
    },
    Piece { //Knight
        dir: &DIRECTIONS[2], n_rot: 8,
        range: 1, graph: '\u{265E}',
        id: 4,
    },
    Piece { //Pawn White
        dir: &DIRECTIONS[3], n_rot: 1,
        range: 1, graph: '\u{265F}',
        id: 5,
    },
    Piece { //Pawn Black
        dir: &DIRECTIONS[4], n_rot: 1,
        range: 1, graph: '\u{265F}',
        id: 5,
    },
];

//------ Board structue ------
pub struct Board {
    white: HashMap<i8, (&'static Piece, bool)>,
    black: HashMap<i8, (&'static Piece, bool)>,
    valid: HashMap<i8, (Vec<i8>, Vec<i8>)>,
    movement: Vec<i8>,
    capture: Vec<i8>,
    king_indx: [i8; 2],
    en_passant: i8,
    square: i8,
    king_state: State,
    white_turn: bool,
}

impl Board {
    pub fn new() -> Board {
        Board{
            white: HashMap::from([
                (48, (&PIECES[5], true)), (49, (&PIECES[5], true)),
                (50, (&PIECES[5], true)), (51, (&PIECES[5], true)),
                (52, (&PIECES[5], true)), (53, (&PIECES[5], true)),
                (54, (&PIECES[5], true)), (55, (&PIECES[5], true)),
                (56, (&PIECES[2], true)), (63, (&PIECES[2], true)),
                (57, (&PIECES[4], true)), (62, (&PIECES[4], true)),
                (58, (&PIECES[3], true)), (61, (&PIECES[3], true)),
                (59, (&PIECES[1], true)), (60, (&PIECES[0], true)),
            ]),
            black: HashMap::from([
                (15, (&PIECES[6], true)), (14, (&PIECES[6], true)),
                (13, (&PIECES[6], true)), (12, (&PIECES[6], true)),
                (11, (&PIECES[6], true)), (10, (&PIECES[6], true)),
                ( 9, (&PIECES[6], true)), ( 8, (&PIECES[6], true)),
                ( 7, (&PIECES[2], true)), ( 0, (&PIECES[2], true)),
                ( 6, (&PIECES[4], true)), ( 1, (&PIECES[4], true)),
                ( 5, (&PIECES[3], true)), ( 2, (&PIECES[3], true)),
                ( 4, (&PIECES[0], true)), ( 3, (&PIECES[1], true)),
            ]),  
            valid: HashMap::with_capacity(16),       
            movement: Vec::with_capacity(27),
            capture: Vec::with_capacity(8),
            king_indx: [60, 4],
            en_passant: 64,
            square: 64,
            king_state: State::Open,
            white_turn: true,
        }
    }

    pub fn draw(&self) {
        let mut indx: i8;
        let mut piece_color: &'static str;
        let mut piece_graph: char;
        let mut s: &str = "\x1b[38;2;255;0;255mTer\
            \x1b[38;2;255;255;0mmi\
            \x1b[38;2;0;255;255mnal\x1b[0m Chess";
        println!("{:>9}{}\n", ' ', s);
        for n in 0..64i8 {            
            if(n % 8) == 0 {
                print!("{:6}{:^2}", ' ', 8 - n / 8);
            }

            indx = (n + n / 8) % 2 + if self.square == n {
                2
            } else if self.movement.contains(&(n)) {
                4
            } else if self.capture.contains(&(n)) {
                6
            } else {
                0
            };

            (piece_color, piece_graph) = if self.white.contains_key(&n) {
                (PIECE_COLOR[0], self.white[&n].0.graph)
            } else if self.black.contains_key(&n) {
                (PIECE_COLOR[1], self.black[&n].0.graph)
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
        print!("{:^8}", ' ');
        for n in 'a'..='h' {
            print!("{:^2}", n);
        }
        match self.king_state {
            State::Mate(_) | State::Stale(_) => println!("\n"),
            _ => {
                s = if self.white_turn {
                    "\x1b[38;2;153;255;204mWhite\x1b[0m"
                } else {
                    "\x1b[38;2;127;0;255mBlack\x1b[0m"
                };
                println!("\n\n{}'s turn",s);
            },
        }        
    }

    pub fn check_start(
        &mut self, indx: &i8
    ) -> Result<(), &'static str> {        
        let err_msg_0:
            &'static str = "You don't have a piece in that square";
        let err_msg_1:
            &'static str = "Incorrect piece, you are in check";
        let err_msg_2: 
            &'static str = "The piece has no valid moves"; 
        if self.valid.contains_key(indx) {
            self.square = *indx;
            self.movement = self.valid[&self.square].0.clone();
            self.capture = self.valid[&self.square].1.clone();
            Ok(())
        } else {
            match self.king_state {
                State::Check => if self.white_turn {
                    if self.white.contains_key(indx) {
                        Err(err_msg_1)
                    } else {
                        Err(err_msg_0)
                    }            
                } else {
                    if self.black.contains_key(indx) {
                        Err(err_msg_1)
                    } else {
                        Err(err_msg_0)
                    }
                },
                _ => if self.white_turn {
                    if self.white.contains_key(indx) {
                        Err(err_msg_2)
                    } else {
                        Err(err_msg_0)
                    }            
                } else {
                    if self.black.contains_key(indx) {
                        Err(err_msg_2)
                    } else {
                        Err(err_msg_0)
                    }
                },
            }
        }
    }

    pub fn check_end(
        &mut self, indx: &i8
    ) -> Result<(), &'static str> {
        if self.movement.contains(indx)
            || self.capture.contains(indx) {
            self.valid.clear();    
            Ok(())
        } else {
            Err("Invalid movement for your piece")
        }
    }

    pub fn move_piece(&mut self, indx: &i8) {
        let (
            current, 
            oposite,
        ): (
            &mut HashMap<i8,(&Piece, bool)>,
            &mut HashMap<i8,(&Piece, bool)>,
        ) = if self.white_turn {
            (&mut self.white, &mut self.black)
        } else {
            (&mut self.black,&mut self.white)
        };
        let mut piece: (
            &'static Piece,
            bool
        ) = current.remove(&self.square).unwrap();
        if self.capture.contains(indx) {
            if self.en_passant == *indx - piece.0.dir[0] * 8
                && piece.0.id == 5{
                oposite.remove(&(*indx - piece.0.dir[0] * 8));
            } else {
                oposite.remove(indx);
            }
        }
        if piece.0.id == 5 && (self.square - indx).abs() == 16 {
            self.en_passant = *indx;
        }
        else {
            self.en_passant = 64;
        }
        if piece.1 {
            piece.1 = false;
        }
        if piece.0.id == 0 {
            if self.white_turn {
                if (self.king_indx[0] - *indx).abs() == 2 {
                    let val: [i8; 2] = if self.king_indx[0] < *indx {
                        [*indx + 1, *indx - 1]
                    } else {
                        [*indx - 2, *indx + 1]
                    };
                    let rook: (
                        &Piece,
                        bool,
                    ) = current.remove(&val[0]).unwrap();
                    current.insert(val[1], rook);                    
                }
                self.king_indx[0] = *indx;
            } else {
                if (self.king_indx[1] - *indx).abs() == 2 {
                    let val: [i8; 2] = if self.king_indx[1] < *indx {
                        [*indx + 1, *indx - 1]
                    } else {
                        [*indx - 2, *indx + 1]
                    };
                    let rook: (&Piece,bool) =
                        current.remove(&val[0]).unwrap();
                    current.insert(val[1], rook);                    
                }
                self.king_indx[1] = *indx;
            }
        }
        current.insert(*indx, piece);
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

    pub fn calc_valid_moves(&mut self) -> State{
        let (
            current,
            oposite
        ): (
            &mut HashMap<i8, (&'static Piece, bool)>,
            &HashMap<i8, (&'static Piece, bool)>,
        ) = if self.white_turn {
            (&mut self.white, &self.black)
        } else {
            (&mut self.black, &self.white)
        };
        
        let (
            path,
            count,
        ): (
            Vec<i8>,
            i8,
        ) = detect_checks(
            &self.king_indx[!self.white_turn as usize],
            current,
            oposite,
            &self.white_turn,
        );

        let mut indx: i8 = 0;
        let mut i: i8 = 0;
        let mut j: i8 = 0;
        
        let mut king_count: u8 = 0; 
        let mut movement_count: u8 = 0; 
        let mut block_count: u8 = 0;
        let keys: Vec<i8> = current.keys().cloned().collect();
        for k in keys.iter() {
            let piece: &'static Piece = current[k].0;
            let mut dir: [i8; 2] = *piece.dir;
            if piece.id == 0 { 
                    let result:
                    (Vec<i8>, Vec<i8>) = check_king_moves(
                    &mut indx,&mut i, &mut j, k,
                    current, oposite, piece, &mut dir,
                    &self.white_turn, &self.king_state,
                );
                king_count =
                    (result.0.len() + result.1.len()) as u8;
                if king_count > 0 {
                    self.valid.insert(*k, result);
                }

            } else if piece.id == 5 {
                let result:
                    (Vec<i8>, Vec<i8>) = check_pawn_moves(
                    &mut indx, &mut i, &mut j, k,
                    current, oposite, &mut dir,
                    &self.en_passant,
                );
                movement_count += 
                    (result.0.len() + result.1.len()) as u8;
                if count == 1 {
                    let mut movement: Vec<i8> = Vec::with_capacity(2);
                    let mut capture: Vec<i8> = Vec::with_capacity(2);
                    for v in result.0 {
                        if path.contains(&v) {
                            block_count += 1;
                            movement.push(v);
                        }
                    } 
                    for v in result.1 {
                        if path.contains(&v) {
                            block_count += 1;
                            capture.push(v);
                        }
                    }   
                    if movement.len() + capture.len() > 0 {
                        self.valid.insert(*k, (movement, capture));
                    }
                } else if count == 0 
                    && result.0.len() + result.1.len() > 0 {
                    self.valid.insert(*k, result);
                }
            } else {
                let result:
                    (Vec<i8>, Vec<i8>) = check_piece_move(
                    &mut indx, &mut i, &mut j, k,
                    current, oposite, piece, &mut dir
                );
                movement_count +=
                    (result.0.len() + result.1.len()) as u8;
                if count == 1 {
                    let mut movement: Vec<i8> = Vec::with_capacity(27);
                    let mut capture: Vec<i8> = Vec::with_capacity(8);
                    for v in result.0 {
                        if path.contains(&v) {
                            block_count += 1;
                            movement.push(v);
                        }
                    } 
                    for v in result.1 {
                        if path.contains(&v) {
                            block_count += 1;
                            capture.push(v);
                        }
                    }
                    if movement.len() + capture.len() > 0 {
                        self.valid.insert(*k, (movement, capture));
                    }   
                } else if count == 0 
                    && result.0.len() + result.1.len() > 0 {
                    self.valid.insert(*k, result);
                }
            }
        }
        match count {
            0 => if king_count + movement_count == 0 {
                self.king_state = 
                    State::Stale(self.white_turn);
                State::Stale(self.white_turn)
            } else {
                self.king_state = State::Open;
                State::Open
            },
            1 => if king_count + block_count == 0 {
                self.king_state =
                    State::Mate(self.white_turn);
                State::Mate(self.white_turn)
            } else {
                self.king_state = State::Check;
                State::Check
            },
            _ => if king_count == 0 {
                self.king_state =
                    State::Mate(self.white_turn);
                State::Mate(self.white_turn)
            } else {
                self.king_state = State::Check;
                State::Check
            }
        }
    }
}