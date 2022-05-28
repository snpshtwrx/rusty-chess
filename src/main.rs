type PC = chessboard::chessboard::PieceColor;
type GS = chessboard::chessboard::Gamestates;

pub mod chessboard;
pub mod movement;
pub mod settings;
pub mod engines;

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
    clear_terminal();
    let conf = settings::settings::Config::get_user_config();
    clear_terminal();
    let endstate = game_loop(INITIAL_BOARD_COLOR_STATE, INITIAL_BOARD_PIECE_STATE, conf.get_beautify());
    endstate.print_result();
}

fn game_loop(board_color_beginning: [[PC; 8]; 8], board_piece_beginning: [[char; 8]; 8], beautify: bool) -> GS {
    let mut board_color_state = board_color_beginning;
    let mut board_piece_state = board_piece_beginning;
    let mut whites_turn = true;
    let mut gamestate: GS = GS::Playing;
    let mut current_move: [usize; 4];
    let mut last_move: [usize; 4] = [0, 0, 0, 0];
    let mut legal_moves: Vec<[usize; 4]>;
    let mut piececount_log: Vec<u8> = Vec::new();
    let mut piececount: u8 = 0;
    chessboard::chessboard::draw_chessboard(&board_color_state, &board_piece_state, beautify);
    loop {
        if whites_turn {
            legal_moves =  movement::movement::get_legal_moves_white(board_color_state, board_piece_state, last_move, false);
        }
        else {
            legal_moves = movement::movement::get_legal_moves_black(board_color_state, board_piece_state, last_move, false);
        }
        loop {
            current_move = get_action(&legal_moves);
            clear_terminal();
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
        (board_color_state, board_piece_state) = update_board_state(current_move, &mut board_color_state, &mut board_piece_state);

        for i in 0..8 {
            piececount = 0;
            for j in 0..8 {
                match board_color_state[i][j] {
                    PC::None => (),
                    _ => piececount += 1,
                }
            }
        }
        piececount_log.push(piececount);

        gamestate = update_gamestate(&board_color_state, &board_piece_state, &piececount_log, &gamestate);

        match gamestate {
            GS::Playing => (),
            GS::Check(_white) => (),
            GS::Stalemate => break,
            GS::Mate(msg) => return GS::Mate(msg),
            GS::Draw(msg) => return GS::Draw(msg),
        }

        whites_turn = !whites_turn;

        clear_terminal();
        chessboard::chessboard::draw_chessboard(&board_color_state, &board_piece_state, beautify);

        last_move = current_move;
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

fn update_gamestate(colormap: &[[PC; 8]; 8], piecemap: &[[char; 8]; 8], piececount: &Vec<u8>, current_gamestate: &GS) -> GS {
    // Check for Check
    // Currently possible to win after only putting the other color in check, after the other color checked us
    // TODO: Fix the error stated above
    if movement::movement::is_square_checked_white(&movement::movement::get_legal_moves_black(*colormap, *piecemap, [0, 0, 0, 0], true), get_king_position_white(colormap, piecemap)) {
        match current_gamestate {
            GS::Check(white) => if !white { return GS::Mate("Black wins".to_string()) },
            _ => (),
        }
        return GS::Check(false);
    }
    if movement::movement::is_square_checked_black(&movement::movement::get_legal_moves_white(*colormap, *piecemap, [0, 0, 0, 0], true), get_king_position_black(colormap, piecemap)) {
        match current_gamestate {
            GS::Check(white) => if *white { return GS::Mate("White wins".to_string()) },
            _ => (),
        }
        return GS::Check(true);
    }
    // Threefold Repetition
    // 50 Moves without capturing
    if piececount.len() >= 50 {
        if piececount[piececount.len() - 1] == piececount[piececount.len() - 50] {
            return GS::Draw("50 moves without capturing".to_string());
        }
    }

    // Check for stalemate
    if movement::movement::get_legal_moves_white(*colormap, *piecemap, [0, 0, 0, 0], false).len() == 0 || movement::movement::get_legal_moves_black(*colormap, *piecemap, [0, 0, 0, 0], false).len() == 0 {
        return GS::Stalemate;
    }
    GS::Playing
}

fn update_board_state(current_move: [usize; 4], board_color_state: &mut [[PC; 8]; 8], board_piece_state: &mut [[char; 8]; 8]) -> ([[PC; 8]; 8], [[char; 8]; 8]) {
    // Handle queen-side casteling
    if board_piece_state[current_move[0]][current_move[1]] == 'K' && (current_move[3] as i8 - current_move[1] as i8) == -2 {
        board_color_state[current_move[0]][3] = board_color_state[current_move[0]][0];
        board_color_state[current_move[0]][0] = PC::None;
        board_piece_state[current_move[0]][3] = 'R';
        board_piece_state[current_move[0]][0] = ' ';
    }
    // King-side casteling
    else if board_piece_state[current_move[0]][current_move[1]] == 'K' && (current_move[3] as i8 - current_move[1] as i8) == 2 {
        board_color_state[current_move[0]][5] = board_color_state[current_move[0]][7];
        board_color_state[current_move[0]][7] = PC::None;
        board_piece_state[current_move[0]][5] = 'R';
        board_piece_state[current_move[0]][7] = ' ';
    }
    // Remove the pawn taken in en passent
    else if board_piece_state[current_move[0]][current_move[1]] == 'P' && board_piece_state[current_move[2]][current_move[3]] == ' ' && (current_move[3] as i8 - current_move[1] as i8).abs() == 1{
        board_color_state[(current_move[2] as i8 - (current_move[2] as i8 - current_move[0] as i8)) as usize][current_move[3]] = PC::None;
        board_piece_state[(current_move[2] as i8 - (current_move[2] as i8 - current_move[0] as i8)) as usize][current_move[3]] = ' ';
    }
    // Pawn promotion
    else if board_piece_state[current_move[0]][current_move[1]] == 'P' && (current_move[2] == 7 || current_move[2] == 0) {
                board_piece_state[current_move[0]][current_move[1]] = get_promotion_choice();
    }
    // Every other movement is handled in the following four statements
    board_color_state[current_move[2]][current_move[3]] = match board_color_state[current_move[0]][current_move[1]] {
        PC::White(_moved) => PC::White(true),
        PC::Black(_moved) => PC::Black(true),
        _ => PC::None,
    };
    board_color_state[current_move[0]][current_move[1]] = PC::None;
    board_piece_state[current_move[2]][current_move[3]] = board_piece_state[current_move[0]][current_move[1]];
    board_piece_state[current_move[0]][current_move[1]] = ' ';

    (*board_color_state, *board_piece_state)
}

fn get_promotion_choice() -> char {
    let mut promotion_choice = String::new();
    println!("Promote to Queen, Rook, Bishop or Knight? (Q/R/B/N):");
    std::io::stdin()
        .read_line(&mut promotion_choice)
        .expect("Failed to read line");
    promotion_choice.to_uppercase().trim().parse().expect("Invalid option")

}

fn get_king_position_white(colormap: &[[PC; 8]; 8], piecemap: &[[char; 8]; 8]) -> [usize; 2] {
    for i in 0..8 {
        for j in 0..8 {
            if piecemap[i][j] == 'K' {
                match colormap[i][j] {
                    PC::White(_moved) => return [i, j],
                    _ => (),
                }
            }
        }
    }
    [0, 0]
}

fn get_king_position_black(colormap: &[[PC; 8]; 8], piecemap: &[[char; 8]; 8]) -> [usize; 2] {
    for i in 0..8 {
        for j in 0..8 {
            if piecemap[i][j] == 'K' {
                match colormap[i][j] {
                    PC::Black(_moved) => return [i, j],
                    _ => (),
                }
            }
        }
    }
    [0, 0]
}

fn clear_terminal() {
    std::process::Command::new("clear").status().unwrap();
}
