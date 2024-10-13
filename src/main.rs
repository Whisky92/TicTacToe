use crate::game::Game;
use crate::game_controller::GameController;

mod game;
mod game_controller;

fn main() {
    let game = Game::new();
    let mut game_controller = GameController::new(game);
    game_controller.handle_game_flow();
}
