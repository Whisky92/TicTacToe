use crate::game::{FieldType, Game};
use std::io;

pub struct GameController {
    game: Game
}

impl GameController {
    pub fn new(game: Game) -> GameController {
        GameController { game }
    }

    pub fn handle_game_flow(&mut self) {
        self.draw_game_state();

        while(!self.game.is_game_finished()) {
            let (row, col) = self.read_input();
            self.game.capture_cell(row, col);
            self.draw_game_state();
            self.game.change_current_player();
        }
        println!("Game over!");
    }

    pub fn read_input(&self) -> (usize, usize) {
        let mut row: usize;
        let mut col: usize;

        let mut row_input;
        let mut col_input;

        loop {
            row_input = String::new();
            col_input = String::new();

            println!("Please enter row (0-2): ");
            io::stdin().read_line(&mut row_input).expect("Failed to read line");

            match row_input.trim().parse::<usize>() {
                Ok(number) => {
                    if (number < 0 || number > 2) {
                        println!("Invalid input");
                        continue;
                    }
                    row = number;
                }
                Err(_) => {
                    println!("Invalid input. Please enter a valid integer.");
                    continue;
                }
            }

            println!("Please enter column (0-2): ");
            io::stdin().read_line(&mut col_input).expect("Failed to read line");

            match col_input.trim().parse::<usize>() {
                Ok(number) => {
                    if (number < 0 || number > 2) {
                        println!("Invalid input");
                        continue;
                    }
                    if (self.game.is_cell_occupied(row, number)) {
                        println!("Cell occupied!");
                        continue;
                    }
                    col = number;
                }
                Err(_) => {
                    println!("Invalid input. Please enter a valid integer.");
                    continue;
                }
            }
            break;
        }

        (row, col)
    }


    pub fn draw_game_state(&self) {
        for i in 0..self.game.get_board_size() {
            let mut current_row = String::from("| ");
            for j in 0..self.game.get_board_size() {
                current_row += self.get_cell_content(i, j);
                current_row += " | ";
            }
            println!("{}", current_row);

            let mut row_separator = String::from(". ");
            for _ in 0..self.game.get_board_size() {
                row_separator += "_ . "
            }
            println!("{}", row_separator);
        }
    }

    fn get_cell_content(&self, row: usize, col: usize) -> &str {
        let field_type = &self.game.get_game_board()[row][col];

        match field_type {
            FieldType::CROSS => "X",
            FieldType::CIRCLE => "0",
            FieldType::UNOWNED => " "
        }
    }
}