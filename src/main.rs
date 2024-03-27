use std::io::{self};

mod lib;

fn main() {
    
    let mut board: [[char; 3]; 3] = [[' '; 3]; 3];
    let mut end_game: bool = false;
    let mut player_one_turn: bool = true;

    println!("Welcome to the tic tac toe game !");
    println!("Player 1, you are the 'X', and player 2, you are the 'O'. Enjoy the game !");

    while !end_game {

        if player_one_turn {println!("Player 1 turn")};

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

        lib::calculate_position(&mut board, position, player_one_turn);
        player_one_turn = lib::get_current_player(player_one_turn);

        end_game = false;
    
    }

}

