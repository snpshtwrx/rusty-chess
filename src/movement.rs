use crate::chessboard;

pub mod movement {
    use crate::PC;

    pub fn get_legal_moves_white(board_color_state: [[PC; 8]; 8], board_piece_state: [[char; 8]; 8], last_move: [usize; 4]) -> Vec<[usize; 4]> {
        let mut legal_moves: Vec<[usize; 4]> = Vec::new();
        let mut alive_pieces_position: Vec<[usize; 2]> = Vec::new();

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

        legal_moves
    }

    pub fn get_legal_moves_black(board_color_state: [[PC; 8]; 8], board_piece_state: [[char; 8]; 8], last_move: [usize; 4]) -> Vec<[usize; 4]> {
        let mut legal_moves: Vec<[usize; 4]> = Vec::new();
        let mut alive_pieces_position: Vec<[usize; 2]> = Vec::new();

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

        legal_moves
    }
}
