type Movement = ([i8;2], u8, i8);

const RESET_COLOR: &str = "\x1b[0m";

const BACKGROUND_COLOR: [&str;8] = [
    "\x1b[48;2;151;147;204m", //light
    "\x1b[48;2;94;101;191m" , //dark
    "\x1b[48;2;182;173;142m", //light_selected
    "\x1b[48;2;143;141;133m", //dark_selected
    "\x1b[48;2;129;175;139m", //light_movement
    "\x1b[48;2;94;147;131m" , //dark_movement
    "\x1b[48;2;181;122;176m", //light_capture
    "\x1b[48;2;147;94;168m" , //dark_capture
];

const PIECE_COLOR: [&str; 3] = [
    "\x1b[38;2;255;255;255m", //white
    "\x1b[38;2;0;0;0m",       //black
    "",                       //no color
];


const PIECE_GRAPH: [char; 7] = [
    '\u{265A}', //king
    '\u{265B}', //queen
    '\u{265C}', //rook
    '\u{265D}', //bishop
    '\u{265E}', //knight
    '\u{265F}', //pawn
    ' ',        //blank
];

const  PIECE_MOVEMENT: [Movement; 5] = [
    ([1, 0], 8, 1),
    ([1, 0], 8, 7),
    ([1, 0], 4, 7),
    ([1, 1], 4, 7),
    ([2, 1], 8, 1),
];

fn sign(num: i8) -> i8 {
    match num {
        num if num > 0 => 1,
        num if num < 0 => -1,
        _ => 0,
    }
}

pub fn decode_notation(s: &String) -> Result<usize, &'static str> {
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
             _  => return Err("Wrong letter, try again"),
        }; 
        let i: usize = match s.trim()
                                .chars()
                                .nth(1)
                                .unwrap()
                                .to_digit(10) {
            Some(num) => num as usize,
            None => return Err("Wrong number, try again"), 
        };
        if i < 9 && i > 0 {
            Ok(j + (8 - i) * 8)
        } else {
            return Err("Wrong number, try again");
        }
    } else {
        return Err("Wrong notation, try again");
    }
}
pub struct Board {
    id: [u8; 64],
    color: [u8; 64],
    movement: Vec<u8>,
    capture: Vec<u8>,
    pawn_moved: [u8;2],
    turn: u8,
    selected: u8,
}

impl Board {
    pub fn change_turn(&mut self) {
        self.turn = (self.turn + 1) % 2;
    }

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
            movement: Vec::with_capacity(27),
            capture: Vec::with_capacity(8),
            pawn_moved: [0; 2],
            turn: 0,
            selected: 64,
        }
    }

    pub fn draw(&self) {
        print!("\n");
        for n in 0..64 {
            if(n % 8) == 0 {
                print!("{:^2}", 8 - n / 8);
            }
            let mut index:usize = (n + n / 8) % 2;
            if self.selected == n as u8{
                index += 2;
            }
            if self.movement.contains(&(n as u8)) {
                index += 4;
            }
            if self.capture.contains(&(n as u8)) {
                index += 6;
            }
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

    pub fn check_start(
            &self, indx: &usize
        ) -> Option<&'static str> {
        if self.id[*indx] == 6 {
            return Some("the square is empty");
        } else if self.color[*indx] != self.turn {
            return Some("the square contains an opponent's piece");
        }
        return None;
    }

    pub fn check_end(
            &mut self, indx: &usize
        ) -> Result<u8, &'static str> {
        if self.movement.contains(&(*indx as u8)) ||
           self.capture.contains(&(*indx as u8)){
            self.movement.clear();
            self.capture.clear();
            self.selected = 64;
            return Ok(self.id[*indx]);
        }
        Err("Invalid movement for your piece")
    }

    pub fn eval_reach(
            &mut self, indx: &usize
        ) -> Option<&'static str> {    
        let mut pos: [i8; 2];        
        let mut new_indx: i8;
        if self.id[*indx] == 5 {
            let val: i8 = if self.color[*indx] == 0 {
                    -1
                } else {
                    1
                };
            pos = [
                    (*indx as i8)/8 + val,
                    (*indx as i8)% 8
                ];
            new_indx = pos[1] + pos[0] * 8;
            if self.id[new_indx as usize] == 6 {
                self.movement.push(new_indx as u8);
            } else {
                return Some("The piece cannot move");
            }
            if (self.pawn_moved[self.turn as usize] >> pos[1]) 
                & 1 == 0 {
                self.pawn_moved[self.turn as usize] |= 1 << pos[1];
                new_indx += val * 8;
                if self.id[new_indx as usize] == 6 {
                    self.movement.push(new_indx as u8);
                }
            } 
            self.selected = *indx as u8;
            return None;
        }
        let mut mov: Movement = PIECE_MOVEMENT[self.id[*indx] as usize];
        for rot in 0..mov.1 {  
            if rot == 4 {
                mov.0 = [
                    mov.0[0] - sign(mov.0[1]),
                    mov.0[1] + sign(mov.0[0])
                ];
            }              
            for reach in 1..(mov.2 + 1) {
                pos = [
                    (*indx as i8)/8 + reach * mov.0[0],
                    (*indx as i8)% 8 + reach * mov.0[1]
                ];
                if (pos[0] > -1 && pos[0] < 8) &&
                    (pos[1] > -1 && pos[1] < 8) {
                    new_indx = pos[1] + pos[0] * 8;
                    if self.id[new_indx as usize] == 6 {
                        self.movement.push(new_indx as u8);
                    } else {
                        if  self.color[new_indx as usize] ==
                            self.turn {
                            break;
                        } else {
                            self.capture.push(new_indx as u8);
                        }
                    }
                } else {
                    break;
                }
            }                
            mov.0 = [-mov.0[1], mov.0[0]];
        }
        if self.movement.len() != 0 || self.capture.len() != 0{
            self.selected = *indx as u8;
            return None;
        }else {
            return Some("The piece cannot move");
        }
    }

    pub fn move_piece(
            &mut self, start: &usize, end: &usize
        ) {
        self.id[*end] = self.id[*start];
        self.color[*end] = self.color[*start];
        self.id[*start] = 6;
        self.color[*start] = 2;
    }
}