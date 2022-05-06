type PC = chessboard::chessboard::PieceColor;
type GS = chessboard::chessboard::Gamestates;

pub mod chessboard;
pub mod movement;

const INITIAL_BOARD_COLOR_STATE: [[PC; 8]; 8] = [
    [PC::Black(false); 8],
    [PC::Black(false); 8],
    [PC::None; 8],
    [PC::None; 8],
    [PC::None; 8],
    [PC::None; 8],
    [PC::White(false); 8],
    [PC::White(false); 8],
];

const INITIAL_BOARD_PIECE_STATE: [[char; 8]; 8] = [
    ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
    ['P'; 8],
    [' '; 8],
    [' '; 8],
    [' '; 8],
    [' '; 8],
    ['P'; 8],
    ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
];

fn main() {
    std::process::Command::new("clear").status().unwrap();
    let beautify = get_user_config();
    let endstate = game_loop(INITIAL_BOARD_COLOR_STATE, INITIAL_BOARD_PIECE_STATE, beautify);
    endstate.print_result();
}

fn get_user_config() -> bool {
    println!("Would you like the chess pieces to be beauty (true/false): ");
    let mut beautify = String::new();
    std::io::stdin()
        .read_line(&mut beautify)
        .expect("Failed to read line from user");

    let beautify: bool = beautify.trim().parse().expect("Please enter a valid boolean value");

    beautify
}

fn game_loop(board_color_beginning: [[PC; 8]; 8], board_piece_beginning: [[char; 8]; 8], beautify: bool) -> GS {
    let mut board_color_state = board_color_beginning;
    let mut board_piece_state = board_piece_beginning;
    let mut whites_turn = true;
    let mut gamestate: GS;
    let mut current_move: [usize; 4];
    let mut last_move: [usize; 4] = [0, 0, 0, 0];
    let mut legal_moves: Vec<[usize; 4]> = Vec::new();
    chessboard::chessboard::draw_chessboard(&board_color_state, &board_piece_state, beautify);
    loop {
        if whites_turn {
            legal_moves =  movement::movement::get_legal_moves_white(board_color_state, board_piece_state, last_move);
        }
        else {
            legal_moves = movement::movement::get_legal_moves_black(board_color_state, board_piece_state, last_move);
        }
        loop {
            current_move = get_action(&legal_moves);
            chessboard::chessboard::draw_chessboard_highlight(&board_color_state, &board_piece_state, beautify, current_move);
            println!("Enter to confirm: ");
            let mut confirm = String::new();
            std::io::stdin()
                .read_line(&mut confirm)
                .expect("Failed to read line from user");
            if confirm == "\n" {
                break;
            }
        }
        (board_color_state, board_piece_state) = update_color_state(current_move, &mut board_color_state, &mut board_piece_state);

        gamestate = update_gamestate(&board_color_state, &board_piece_state);

        match gamestate {
            GS::Playing => (),
            GS::Mate => (),
            GS::CheckMate => break,
            GS::Stalemate => break,
            GS::Draw => break,
        }

        whites_turn = !whites_turn;

        std::process::Command::new("clear").status().unwrap();
        chessboard::chessboard::draw_chessboard(&board_color_state, &board_piece_state, beautify);

        last_move = current_move;
        //break;
    }

    gamestate
}

fn get_action(legal_moves: &Vec<[usize; 4]>) -> [usize; 4] {
    println!("Choose from Option 0 to {}", legal_moves.len() - 1);
    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Could not read line!");
    let choice: usize = choice.trim().parse().expect("Not a valid value");
    legal_moves[choice]
}

fn get_legal_moves(whites_turn: bool, board_color_state: [[PC; 8]; 8], board_piece_state: [[char; 8]; 8], last_move: [usize; 4]) -> Vec<[usize; 4]> {
    let mut legal_moves: Vec<[usize; 4]> = Vec::new();
    let mut alive_pieces_position: Vec<[usize; 2]> = Vec::new();

    if whites_turn {
        for i in 0..8 {
            for j in 0..8 {
                match board_color_state[i][j] {
                    PC::White(_moved) => alive_pieces_position.push([i, j]),
                    _ => (),
                }
            }
        }

        for i in alive_pieces_position {
            if board_piece_state[i[0]][i[1]] == 'P' {
                // Is the square infront of the pawn not taken by any of the colors?
                // If so add moving one square ahead to the list of legal moves
                match board_color_state[i[0] - 1][i[1]] {
                    PC::None => legal_moves.push([i[0], i[1], i[0] - 1, i[1]]),
                    _ => (),
                }

                // Is the pawn still on its homerow?
                // Is the square two ahead not taken by any of the colors?
                if i[0] == 6 && board_piece_state[i[0] - 1][i[0]] == ' ' {
                    match board_color_state[i[0] - 2][i[1]] {
                        PC::None => legal_moves.push([i[0], i[1], i[0] - 2, i[1]]),
                        _ => (),
                    }
                }

                if i[0] == 3 && board_piece_state[last_move[2]][last_move[3]] == 'P' && last_move[2] == 3 && last_move[0] == 1 && ((last_move[3] as i8 - 1) == i[1] as i8 || (last_move[3] + 1) == i[1]) {
                    legal_moves.push([i[0], i[1], i[0] - 1, (i[1] as i8 + (last_move[3] as i8 - i[1] as  i8)) as usize])
                }

                if (i[0] as i8 - 1) >= 0 && (i[1] as i8 - 1) >= 0 {
                    match board_color_state[i[0] - 1][i[1] - 1] {
                        PC::Black(_moved) => legal_moves.push([i[0], i[1], i[0] - 1, i[1] - 1]),
                        _ => (),
                    }
                }
                if (i[0] as i8 - 1) >= 0 && (i[1] + 1) < 8 {
                    if let PC::Black(_moved) = board_color_state[i[0] - 1][i[1] + 1] {
                        legal_moves.push([i[0], i[1], i[0] - 1, i[1] + 1])
                    }
                }
            }
            else if board_piece_state[i[0]][i[1]] == 'B' {
                for j in 1..(i[0] as i8 + 1) {
                    // Movement towards top left
                    if (i[1] as i8 - j) > -1 {
                        match board_color_state[(i[0] as i8 - j) as usize][(i[1] as i8 - j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, (i[1] as i8 - j) as usize]),
                            PC::Black(_moved) => {
                                legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, (i[1] as i8 - j) as usize]);
                                break;
                            },
                            PC::White(_moved) => break,
                        }
                    }
                }
                for j in 1..(i[0] as i8 + 1) {
                    // Movement towards top right
                    if (i[1] as i8 + j) < 8 {
                        match board_color_state[(i[0] as i8 - j) as usize][(i[1] as i8 + j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] - j as usize, i[1] + j as usize]),
                            PC::Black(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] - j as usize, i[1] + j as usize]);
                                break;
                            },
                            PC::White(_moved) => break,
                        }
                    }
                }
                for j in 1..(8 - i[0] as i8) {
                    // Movement towards the bottom left
                    if (i[1] as i8 - j) > -1 {
                        match board_color_state[(i[0] as i8 + j) as usize][(i[1] as i8 - j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] - j as usize]),
                            PC::Black(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] - j as usize]);
                                break;
                            },
                            PC::White(_moved) => break,
                        }
                    }
                }
                for j in 1..(8 - i[0] as i8) {
                    if (i[1] as i8 + j) < 8 {
                        // Movement towards the bottom right
                        match board_color_state[(i[0] as i8 + j) as usize][(i[1] as i8 + j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] + j as usize]),
                            PC::Black(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] + j as usize]);
                                break;
                            },
                            PC::White(_moved) => break,
                        }
                    }
                }
            }
            else if board_piece_state[i[0]][i[1]] == 'N' {
                //todo!()
            }
            else if board_piece_state[i[0]][i[1]] == 'R' {
                for j in 1..(i[0] as i8 + 1) {
                    match board_color_state[(i[0] as i8 - j) as usize][i[1]] {
                        PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, i[1]]),
                        PC::Black(_moved) => {
                            legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, i[1]]);
                            break;
                        },
                        PC::White(_moved) => break,
                    }
                }
                for j in 1..(8 - i[0] as i8) {
                    match board_color_state[(i[0] as i8 + j) as usize][i[1]] {
                        PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 + j) as usize, i[1]]),
                        PC::Black(_moved) => {
                            legal_moves.push([i[0], i[1], (i[0] as i8 + j) as usize, i[1]]);
                            break;
                        },
                        PC::White(_moved) => break,
                    }
                }
                for j in 1..(i[1] as i8 + 1) {
                    match board_color_state[i[0]][(i[1] as i8 - j) as usize] {
                        PC::None => legal_moves.push([i[0], i[1], i[0], (i[1] as i8 - j) as usize]),
                        PC::Black(_moved) => {
                            legal_moves.push([i[0], i[1], i[0], (i[1] as i8 - j) as usize]);
                            break;
                        },
                        PC::White(_moved) => break,
                    }
                }
                for j in 1..(8 - i[1] as i8) {
                    match board_color_state[i[0]][(i[1] as i8 + j) as usize] {
                        PC::None => legal_moves.push([i[0], i[1], i[0], (i[1] as i8 + j) as usize]),
                        PC::Black(_moved) => {
                            legal_moves.push([i[0], i[1], i[0], (i[1] as i8 + j) as usize]);
                            break;
                        },
                        PC::White(_moved) => break,
                    } 
                }
            }
            else if board_piece_state[i[0]][i[1]] == 'Q' {
                for j in 1..(i[0] as i8 + 1) {
                    match board_color_state[(i[0] as i8 - j) as usize][i[1]] {
                        PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, i[1]]),
                        PC::Black(_moved) => {
                            legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, i[1]]);
                            break;
                        },
                        PC::White(_moved) => break,
                    }
                }
                for j in 1..(8 - i[0] as i8) {
                    match board_color_state[(i[0] as i8 + j) as usize][i[1]] {
                        PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 + j) as usize, i[1]]),
                        PC::Black(_moved) => {
                            legal_moves.push([i[0], i[1], (i[0] as i8 + j) as usize, i[1]]);
                            break;
                        },
                        PC::White(_moved) => break,
                    }
                }
                for j in 1..(i[1] as i8 + 1) {
                    match board_color_state[i[0]][(i[1] as i8 - j) as usize] {
                        PC::None => legal_moves.push([i[0], i[1], i[0], (i[1] as i8 - j) as usize]),
                        PC::Black(_moved) => {
                            legal_moves.push([i[0], i[1], i[0], (i[1] as i8 - j) as usize]);
                            break;
                        },
                        PC::White(_moved) => break,
                    }
                }
                for j in 1..(8 - i[1] as i8) {
                    match board_color_state[i[0]][(i[1] as i8 + j) as usize] {
                        PC::None => legal_moves.push([i[0], i[1], i[0], (i[1] as i8 + j) as usize]),
                        PC::Black(_moved) => {
                            legal_moves.push([i[0], i[1], i[0], (i[1] as i8 + j) as usize]);
                            break;
                        },
                        PC::White(_moved) => break,
                    } 
                }
                for j in 1..(i[0] as i8 + 1) {
                    // Movement towards top left
                    if (i[1] as i8 - j) > -1 {
                        match board_color_state[(i[0] as i8 - j) as usize][(i[1] as i8 - j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, (i[1] as i8 - j) as usize]),
                            PC::Black(_moved) => {
                                legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, (i[1] as i8 - j) as usize]);
                                break;
                            },
                            PC::White(_moved) => break,
                        }
                    }
                }
                for j in 1..(i[0] as i8 + 1) {
                    // Movement towards top right
                    if (i[1] as i8 + j) < 8 {
                        match board_color_state[(i[0] as i8 - j) as usize][(i[1] as i8 + j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] - j as usize, i[1] + j as usize]),
                            PC::Black(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] - j as usize, i[1] + j as usize]);
                                break;
                            },
                            PC::White(_moved) => break,
                        }
                    }
                }
                for j in 1..(8 - i[0] as i8) {
                    // Movement towards the bottom left
                    if (i[1] as i8 - j) > -1 {
                        match board_color_state[(i[0] as i8 + j) as usize][(i[1] as i8 - j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] - j as usize]),
                            PC::Black(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] - j as usize]);
                                break;
                            },
                            PC::White(_moved) => break,
                        }
                    }
                }
                for j in 1..(8 - i[0] as i8) {
                    if (i[1] as i8 + j) < 8 {
                        // Movement towards the bottom right
                        match board_color_state[(i[0] as i8 + j) as usize][(i[1] as i8 + j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] + j as usize]),
                            PC::Black(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] + j as usize]);
                                break;
                            },
                            PC::White(_moved) => break,
                        }
                    }
                }
            }
            else if board_piece_state[i[0]][i[1]] == 'K' {
                //todo!()
            }
        }
    }
    else {
        for i in 0..8 {
            for j in 0..8 {
                match board_color_state[i][j] {
                    PC::Black(_moved) => alive_pieces_position.push([i, j]),
                    _ => (),
                }
            }
        }

        for i in alive_pieces_position {
            if board_piece_state[i[0]][i[1]] == 'P' {
                // Is the square infront of the pawn not taken by any of the colors?
                // If so add moving one square ahead to the list of legal moves
                match board_color_state[i[0] + 1][i[1]] {
                    PC::None => legal_moves.push([i[0], i[1], i[0] + 1, i[1]]),
                    _ => (),
                }

                // Is the pawn still on its homerow?
                // Is the square two ahead not taken by any of the colors?
                if i[0] == 1 {
                    match board_color_state[i[0] + 2][i[1]] {
                        PC::None => legal_moves.push([i[0], i[1], i[0] + 2, i[1]]),
                        _ => (),
                    }
                }

                if i[0] == 4 && board_piece_state[last_move[2]][last_move[3]] == 'P' && last_move[2] == 4 && last_move[0] == 6 && ((last_move[3] as i8 - 1) == i[1] as i8 || (last_move[3] + 1) == i[1]) {
                    legal_moves.push([i[0], i[1], i[0] + 1, (i[1] as i8 + (last_move[3] as i8 - i[1] as  i8)) as usize])
                }

                if (i[0] as i8 + 1) < 8 && (i[1] as i8 - 1) >= 0{
                    match board_color_state[i[0] + 1][i[1] - 1] {
                        PC::White(_moved) => legal_moves.push([i[0], i[1], i[0] + 1, i[1] - 1]),
                        _ => (),
                    }
                }
                if (i[0] as i8 + 1) >= 0 && (i[1] + 1) < 8 {
                    match board_color_state[i[0] + 1][i[1] + 1] {
                        PC::White(_moved) => legal_moves.push([i[0], i[1], i[0] + 1, i[1] + 1]),
                        _ => (),
                    }
                }
            }
        }
    }

    legal_moves
}

fn update_gamestate(colormap: &[[PC; 8]; 8], piecemap: &[[char; 8]; 8]) -> GS {
    GS::Playing
}

fn update_color_state(current_move: [usize; 4], board_color_state: &mut [[PC; 8]; 8], board_piece_state: &mut [[char; 8]; 8]) -> ([[PC; 8]; 8], [[char; 8]; 8]) {
    if board_piece_state[current_move[0]][current_move[1]] == 'K' && (current_move[3] as i8 - current_move[1] as i8).abs() == 2{

    }
    else if board_piece_state[current_move[0]][current_move[1]] == 'P' && board_piece_state[current_move[2]][current_move[3]] == ' ' && (current_move[3] as i8 - current_move[1] as i8).abs() == 1{
        board_color_state[(current_move[2] as i8 - (current_move[2] as i8 - current_move[0] as i8)) as usize][current_move[3]] = PC::None;
        board_piece_state[(current_move[2] as i8 - (current_move[2] as i8 - current_move[0] as i8)) as usize][current_move[3]] = ' ';
    }
    board_color_state[current_move[2]][current_move[3]] = board_color_state[current_move[0]][current_move[1]];
    board_color_state[current_move[0]][current_move[1]] = PC::None;
    board_piece_state[current_move[2]][current_move[3]] = board_piece_state[current_move[0]][current_move[1]];
    board_piece_state[current_move[0]][current_move[1]] = ' ';

    (*board_color_state, *board_piece_state)
}
