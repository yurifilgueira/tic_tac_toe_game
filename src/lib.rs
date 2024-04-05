pub mod game {

    use std::io::{self, Write};

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

        pub fn is_position_free(board: &[[char; 3]; 3], position: i32) -> bool {

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
    

    mod initial_message_game{

            pub fn display_introduction() {        
            println!("\nWelcome to the tic tac toe game !");
            println!("Player 1, you are the 'X', and player 2, you are the 'O'. Enjoy the game !\n");

            println!("Choose a number between 1 and 9 to define the position you want. See the corresponding number to the position below:");
            println!("               1 | 2 | 3");
            println!("              -----------");
            println!("               4 | 5 | 6");
            println!("              -----------");
            println!("               7 | 8 | 9\n");
        }
        
    }

    pub fn switch_player(player_one_turn: bool) -> bool {
        return !player_one_turn;
    }

        pub fn game_is_tie(board: [[char; 3]; 3]) -> bool {

        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == ' ' {
                    return false;
                }
            }
        } 
        return true;
    }

    pub fn verify_win(board: &[[char; 3]; 3], player_character: &char) -> bool { 
        
        if board[0][0] == board[1][1] && board[2][2] == *player_character{
            return true;
        }
        else {
            
            let mut count_row = 0;
            let mut count_column = 0;

            for i in 0..3 {
                for j in 0..3 {

                    if board[i][j] == *player_character {
                        count_row = count_row + 1;
                    }

                    if board[j][i] == *player_character {
                        count_column = count_column + 1;
                    }
                }

                if count_row == 3 || count_column == 3 {
                    return true;
                }
                else {
                    count_row = 0;
                    count_column = 0;
                }

            }           
        }

        return false;
    }

    pub fn start_game(){
        let mut board: [[char; 3]; 3] = [[' '; 3]; 3];
        let mut end_game: bool = false;
        let mut player_one_turn: bool = true;

        initial_message_game::display_introduction();

        println!("\n           --- 1, 2, 3 GO! ---\n");

        while !end_game {

            if player_one_turn {println!("          --- Player 1 turn ---\n")} else {println!("          --- Player 2 turn ---\n")};

            println!("               {} | {} | {}", board[0][0], board[0][1], board[0][2]);
            println!("              -----------");
            println!("               {} | {} | {}", board[1][0], board[1][1], board[1][2]);
            println!("              -----------");
            println!("               {} | {} | {}", board[2][0], board[2][1], board[2][2]);

            print!("\nInsert the position that you want: ");
            let _ = io::stdout().flush();
            
            let mut position: String = String::new();
            io::stdin().read_line(&mut position).expect("Fail to read input");

            let position: i32 = match position.trim().parse::<i32>() {
                Ok(numb) => {numb},
                Err(_) => {
                    println!("Invalid input.");
                    continue;
                }
            };

            if position::is_position_free(&board, position) {
                position::calculate_position(&mut board, position, player_one_turn);
                
                let player_character;
                
                if player_one_turn {
                    player_character = 'X';
                }
                else {
                    player_character = 'O';
                }
                
                if verify_win(&board, &player_character) {
                    println!("Some player wins!");
                    break;
                }
                
                player_one_turn = switch_player(player_one_turn);
            }
            else {
                println!("You can not choose this position!");
            }
            
            end_game = game_is_tie(board);

            // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        }

        print!("End game!");
    }
}
