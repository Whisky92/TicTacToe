#[derive(PartialEq)]
pub enum Color {
    RED, BLUE
}

#[derive(PartialEq)]
pub enum FieldType {
    CROSS, CIRCLE, UNOWNED
}

pub struct Game {
    current_player: Color,
    game_board: Vec<Vec<FieldType>>
}

impl Game {
    const STARTER_PLAYER: Color = Color::BLUE;
    const BOARD_SIZE: usize = 5;

    pub fn new() -> Self {
        Game {
            current_player: Self::STARTER_PLAYER,
            game_board: Self::init_board()
        }
    }

    pub fn get_current_player(&self) -> &Color { &self.current_player }

    pub fn get_board_size(&self) -> usize {
        Self::BOARD_SIZE
    }

    pub fn get_game_board(&self) -> &Vec<Vec<FieldType>> {
        &self.game_board
    }

    pub fn is_cell_occupied(&self, row: usize, col: usize) -> bool {
        self.game_board[row][col] != FieldType::UNOWNED
    }

    pub fn change_current_player(&mut self) {
        self.current_player =
            if self.current_player == Color::BLUE {
                Color::RED
            } else {
                Color::BLUE
            };
    }

    pub fn capture_cell(&mut self, row: usize, col: usize) {
        let field_type =
            if self.current_player == Self::STARTER_PLAYER {
                FieldType::CROSS
            } else {
                FieldType::CIRCLE
            };

        self.game_board[row][col] = field_type;
    }

    pub fn is_game_finished(&self) -> bool {
        let is_any_row_filled = self.is_any_row_filled();
        let is_any_col_filled = self.is_any_col_filled();
        let is_any_diagonal_filled = self.is_any_diagonal_filled();

        is_any_row_filled || is_any_col_filled || is_any_diagonal_filled
    }

    fn is_any_row_filled(&self) -> bool {
        self.game_board.iter().any(
            |row| row.iter().all(
                |cell| {
                    *cell != FieldType::UNOWNED && *cell == row[0]
                }
            )
        )
    }
    
    fn is_any_col_filled(&self) -> bool {
        for i in 0..Self::BOARD_SIZE {
            let mut is_filled = true;
            for j in 0..Self::BOARD_SIZE {
                if self.game_board[j][i] != self.game_board[0][i]
                    || self.game_board[j][i] == FieldType::UNOWNED {
                    is_filled = false;
                }
            }
            if is_filled {return true};
        }
        false
    }

    fn is_any_diagonal_filled(&self) -> bool {
        (0..Self::BOARD_SIZE).all(|index|
            self.game_board[index][index] != FieldType::UNOWNED && self.game_board[index][index] == self.game_board[0][0])
        || (0..Self::BOARD_SIZE).all(|index|
            self.game_board[index][Self::BOARD_SIZE - 1 - index] != FieldType::UNOWNED && self.game_board[index][Self::BOARD_SIZE - 1 - index] == self.game_board[0][Self::BOARD_SIZE - 1])
    }

    fn init_board() -> Vec<Vec<FieldType>> {
        let mut board = Vec::new();

        for i in 0..Self::BOARD_SIZE {
            board.push(Vec::new());
            for _ in 0..Self::BOARD_SIZE {
                board[i].push(FieldType::UNOWNED);
            }
        }
        board
    }
}