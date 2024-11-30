use crate::game::Game;
use iced::alignment::Horizontal;
use iced::widget::{button, column, container, image, row, text};
use iced::window::Settings;
use iced::{Alignment, Background, Color, ContentFit, Element, Size, Task, Theme};

mod game;
mod game_controller;

pub fn main() -> iced::Result {
    let game = Game::new();
    iced::application("Tic Tac Toe", update, view)
        .theme(|_| Theme::Light)
        .centered()
        .window(Settings {
            size: Size::new(1920.0, 1080.0),
            ..Default::default()
        })
        .run_with(move || {
            (MyState::new(game), Task::none())
        })
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed { row: usize, column: usize },
}

struct MyState {
    img_srcs: Vec<Vec<String>>,
    cell_size: f32,
    game: Game,
    is_finished_game: bool
}

impl MyState {
    fn new(game: Game) -> Self {
        let img_src = vec![
            vec!["".to_string(); game.get_board_size()];
            game.get_board_size()
        ];

        MyState {
            img_srcs: img_src,
            cell_size: 150.0,
            game,
            is_finished_game: false,
        }
    }
}


fn update(state: &mut MyState, message: Message) {
    if state.is_finished_game {
        return;
    }
    match message {
        Message::ButtonPressed { row, column } => {
            if !state.game.is_cell_occupied(row, column) {
                let current_player = state.game.get_current_player();
                let is_blue = *current_player == game::Color::BLUE;

                state.game.capture_cell(row, column);
                if is_blue {
                    state.img_srcs[row][column] = "assets/cross.png".to_string();
                } else {
                    state.img_srcs[row][column] = "assets/circle.png".to_string();
                }
                state.game.change_current_player();
                if state.game.is_game_finished() {
                    state.is_finished_game = true;
                }
            }
        }
    }
}


fn view(state: &MyState) -> Element<Message> {
    let mut col = column![];

    for row_index in 0..state.game.get_board_size() {
        let mut buttons_row = row![];

        for col_index in 0..state.game.get_board_size() {
            let button = button(
                container(
                    image(state.img_srcs[row_index][col_index].clone())
                        .content_fit(ContentFit::Fill)
                        .width(iced::Length::Fixed(state.cell_size))
                        .height(iced::Length::Fixed(state.cell_size)),
                )
                    .style(|_: &Theme| {
                        container::Style {
                            background: Some(Background::Color(Color::from_rgb(1.0, 1.0, 1.0))),
                            ..Default::default()
                        }
                    }),
            )
                .on_press(Message::ButtonPressed {
                    row: row_index,
                    column: col_index,
                });

            buttons_row = buttons_row.push(button);
        }
        col = col.push(buttons_row.spacing(5.0));
    }
    if state.is_finished_game {
        let button = button(
            text("Game Over")
                .size(20)
                .color(Color::BLACK)
                .width(200.0)
                .height(50.0)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center));

        col = col.push(button);
    }
    col
        .width(1920.0)
        .align_x(Horizontal::Center)
        .spacing(5.0)
        .into()
}
