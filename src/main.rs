use std::io::{self};

fn main() {
    
    let mut board: [[char; 3]; 3] = [[' '; 3]; 3];

    while true {

        println!(" {} | {} | {}", board[0][0], board[0][1], board[0][2]);
        println!("-----------");
        println!(" {} | {} | {}", board[1][0], board[1][1], board[1][2]);
        println!("-----------");
        println!(" {} | {} | {}", board[2][0], board[2][1], board[2][2]);

        let mut position: String = String::new();
        io::stdin().read_line(&mut position).expect("Fail to read input");

        let position: i32 = match position.trim().parse::<i32>() {
            Ok(numb) => {numb},
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };

        calculate_position(&mut board, position);
    
    }

}

fn calculate_position(board: &mut [[char; 3]; 3], position: i32){

    if position == 1 {
        board[0][0] = 'X';
    }

}