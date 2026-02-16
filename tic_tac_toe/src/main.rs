use std::io;

const  TRIO: [u16; 8] = [
    7, 56, 73, 84, 146, 273, 292, 448
];

fn show_game(arr: &[&str; 9]) {
    println!(
        "{} | {} | {}\n---------\n{} | {} | {}\n---------\n{} | {} | {}",
        arr[0], arr[1], arr[2], arr[3], arr[4],
        arr[5], arr[6], arr[7], arr[8]
    );
}

fn main() {
    println!("welcome to Tic-Tac-Toe");
    println!("do you want to play? y[yes]/ anything else[no]");

    let mut input_val: String = String::new(); 
    io::stdin()
        .read_line(&mut input_val)
        .expect("Failed to read line");

    if input_val.trim() != "y" {
        return;
    }

    print!("\x1bc");

    let mut game: [&str; 9] = [
        " ", " ", " ", " ", " ", " ", " ", " ", " "
    ];
    let mut player_num: u8 = 0;  

    let mut results: [u16; 2] = [0; 2];

    loop {
        println!("\x1b[91mTic\x1b[0m-\x1b[92mTac\x1b[0m-\x1b[94mToe\x1b[0m\n");
        show_game(&game);

        // game logic execution

        for l in TRIO {
            let player_index = (player_num + 1) % 2;
            if (results[player_index as usize] & l) == l {
                println!(
                    "\n\x1b[95mGame finished player\x1b[0m {} \x1b[95mwins!\x1b[0m",
                    player_index + 1
                );
                return;
            }
        }

        // game finished condition
        if results[0] | results[1] == 511 {
            println!("\n\x1b[95mGame finished in a tie!\x1b[0m");
            break;
        }

        print!("\n\x1b[95mPlayer\x1b[0m {} ", player_num + 1);
        println!("\x1b[95mto move\n\x1b[0m");
        println!("type the positon you want to play in or type q to close:");
        
       input_val = String::new();
        io::stdin()
            .read_line(&mut input_val)
            .expect("Failed to read line");
        if input_val.trim() == "q" {
            break;
        }

        let turn: u8 = match input_val.trim().parse::<u8>() {
            Ok(num) => {
                if num < 10 && num > 0 {
                    num - 1
                }
                else {
                    print!("\x1bc");
                    println!("\x1b[33mInvalid position!!!\x1b[0m\n");
                    continue
                }
            },
            Err(_) => {
                print!("\x1bc");
                println!("\x1b[33mInvalid position!!!\x1b[0m\n");
                continue
            },
        };

        let index:usize = turn as usize;
        if (results[0] | results[1]) & (1 << turn) == 0 {
            results[player_num as usize] |= 1 << turn;
        }
        else {
            print!("\x1bc");
            println!("\x1b[33mPosition already played!!!\x1b[0m\n");
            continue;
        }

        if player_num == 0{
            game[index] = "\x1b[96mX\x1b[0m";
        }
        else {
            game[index] = "\x1b[93mO\x1b[0m";
        }
        player_num = (player_num + 1) % 2;

        print!("\x1bc");
    }  
}