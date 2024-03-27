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

pub fn get_current_player(player_one_turn: bool) -> bool {
    return !player_one_turn;
}