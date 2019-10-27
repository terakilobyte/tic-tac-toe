use nannou::prelude::*;
mod board;
mod board_display;
mod brain;
mod eval;
mod field;
mod model;
mod player;
use board::Board;
pub use field::Field;
use model::{GameMode, Model, PlayerMode};

fn main() {
    nannou::app(model).view(view).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .with_dimensions(600, 600)
        .event(window_event)
        .build()
        .unwrap();
    Model {
        board: Board::new(app.window_rect(), PlayerMode::PlayUndecided),
        mode: GameMode::Waiting,
        player_mode: PlayerMode::PlayUndecided,
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    frame.clear(SKYBLUE);
    let draw = app.draw();
    model.display(&draw, &app.window_rect());
    draw.to_frame(app, &frame).unwrap();
}

fn window_event(app: &App, model: &mut Model, event: WindowEvent) {
    if model.mode == GameMode::SinglePlayer(PlayerMode::PlayX)
        || model.mode == GameMode::SinglePlayer(PlayerMode::PlayO)
    {
        if model.board.player_2 == model.board.current_player {
            model.board.computer_move();
        }
    }
    match event {
        MousePressed(_button) => match &model.mode {
            GameMode::Waiting => {
                model.mode = model.check_mode(&app.window_rect(), app.mouse.position());
            }
            GameMode::SinglePlayer(x) => match x {
                PlayerMode::PlayUndecided => {
                    model.mode = model
                        .check_player_mode(&app.window_rect(), app.mouse.position());
                    model.player_mode =
                        if model.mode == GameMode::SinglePlayer(PlayerMode::PlayX) {
                            PlayerMode::PlayX
                        } else {
                            PlayerMode::PlayO
                        };
                    model.board = Board::new(app.window_rect(), model.player_mode);
                }
                _ => {
                    model.check_new(app);
                }
            },
            _ => {
                model.check_new(app);
            }
        },
        Resized(_size) => {
            model.board = Board::new(app.window_rect(), PlayerMode::PlayUndecided);
        }

        _ => {}
    }
}
