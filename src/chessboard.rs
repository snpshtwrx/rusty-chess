pub mod chessboard {
    use ansi_term::Color;
    use ansi_term::Style;

    #[derive(Clone, Copy)]
    pub enum PieceColor {
        Black(bool),
        White(bool),
        None,
    }

    impl PieceColor {
        fn return_color(&self) -> ansi_term::Color {
            match self {
                PieceColor::White(_moved) => Color::RGB(254, 251, 234),
                PieceColor::Black(_moved) => Color::Black,
                _ => Color::White,
            }
        }
    }

    pub enum Gamestates {
        Playing,
        Draw(String),
        Stalemate,
        Check(bool),
        Mate(String),
    }

    impl Gamestates {
        pub fn print_result(&self) {
            match self {
                Gamestates::Mate(msg) => println!("Checkmate! {}", &msg),
                Gamestates::Stalemate => println!("Stalemate"),
                Gamestates::Draw(msg) => println!("Draw, due to {}", &msg),
                _ => println!("Unexpected termination of the game!"),
            }
        }
    }

    pub fn draw_chessboard(colormap: &[[PieceColor; 8]; 8], piecemap: &[[char; 8]; 8], beautify: bool) {
        let mut white = true;
        for i in 0..8 {
            for j in 0..8 {
                if white {
                    if beautify {
                        print!("{}", Style::new().on(Color::White).fg(colormap[i][j].return_color()).paint(get_piece_representation(piecemap[i][j])));
                    }
                    else {
                        print!("{}", Style::new().on(Color::White).fg(colormap[i][j].return_color()).paint(piecemap[i][j].to_string()));
                    }
                }
                else {
                    if beautify {
                        print!("{}", Style::new().on(Color::RGB(0, 153, 51)).fg(colormap[i][j].return_color()).paint(get_piece_representation(piecemap[i][j])));
                    }
                    else {
                        print!("{}", Style::new().on(Color::RGB(0, 153, 51)).fg(colormap[i][j].return_color()).paint(piecemap[i][j].to_string()));
                    }
                }
                white = !white;
            }
            white = !white;
            println!();
        }
    }

    pub fn draw_chessboard_highlight(colormap: &[[PieceColor; 8]; 8], piecemap: &[[char; 8]; 8], beautify: bool, higlighted_move: [usize; 4]) {
        let mut white = true;
        for i in 0..8 {
            for j in 0..8 {
                if white {
                    if beautify {
                        if (higlighted_move[0] == i && higlighted_move[1] == j) || (higlighted_move[2] == i && higlighted_move[3] == j) {
                            print!("{}", Style::new().on(Color::Red).fg(colormap[i][j].return_color()).paint(get_piece_representation(piecemap[i][j])));
                        }
                         else {
                             print!("{}", Style::new().on(Color::White).fg(colormap[i][j].return_color()).paint(get_piece_representation(piecemap[i][j])));
                         }
                    }
                    else {
                        if (higlighted_move[0] == i && higlighted_move[1] == j) || (higlighted_move[2] == i && higlighted_move[3] == j) {
                            print!("{}", Style::new().on(Color::Red).fg(colormap[i][j].return_color()).paint(get_piece_representation(piecemap[i][j])));
                        }
                        else {
                            print!("{}", Style::new().on(Color::White).fg(colormap[i][j].return_color()).paint(get_piece_representation(piecemap[i][j])));
                        }
                    }
                }
                else {
                    if beautify {
                        if (higlighted_move[0] == i && higlighted_move[1] == j) || (higlighted_move[2] == i && higlighted_move[3] == j) {
                            print!("{}", Style::new().on(Color::Red).fg(colormap[i][j].return_color()).paint(get_piece_representation(piecemap[i][j])));
                        }
                        else {
                            print!("{}", Style::new().on(Color::RGB(0, 153, 51)).fg(colormap[i][j].return_color()).paint(get_piece_representation(piecemap[i][j])));
                        }
                    }
                    else {
                        if (higlighted_move[0] == i && higlighted_move[1] == j) || (higlighted_move[2] == i && higlighted_move[3] == j) {
                            print!("{}", Style::new().on(Color::Red).fg(colormap[i][j].return_color()).paint(get_piece_representation(piecemap[i][j])));
                        }
                        else {
                            print!("{}", Style::new().on(Color::RGB(0, 153, 51)).fg(colormap[i][j].return_color()).paint(get_piece_representation(piecemap[i][j])));
                        }
                    }
                }
                white = !white;
            }
            white = !white;
            println!();
        }
    }

    fn get_piece_representation(piece: char) -> &'static str {
        match piece {
            'P' => "",
            'R' => "",
            'N' => "",
            'B' => "",
            'Q' => "",
            'K' => "",
            _ => " ",
        }
    }
}
