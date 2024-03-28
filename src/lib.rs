pub mod game {

    use std::io::{self};

    mod position {
        pub fn calculate_position(board: &mut [[char; 3]; 3], position: i32, player_one_turn: bool){
    
            let identifier: char = if player_one_turn {'X'} else {'O'};
    
            match position{
                1 => board[0][0] = identifier,
                2 => board[0][1] = identifier,
                3 => board[0][2] = identifier,
                4 => board[1][0] = identifier,
                5 => board[1][1] = identifier,
                6 => board[1][2] = identifier,
                7 => board[2][0] = identifier,
                8 => board[2][1] = identifier,
                9 => board[2][2] = identifier,
                _ => print!("Input error"),
            }
        }

        pub fn is_position_free(board: [[char; 3]; 3], position: i32) -> bool {

            match position{
                1 => if board[0][0] == ' ' {true} else {false},
                2 => if board[0][1] == ' ' {true} else {false},
                3 => if board[0][2] == ' ' {true} else {false},
                4 => if board[1][0] == ' ' {true} else {false},
                5 => if board[1][1] == ' ' {true} else {false},
                6 => if board[1][2] == ' ' {true} else {false},
                7 => if board[2][0] == ' ' {true} else {false},
                8 => if board[2][1] == ' ' {true} else {false},
                9 => if board[2][2] == ' ' {true} else {false},
                _ => false,
            }
        
        }

    }

pub fn get_current_player(player_one_turn: bool) -> bool {
    return !player_one_turn;
}

pub fn start_game(){
    let mut board: [[char; 3]; 3] = [[' '; 3]; 3];
    let mut end_game: bool = false;
    let mut player_one_turn: bool = true;

    println!("\nWelcome to the tic tac toe game !");
    println!("Player 1, you are the 'X', and player 2, you are the 'O'. Enjoy the game !\n");

    while !end_game {

        if player_one_turn {println!("          --- Player 1 turn ---\n")} else {println!("         -- Player 2 turn ---\n")};

        println!("               {} | {} | {}", board[0][0], board[0][1], board[0][2]);
        println!("              -----------");
        println!("               {} | {} | {}", board[1][0], board[1][1], board[1][2]);
        println!("              -----------");
        println!("               {} | {} | {}", board[2][0], board[2][1], board[2][2]);

        let mut position: String = String::new();
        io::stdin().read_line(&mut position).expect("Fail to read input");

        let position: i32 = match position.trim().parse::<i32>() {
            Ok(numb) => {numb},
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };

        if position::is_position_free(board, position) {
            position::calculate_position(&mut board, position, player_one_turn);
            player_one_turn = get_current_player(player_one_turn);
        }
        else {
            println!("You can not choose this position!");
        }
        end_game = false;
    
    }
}

}