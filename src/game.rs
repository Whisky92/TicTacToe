#[derive(PartialEq)]
enum Color {
    RED, BLUE
}

#[derive(PartialEq)]
enum FieldType {
    CROSS, CIRCLE, UNOWNED
}

struct Game {
    current_player: Color,
    game_board: Vec<Vec<FieldType>>
}

impl Game {
    const STARTER_PLAYER: Color = Color::BLUE;
    const BOARD_SIZE: i8 = 3;

    pub fn new(&mut self) -> Game {
        Game {
            current_player: Self::STARTER_PLAYER,
            game_board: self.init_board()
        }
    }

    pub fn change_current_player(&mut self) {
        self.current_player =
            if self.current_player == Color::BLUE {
                Color::RED
            } else {
                Color::BLUE
            };
    }

    pub fn capture_cell(&mut self, row: i8, col: i8) {
        let field_type =
            if self.current_player == Self::STARTER_PLAYER {
                FieldType::CROSS
            } else {
                FieldType::CIRCLE
            };

        self.game_board[row][col] = field_type;
    }

    pub fn is_game_finished(&self) -> bool {
        self.is_any_row_filled() || self.is_any_col_filled() || self.is_any_diagonal_filled()
    }

    fn is_any_row_filled(&self) -> bool {
        self.game_board.iter().any(
            |row| row.iter().all(
                |cell| {
                    *cell != FieldType::UNOWNED && *cell != row[0]
                }
            )
        )
    }
    
    fn is_any_col_filled(&self) -> bool {
        for i  in 0..Self::BOARD_SIZE {
            for j in 0..Self::BOARD_SIZE {
                return !(self.game_board[j][i] != self.game_board[0][i]
                    || self.game_board[j][j] == FieldType::UNOWNED)
            }
        }
        true
    }

    fn is_any_diagonal_filled(&self) -> bool {
        (0..Self::BOARD_SIZE).all(|index|
            self.game_board[index][index] != FieldType::UNOWNED && self.game_board[index][index] == self.game_board[0][0])
        || (0..Self::BOARD_SIZE).all(|index|
            self.game_board[index][2-index] != FieldType::UNOWNED && self.game_board[index][2-index] == self.game_board[0][3])
    }

    fn init_board(&mut self) -> Vec<Vec<FieldType>> {
        let board = Vec::new();

        for _ in 0..Self::BOARD_SIZE {
            self.game_board.push(Vec::new());
            for i in 0..Self::BOARD_SIZE {
                self.game_board[i].push(FieldType::UNOWNED);
            }
        }
        board
    }
}