pub mod movement {
    use crate::PC;

    // Get all legal moves for the white pieces from a given board state
    pub fn get_legal_moves_white(board_color_state: [[PC; 8]; 8], board_piece_state: [[char; 8]; 8], last_move: [usize; 4], check_for_check: bool) -> Vec<[usize; 4]> {
        let mut legal_moves: Vec<[usize; 4]> = Vec::new();
        let alive_pieces_position: Vec<[usize; 2]> = get_alive_pieces_white(board_color_state);

        // Go through all the white pieeces that have been found to be on the bord
        for i in alive_pieces_position {
            // This checks for all possible moves that pawns can perform, except for promoting, which is handled separately
            if board_piece_state[i[0]][i[1]] == 'P' {
                // Check whether the square infront of the pawn is occupied, if it is not, then add this square to the legal moves
                match board_color_state[i[0] - 1][i[1]] {
                    PC::None => {
                        if !check_for_check {
                            legal_moves.push([i[0], i[1], i[0] - 1, i[1]]);
                        }
                },
                    _ => (),
                }

                // If the pawn is still in the homerow it is able to take two steps forward, unless these squares are blocked by other pieces
                if i[0] == 6 && board_piece_state[i[0] - 1][i[0]] == ' ' {
                    match board_color_state[i[0] - 2][i[1]] {
                        PC::None => {
                            if !check_for_check {
                                legal_moves.push([i[0], i[1], i[0] - 2, i[1]]);
                            }
                        }
                        _ => (),
                    }
                }

                // Check whether en passent is possible
                if i[0] == 3 && board_piece_state[last_move[2]][last_move[3]] == 'P' && last_move[2] == 3 && last_move[0] == 1 && ((last_move[3] as i8 - 1) == i[1] as i8 || (last_move[3] + 1) == i[1]) {
                    legal_moves.push([i[0], i[1], i[0] - 1, (i[1] as i8 + (last_move[3] as i8 - i[1] as  i8)) as usize])
                }

                // Is there another piece diagonally infront of the pawn (Left side)
                if (i[0] as i8 - 1) >= 0 && (i[1] as i8 - 1) >= 0 {
                    match board_color_state[i[0] - 1][i[1] - 1] {
                        PC::Black(_moved) => legal_moves.push([i[0], i[1], i[0] - 1, i[1] - 1]),
                        _ => (),
                    }
                }
                // Right side diagonally?
                if (i[0] as i8 - 1) >= 0 && (i[1] + 1) < 8 {
                    if let PC::Black(_moved) = board_color_state[i[0] - 1][i[1] + 1] {
                        legal_moves.push([i[0], i[1], i[0] - 1, i[1] + 1])
                    }
                }
            }
            // All moves bishops can perform
            else if board_piece_state[i[0]][i[1]] == 'B' {
                // Movement towards top left
                for j in 1..(i[0] as i8 + 1) {
                    if (i[1] as i8 - j) > -1 {
                        match board_color_state[(i[0] as i8 - j) as usize][(i[1] as i8 - j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, (i[1] as i8 - j) as usize]),
                            PC::Black(_moved) => {
                                legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, (i[1] as i8 - j) as usize]);
                                break;
                            },
                            PC::White(_moved) => {
                                if check_for_check {
                                    legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, (i[1] as i8 - j) as usize]);
                                }
                                break;
                            },
                        }
                    }
                }
                // Movement towards top right
                for j in 1..(i[0] as i8 + 1) {
                    if (i[1] as i8 + j) < 8 {
                        match board_color_state[(i[0] as i8 - j) as usize][(i[1] as i8 + j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] - j as usize, i[1] + j as usize]),
                            PC::Black(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] - j as usize, i[1] + j as usize]);
                                break;
                            },
                            PC::White(_moved) => {
                                if check_for_check {
                                    legal_moves.push([i[0], i[1], i[0] - j as usize, i[1] + j as usize]);
                                }
                                break;
                            },
                        }
                    }
                }
                // Movement towards the bottom left
                for j in 1..(8 - i[0] as i8) {
                    if (i[1] as i8 - j) > -1 {
                        match board_color_state[(i[0] as i8 + j) as usize][(i[1] as i8 - j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] - j as usize]),
                            PC::Black(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] - j as usize]);
                                break;
                            },
                            PC::White(_moved) => {
                                if check_for_check {
                                    legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] - j as usize]);
                                }
                                break;
                            },
                        }
                    }
                }
                // Movement towards the bottom right
                for j in 1..(8 - i[0] as i8) {
                    if (i[1] as i8 + j) < 8 {
                        match board_color_state[(i[0] as i8 + j) as usize][(i[1] as i8 + j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] + j as usize]),
                            PC::Black(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] + j as usize]);
                                break;
                            },
                            PC::White(_moved) => {
                                if check_for_check {
                                    legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] + j as usize]);
                                }
                                break;
                            },
                        }
                    }
                }
            }
            // Knight movement
            else if board_piece_state[i[0]][i[1]] == 'N' {
                // Up-left
                if (i[0] as i8 - 2) > -1 && (i[1] as i8 - 1) > -1 {
                    match board_color_state[i[0] - 2][i[1] - 1] {
                        PC::White(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] - 2, i[1] - 1]),
                    }
                }
                // Up-right
                if (i[0] as i8 - 2) > -1 && (i[1] as i8 + 1) < 8 {
                    match board_color_state[i[0] - 2][i[1] + 1] {
                        PC::White(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] - 2, i[1] + 1]),
                    }
                }
                // Right-up
                if (i[0] as i8 - 1) > -1 && (i[1] as i8 + 2) < 8 {
                    match board_color_state[i[0] - 1][i[1] + 2] {
                        PC::White(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] - 1, i[1] + 2]),
                    }
                }
                // Right-down
                if (i[0] as i8 + 1) < 8 && (i[1] as i8 + 2) < 8 {
                    match board_color_state[i[0] + 1][i[1] + 2] {
                        PC::White(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] + 1, i[1] + 2]),
                    }
                }
                // Down-right
                if (i[0] as i8 + 2) < 8 && (i[1] as i8 + 1) < 8 {
                    match board_color_state[i[0] + 2][i[1] + 1] {
                        PC::White(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] + 2, i[1] + 1]),
                    }
                }
                // Down-left
                if (i[0] as i8 + 2) < 8 && (i[1] as i8 - 1) > -1 {
                    match board_color_state[i[0] + 2][i[1] - 1] {
                        PC::White(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] + 2, i[1] - 1]),
                    }
                }
                // Left-down
                if (i[0] as i8 + 1) < 8 && (i[1] as i8 - 2) > -1 {
                    match board_color_state[i[0] + 1][i[1] - 2] {
                        PC::White(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] + 1, i[1] - 2]),
                    }
                }
                // Left-up
                if (i[0] as i8 - 1) > -1 && (i[1] as i8 - 2) > -1 {
                    match board_color_state[i[0] - 1][i[1] - 2] {
                        PC::White(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] - 1, i[1] - 2]),
                    }
                }
            }
            // Rook movement
            else if board_piece_state[i[0]][i[1]] == 'R' {
                // Upwards movement
                for j in 1..(i[0] as i8 + 1) {
                    match board_color_state[(i[0] as i8 - j) as usize][i[1]] {
                        PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, i[1]]),
                        PC::Black(_moved) => {
                            legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, i[1]]);
                            break;
                        },
                        PC::White(_moved) => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, i[1]]);
                            }
                            break;
                        },
                    }
                }
                // Downwards movement
                for j in 1..(8 - i[0] as i8) {
                    match board_color_state[(i[0] as i8 + j) as usize][i[1]] {
                        PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 + j) as usize, i[1]]),
                        PC::Black(_moved) => {
                            legal_moves.push([i[0], i[1], (i[0] as i8 + j) as usize, i[1]]);
                            break;
                        },
                        PC::White(_moved) => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], (i[0] as i8 + j) as usize, i[1]]);
                            }
                            break;
                        },
                    }
                }
                // Rightwards movement
                for j in 1..(i[1] as i8 + 1) {
                    match board_color_state[i[0]][(i[1] as i8 - j) as usize] {
                        PC::None => legal_moves.push([i[0], i[1], i[0], (i[1] as i8 - j) as usize]),
                        PC::Black(_moved) => {
                            legal_moves.push([i[0], i[1], i[0], (i[1] as i8 - j) as usize]);
                            break;
                        },
                        PC::White(_moved) => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], i[0], (i[1] as i8 - j) as usize]);
                            }
                            break;
                        },
                    }
                }
                // Leftwards movement
                for j in 1..(8 - i[1] as i8) {
                    match board_color_state[i[0]][(i[1] as i8 + j) as usize] {
                        PC::None => legal_moves.push([i[0], i[1], i[0], (i[1] as i8 + j) as usize]),
                        PC::Black(_moved) => {
                            legal_moves.push([i[0], i[1], i[0], (i[1] as i8 + j) as usize]);
                            break;
                        },
                        PC::White(_moved) => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], i[0], (i[1] as i8 + j) as usize]);
                            }
                            break;
                        },
                    }
                }
            }
            // Queen movement
            else if board_piece_state[i[0]][i[1]] == 'Q' {
                // Upwards movement
                for j in 1..(i[0] as i8 + 1) {
                    match board_color_state[(i[0] as i8 - j) as usize][i[1]] {
                        PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, i[1]]),
                        PC::Black(_moved) => {
                            legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, i[1]]);
                            break;
                        },
                        PC::White(_moved) => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, i[1]]);
                            }
                            break;
                        },
                    }
                }
                // Downwards movement
                for j in 1..(8 - i[0] as i8) {
                    match board_color_state[(i[0] as i8 + j) as usize][i[1]] {
                        PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 + j) as usize, i[1]]),
                        PC::Black(_moved) => {
                            legal_moves.push([i[0], i[1], (i[0] as i8 + j) as usize, i[1]]);
                            break;
                        },
                        PC::White(_moved) => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], (i[0] as i8 + j) as usize, i[1]]);
                            }
                            break;
                        },
                    }
                }
                // Rightwards movement
                for j in 1..(i[1] as i8 + 1) {
                    match board_color_state[i[0]][(i[1] as i8 - j) as usize] {
                        PC::None => legal_moves.push([i[0], i[1], i[0], (i[1] as i8 - j) as usize]),
                        PC::Black(_moved) => {
                            legal_moves.push([i[0], i[1], i[0], (i[1] as i8 - j) as usize]);
                            break;
                        },
                        PC::White(_moved) => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], i[0], (i[1] as i8 - j) as usize]);
                            }
                            break;
                        },
                    }
                }
                // Leftwards movement
                for j in 1..(8 - i[1] as i8) {
                    match board_color_state[i[0]][(i[1] as i8 + j) as usize] {
                        PC::None => legal_moves.push([i[0], i[1], i[0], (i[1] as i8 + j) as usize]),
                        PC::Black(_moved) => {
                            legal_moves.push([i[0], i[1], i[0], (i[1] as i8 + j) as usize]);
                            break;
                        },
                        PC::White(_moved) => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], i[0], (i[1] as i8 + j) as usize]);
                            }
                            break;
                        },
                    }
                }
                // Movement towards top left
                for j in 1..(i[0] as i8 + 1) {
                    if (i[1] as i8 - j) > -1 {
                        match board_color_state[(i[0] as i8 - j) as usize][(i[1] as i8 - j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, (i[1] as i8 - j) as usize]),
                            PC::Black(_moved) => {
                                legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, (i[1] as i8 - j) as usize]);
                                break;
                            },
                            PC::White(_moved) => {
                                if check_for_check {
                                    legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, (i[1] as i8 - j) as usize]);
                                }
                                break;
                            },
                        }
                    }
                }
                // Movement towards top right
                for j in 1..(i[0] as i8 + 1) {
                    if (i[1] as i8 + j) < 8 {
                        match board_color_state[(i[0] as i8 - j) as usize][(i[1] as i8 + j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] - j as usize, i[1] + j as usize]),
                            PC::Black(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] - j as usize, i[1] + j as usize]);
                                break;
                            },
                            PC::White(_moved) => {
                                if check_for_check {
                                    legal_moves.push([i[0], i[1], i[0] - j as usize, i[1] + j as usize]);
                                }
                                break;
                            },
                        }
                    }
                }
                // Movement towards the bottom left
                for j in 1..(8 - i[0] as i8) {
                    if (i[1] as i8 - j) > -1 {
                        match board_color_state[(i[0] as i8 + j) as usize][(i[1] as i8 - j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] - j as usize]),
                            PC::Black(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] - j as usize]);
                                break;
                            },
                            PC::White(_moved) => {
                                if check_for_check {
                                    legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] - j as usize]);
                                }
                                break;
                            },
                        }
                    }
                }
                // Movement towards the bottom right
                for j in 1..(8 - i[0] as i8) {
                    if (i[1] as i8 + j) < 8 {
                        match board_color_state[(i[0] as i8 + j) as usize][(i[1] as i8 + j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] + j as usize]),
                            PC::Black(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] + j as usize]);
                                break;
                            },
                            PC::White(_moved) => {
                                if check_for_check {
                                    legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] + j as usize]);
                                }
                                break;
                            },
                        }
                    }
                }
            }
            // King movement
            else if board_piece_state[i[0]][i[1]] == 'K' {
                let legal_moves_black: Vec<[usize; 4]> = if !check_for_check { get_legal_moves_black(board_color_state, board_piece_state, [0usize, 0usize, 0usize, 0usize], true) } else { Vec::new() };
                // Up
                if (i[0] as i8 - 1) > -1 && (if !check_for_check { !is_square_checked_black(&legal_moves_black, [i[0] - 1, i[1]]) } else { true }) {
                    match board_color_state[i[0] - 1][i[1]] {
                        PC::White(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] - 1, i[1]]),
                    }
                }
                // Up-right
                if (i[0] as i8 - 1) > -1 && (i[1] as i8 + 1) < 8 && (if !check_for_check { !is_square_checked_black(&legal_moves_black, [i[0] - 1, i[1] + 1])} else { true }) {
                    match board_color_state[i[0] - 1][i[1] + 1] {
                        PC::White(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] - 1, i[1] + 1]),
                    }
                }
                // Right
                if(i[1] as i8 + 1) < 8 && if check_for_check { !is_square_checked_black(&legal_moves, [i[0], i[1] + 1]) } else { true } {
                    match board_color_state[i[0]][i[1] + 1] {
                        PC::White(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0], i[1] + 1]),
                    }
                }
                // Down-right
                if (i[0] as i8 + 1) < 8 && (i[1] as i8 + 1) < 8 && if !check_for_check { !is_square_checked_black(&legal_moves_black, [i[0] + 1, i[1] + 1])} else { true } {
                    match board_color_state[i[0] + 1][i[1] + 1] {
                        PC::White(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] + 1, i[1] + 1]),
                    }
                }
                // Down
                if (i[0] as i8 + 1) < 8 && if !check_for_check { !is_square_checked_black(&legal_moves_black, [i[0] + 1, i[1]]) } else { true } {
                    match board_color_state[i[0] + 1][i[1]] {
                        PC::White(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] + 1, i[1]]),
                    }
                }
                // Down-left
                if (i[0] as i8 + 1) < 8 && (i[1] as i8 - 1) > -1 && if !check_for_check { !is_square_checked_black(&legal_moves_black, [i[0] + 1, i[1] - 1])} else { true } {
                    match board_color_state[i[0] + 1][i[1] - 1] {
                        PC::White(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] + 1, i[1] - 1]),
                    }
                }
                // Left
                if (i[1] as i8 - 1) > -1 && if !check_for_check { !is_square_checked_black(&legal_moves, [i[0], i[1] - 1]) } else { true } {
                    match board_color_state[i[0]][i[1] - 1] {
                        PC::White(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0], i[1] - 1]),
                    }
                }
                // Up-left
                if (i[0] as i8 - 1) > -1 && (i[1] as i8 - 1) > -1 && if !check_for_check { !is_square_checked_black(&legal_moves_black, [i[0] - 1, i[1] - 1])} else { true } {
                    match board_color_state[i[0] - 1][i[1] - 1] {
                        PC::White(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] - 1, i[1] - 1]),
                    }
                }
                // Left castle
                match board_color_state[i[0]][0] {
                    PC::White(moved) => {
                        if !moved && board_piece_state[i[0]][3] == ' ' && board_piece_state[i[0]][2] == ' ' && board_piece_state[i[0]][1] == ' ' {
                            match board_color_state[i[0]][4] {
                                PC::White(king_moved) => {
                                    if !king_moved {
                                        legal_moves.push([i[0], i[1], i[0], i[1] - 2]);
                                    }
                                },
                                _ => (),
                            }
                        }
                    },
                    _ => (),
                }
                // Right castle
                match board_color_state[i[0]][7] {
                    PC::White(moved) => {
                        if !moved && board_piece_state[i[0]][5] == ' ' && board_piece_state[i[0]][6] == ' ' {
                            match board_color_state[i[0]][4] {
                                PC::White(king_moved) => {
                                    if !king_moved {
                                        legal_moves.push([i[0], i[1], i[0], i[1] + 2]);
                                    }
                                },
                                _ => (),
                            }
                        }
                    },
                    _ => (),
                }
            }
        }

        legal_moves
    }

    pub fn get_alive_pieces_white(board_color_state: [[PC; 8]; 8]) -> Vec<[usize; 2]> {
        let mut alive_pieces_position: Vec<[usize; 2]> = Vec::new();
        // Gather the positions of all the white pieces
        for i in 0..8 {
            for j in 0..8 {
                match board_color_state[i][j] {
                    PC::White(_moved) => alive_pieces_position.push([i, j]),
                    _ => (),
                }
            }
        }

        alive_pieces_position
    }

    pub fn is_square_checked_white(legal_moves: &Vec<[usize; 4]>, square_to_check: [usize; 2]) -> bool {
        for i in legal_moves {
            if i[2] == square_to_check[0] && i[3] == square_to_check[1] {
                return true
            }
        }
        false
    }

    pub fn get_legal_moves_black(board_color_state: [[PC; 8]; 8], board_piece_state: [[char; 8]; 8], last_move: [usize; 4], check_for_check: bool) -> Vec<[usize; 4]> {
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
                    PC::None => {
                        if !check_for_check {
                            legal_moves.push([i[0], i[1], i[0] + 1, i[1]]);
                        }
                    },
                    _ => (),
                }
                // Is the pawn still on its homerow?
                // Is the square two ahead not taken by any of the colors?
                if i[0] == 1 {
                    match board_color_state[i[0] + 2][i[1]] {
                        PC::None => {
                            if !check_for_check {
                                legal_moves.push([i[0], i[1], i[0] + 2, i[1]]);
                            }
                        },
                        _ => (),
                    }
                }
                if i[0] == 4 && board_piece_state[last_move[2]][last_move[3]] == 'P' && last_move[2] == 4 && last_move[0] == 6 && ((last_move[3] as i8 - 1) == i[1] as i8 || (last_move[3] + 1) == i[1]) {
                    legal_moves.push([i[0], i[1], i[0] + 1, (i[1] as i8 + (last_move[3] as i8 - i[1] as  i8)) as usize])
                }
                if (i[0] as i8 + 1) < 8 && (i[1] as i8 - 1) >= 0{
                    match board_color_state[i[0] + 1][i[1] - 1] {
                        PC::White(_moved) => legal_moves.push([i[0], i[1], i[0] + 1, i[1] - 1]),
                        _ => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], i[0] + 1, i[1] - 1]);
                            }
                        },
                    }
                }
                if (i[0] as i8 + 1) >= 0 && (i[1] + 1) < 8 {
                    match board_color_state[i[0] + 1][i[1] + 1] {
                        PC::White(_moved) => legal_moves.push([i[0], i[1], i[0] + 1, i[1] + 1]),
                        _ => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], i[0] + 1, i[1] + 1]);
                            }
                        },
                    }
                }
            }
            // All moves bishops can perform
            else if board_piece_state[i[0]][i[1]] == 'B' {
                // Movement towards top left
                for j in 1..(i[0] as i8 + 1) {
                    if (i[1] as i8 - j) > -1 {
                        match board_color_state[(i[0] as i8 - j) as usize][(i[1] as i8 - j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, (i[1] as i8 - j) as usize]),
                            PC::White(_moved) => {
                                legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, (i[1] as i8 - j) as usize]);
                                break;
                            },
                            PC::Black(_moved) => {
                                if check_for_check {
                                    legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, (i[1] as i8 - j) as usize]);
                                }
                                break;
                            },
                        }
                    }
                }
                // Movement towards top right
                for j in 1..(i[0] as i8 + 1) {
                    if (i[1] as i8 + j) < 8 {
                        match board_color_state[(i[0] as i8 - j) as usize][(i[1] as i8 + j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] - j as usize, i[1] + j as usize]),
                            PC::White(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] - j as usize, i[1] + j as usize]);
                                break;
                            },
                            PC::Black(_moved) => {
                                if check_for_check {
                                    legal_moves.push([i[0], i[1], i[0] - j as usize, i[1] + j as usize]);
                                }
                                break;
                            },
                        }
                    }
                }
                // Movement towards the bottom left
                for j in 1..(8 - i[0] as i8) {
                    if (i[1] as i8 - j) > -1 {
                        match board_color_state[(i[0] as i8 + j) as usize][(i[1] as i8 - j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] - j as usize]),
                            PC::White(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] - j as usize]);
                                break;
                            },
                            PC::Black(_moved) => {
                                if check_for_check {
                                    legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] - j as usize]);
                                }
                                break;
                            },
                        }
                    }
                }
                // Movement towards the bottom right
                for j in 1..(8 - i[0] as i8) {
                    if (i[1] as i8 + j) < 8 {
                        match board_color_state[(i[0] as i8 + j) as usize][(i[1] as i8 + j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] + j as usize]),
                            PC::White(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] + j as usize]);
                                break;
                            },
                            PC::Black(_moved) => {
                                if check_for_check {
                                    legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] + j as usize]);
                                }
                                break;
                            },
                        }
                    }
                }
            }
            // Knight movement
            else if board_piece_state[i[0]][i[1]] == 'N' {
                // Up-left
                if (i[0] as i8 - 2) > -1 && (i[1] as i8 - 1) > -1 {
                    match board_color_state[i[0] - 2][i[1] - 1] {
                        PC::Black(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] - 2, i[1] - 1]),
                    }
                }
                // Up-right
                if (i[0] as i8 - 2) > -1 && (i[1] as i8 + 1) < 8 {
                    match board_color_state[i[0] - 2][i[1] + 1] {
                        PC::Black(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] - 2, i[1] + 1]),
                    }
                }
                // Right-up
                if (i[0] as i8 - 1) > -1 && (i[1] as i8 + 2) < 8 {
                    match board_color_state[i[0] - 1][i[1] + 2] {
                        PC::Black(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] - 1, i[1] + 2]),
                    }
                }
                // Right-down
                if (i[0] as i8 + 1) < 8 && (i[1] as i8 + 2) < 8 {
                    match board_color_state[i[0] + 1][i[1] + 2] {
                        PC::Black(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] + 1, i[1] + 2]),
                    }
                }
                // Down-right
                if (i[0] as i8 + 2) < 8 && (i[1] as i8 + 1) < 8 {
                    match board_color_state[i[0] + 2][i[1] + 1] {
                        PC::Black(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] + 2, i[1] + 1]),
                    }
                }
                // Down-left
                if (i[0] as i8 + 2) < 8 && (i[1] as i8 - 1) > -1 {
                    match board_color_state[i[0] + 2][i[1] - 1] {
                        PC::Black(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] + 2, i[1] - 1]),
                    }
                }
                // Left-down
                if (i[0] as i8 + 1) < 8 && (i[1] as i8 - 2) > -1 {
                    match board_color_state[i[0] + 1][i[1] - 2] {
                        PC::Black(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] + 1, i[1] - 2]),
                    }
                }
                // Left-up
                if (i[0] as i8 - 1) > -1 && (i[1] as i8 - 2) > -1 {
                    match board_color_state[i[0] - 1][i[1] - 2] {
                        PC::Black(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] - 1, i[1] - 2]),
                    }
                }
            }
            // Rook movement
            else if board_piece_state[i[0]][i[1]] == 'R' {
                // Upwards movement
                for j in 1..(i[0] as i8 + 1) {
                    match board_color_state[(i[0] as i8 - j) as usize][i[1]] {
                        PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, i[1]]),
                        PC::White(_moved) => {
                            legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, i[1]]);
                            break;
                        },
                        PC::Black(_moved) => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, i[1]]);
                            }
                            break;
                        },
                    }
                }
                // Downwards movement
                for j in 1..(8 - i[0] as i8) {
                    match board_color_state[(i[0] as i8 + j) as usize][i[1]] {
                        PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 + j) as usize, i[1]]),
                        PC::White(_moved) => {
                            legal_moves.push([i[0], i[1], (i[0] as i8 + j) as usize, i[1]]);
                            break;
                        },
                        PC::Black(_moved) => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], (i[0] as i8 + j) as usize, i[1]]);
                            }
                            break;
                        },
                    }
                }
                // Rightwards movement
                for j in 1..(i[1] as i8 + 1) {
                    match board_color_state[i[0]][(i[1] as i8 - j) as usize] {
                        PC::None => legal_moves.push([i[0], i[1], i[0], (i[1] as i8 - j) as usize]),
                        PC::White(_moved) => {
                            legal_moves.push([i[0], i[1], i[0], (i[1] as i8 - j) as usize]);
                            break;
                        },
                        PC::Black(_moved) => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], i[0], (i[1] as i8 - j) as usize]);
                            }
                            break;
                        },
                    }
                }
                // Leftwards movement
                for j in 1..(8 - i[1] as i8) {
                    match board_color_state[i[0]][(i[1] as i8 + j) as usize] {
                        PC::None => legal_moves.push([i[0], i[1], i[0], (i[1] as i8 + j) as usize]),
                        PC::White(_moved) => {
                            legal_moves.push([i[0], i[1], i[0], (i[1] as i8 + j) as usize]);
                            break;
                        },
                        PC::Black(_moved) => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], i[0], (i[1] as i8 + j) as usize]);
                            }
                            break;
                        },
                    }
                }
            }
            // Queen movement
            else if board_piece_state[i[0]][i[1]] == 'Q' {
                // Upwards movement
                for j in 1..(i[0] as i8 + 1) {
                    match board_color_state[(i[0] as i8 - j) as usize][i[1]] {
                        PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, i[1]]),
                        PC::White(_moved) => {
                            legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, i[1]]);
                            break;
                        },
                        PC::Black(_moved) => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, i[1]]);
                            }
                            break;
                        },
                    }
                }
                // Downwards movement
                for j in 1..(8 - i[0] as i8) {
                    match board_color_state[(i[0] as i8 + j) as usize][i[1]] {
                        PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 + j) as usize, i[1]]),
                        PC::White(_moved) => {
                            legal_moves.push([i[0], i[1], (i[0] as i8 + j) as usize, i[1]]);
                            break;
                        },
                        PC::Black(_moved) => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], (i[0] as i8 + j) as usize, i[1]]);
                            }
                            break;
                        },
                    }
                }
                // Rightwards movement
                for j in 1..(i[1] as i8 + 1) {
                    match board_color_state[i[0]][(i[1] as i8 - j) as usize] {
                        PC::None => legal_moves.push([i[0], i[1], i[0], (i[1] as i8 - j) as usize]),
                        PC::White(_moved) => {
                            legal_moves.push([i[0], i[1], i[0], (i[1] as i8 - j) as usize]);
                            break;
                        },
                        PC::Black(_moved) => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], i[0], (i[1] as i8 - j) as usize]);
                            }
                            break;
                        },
                    }
                }
                // Leftwards movement
                for j in 1..(8 - i[1] as i8) {
                    match board_color_state[i[0]][(i[1] as i8 + j) as usize] {
                        PC::None => legal_moves.push([i[0], i[1], i[0], (i[1] as i8 + j) as usize]),
                        PC::White(_moved) => {
                            legal_moves.push([i[0], i[1], i[0], (i[1] as i8 + j) as usize]);
                            break;
                        },
                        PC::Black(_moved) => {
                            if check_for_check {
                                legal_moves.push([i[0], i[1], i[0], (i[1] as i8 + j) as usize]);
                            }
                            break;
                        },
                    }
                }
                // Movement towards top left
                for j in 1..(i[0] as i8 + 1) {
                    if (i[1] as i8 - j) > -1 {
                        match board_color_state[(i[0] as i8 - j) as usize][(i[1] as i8 - j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, (i[1] as i8 - j) as usize]),
                            PC::White(_moved) => {
                                legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, (i[1] as i8 - j) as usize]);
                                break;
                            },
                            PC::Black(_moved) => {
                                if check_for_check {
                                    legal_moves.push([i[0], i[1], (i[0] as i8 - j) as usize, (i[1] as i8 - j) as usize]);
                                }
                                break;
                            },
                        }
                    }
                }
                // Movement towards top right
                for j in 1..(i[0] as i8 + 1) {
                    if (i[1] as i8 + j) < 8 {
                        match board_color_state[(i[0] as i8 - j) as usize][(i[1] as i8 + j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] - j as usize, i[1] + j as usize]),
                            PC::White(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] - j as usize, i[1] + j as usize]);
                                break;
                            },
                            PC::Black(_moved) => {
                                if check_for_check {
                                    legal_moves.push([i[0], i[1], i[0] - j as usize, i[1] + j as usize]);
                                }
                                break;
                            },
                        }
                    }
                }
                // Movement towards the bottom left
                for j in 1..(8 - i[0] as i8) {
                    if (i[1] as i8 - j) > -1 {
                        match board_color_state[(i[0] as i8 + j) as usize][(i[1] as i8 - j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] - j as usize]),
                            PC::White(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] - j as usize]);
                                break;
                            },
                            PC::Black(_moved) => {
                                if check_for_check {
                                    legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] - j as usize]);
                                }
                                break;
                            },
                        }
                    }
                }
                // Movement towards the bottom right
                for j in 1..(8 - i[0] as i8) {
                    if (i[1] as i8 + j) < 8 {
                        match board_color_state[(i[0] as i8 + j) as usize][(i[1] as i8 + j) as usize] {
                            PC::None => legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] + j as usize]),
                            PC::White(_moved) => {
                                legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] + j as usize]);
                                break;
                            },
                            PC::Black(_moved) => {
                                if check_for_check {
                                    legal_moves.push([i[0], i[1], i[0] + j as usize, i[1] + j as usize]);
                                }
                                break;
                            },
                        }
                    }
                }
            }
            // King movement
            else if board_piece_state[i[0]][i[1]] == 'K' {
                let legal_moves_black: Vec<[usize; 4]> = if !check_for_check { get_legal_moves_white(board_color_state, board_piece_state, [0usize, 0usize, 0usize, 0usize], true) } else { Vec::new() };
                // Up
                if (i[0] as i8 - 1) > -1 && (if !check_for_check { !is_square_checked_white(&legal_moves_black, [i[0] - 1, i[1]]) } else { true }) {
                    match board_color_state[i[0] - 1][i[1]] {
                        PC::Black(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] - 1, i[1]]),
                    }
                }
                // Up-right
                if (i[0] as i8 - 1) > -1 && (i[1] as i8 + 1) < 8 && (if !check_for_check { !is_square_checked_white(&legal_moves_black, [i[0] - 1, i[1] + 1])} else { true }) {
                    match board_color_state[i[0] - 1][i[1] + 1] {
                        PC::Black(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] - 1, i[1] + 1]),
                    }
                }
                // Right
                if(i[1] as i8 + 1) < 8 && if check_for_check { !is_square_checked_white(&legal_moves, [i[0], i[1] + 1]) } else { true } {
                    match board_color_state[i[0]][i[1] + 1] {
                        PC::Black(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0], i[1] + 1]),
                    }
                }
                // Down-right
                if (i[0] as i8 + 1) < 8 && (i[1] as i8 + 1) < 8 && if !check_for_check { !is_square_checked_white(&legal_moves_black, [i[0] + 1, i[1] + 1])} else { true } {
                    match board_color_state[i[0] + 1][i[1] + 1] {
                        PC::Black(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] + 1, i[1] + 1]),
                    }
                }
                // Down
                if (i[0] as i8 + 1) < 8 && if !check_for_check { !is_square_checked_white(&legal_moves_black, [i[0] + 1, i[1]]) } else { true } {
                    match board_color_state[i[0] + 1][i[1]] {
                        PC::Black(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] + 1, i[1]]),
                    }
                }
                // Down-left
                if (i[0] as i8 + 1) < 8 && (i[1] as i8 - 1) > -1 && if !check_for_check { !is_square_checked_white(&legal_moves_black, [i[0] + 1, i[1] - 1])} else { true } {
                    match board_color_state[i[0] + 1][i[1] - 1] {
                        PC::Black(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] + 1, i[1] - 1]),
                    }
                }
                // Left
                if (i[1] as i8 - 1) > -1 && if !check_for_check { !is_square_checked_white(&legal_moves, [i[0], i[1] - 1]) } else { true } {
                    match board_color_state[i[0]][i[1] - 1] {
                        PC::Black(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0], i[1] - 1]),
                    }
                }
                // Up-left
                if (i[0] as i8 - 1) > -1 && (i[1] as i8 - 1) > -1 && if !check_for_check { !is_square_checked_white(&legal_moves_black, [i[0] - 1, i[1] - 1])} else { true } {
                    match board_color_state[i[0] - 1][i[1] - 1] {
                        PC::Black(_moved) => (),
                        _ => legal_moves.push([i[0], i[1], i[0] - 1, i[1] - 1]),
                    }
                }
                // Left castle
                match board_color_state[i[0]][0] {
                    PC::Black(moved) => {
                        if !moved && board_piece_state[i[0]][3] == ' ' && board_piece_state[i[0]][2] == ' ' && board_piece_state[i[0]][1] == ' ' {
                            match board_color_state[i[0]][4] {
                                PC::Black(king_moved) => {
                                    if !king_moved {
                                        legal_moves.push([i[0], i[1], i[0], i[1] - 2]);
                                    }
                                },
                                _ => (),
                            }
                        }
                    },
                    _ => (),
                }
                // Right castle
                match board_color_state[i[0]][7] {
                    PC::Black(moved) => {
                        if !moved && board_piece_state[i[0]][5] == ' ' && board_piece_state[i[0]][6] == ' ' {
                            match board_color_state[i[0]][4] {
                                PC::Black(king_moved) => {
                                    if !king_moved {
                                        legal_moves.push([i[0], i[1], i[0], i[1] + 2]);
                                    }
                                },
                                _ => (),
                            }
                        }
                    },
                    _ => (),
                }
            }
        }

        legal_moves
    }

    pub fn is_square_checked_black(legal_moves: &Vec<[usize; 4]>, square_to_check: [usize; 2]) -> bool {
        for i in legal_moves {
            if i[2] == square_to_check[0] && i[3] == square_to_check[1] {
                return true
            }
        }
        false
    }
}
