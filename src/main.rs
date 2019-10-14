use nannou::prelude::*;
mod board;
mod eval;
mod field;
mod player;
use board::{Board, BoardState};
pub use field::Field;

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

#[derive(Debug, PartialEq, Eq)]
enum GameMode {
    SinglePlayer,
    MultiPlayer,
    Waiting,
}

#[derive(Debug)]
struct Model {
    board: Board,
    mode: GameMode,
}

impl Model {
    fn check_mode(&self, rect: &Rect, mouse: Point2) -> GameMode {
        let x_single_player = rect.left() / 3.0;
        let y_single_player = 0.0;
        let x_multi_player = rect.right() / 3.0;
        let y_multi_player = 0.0;
        let width = 150.0;
        let height = width / 1.618;
        let single_player =
            Rect::from_x_y_w_h(x_single_player, y_single_player, width, height);
        let multi_player =
            Rect::from_x_y_w_h(x_multi_player, y_multi_player, width, height);

        if single_player.contains(mouse) {
            return GameMode::SinglePlayer;
        } else if multi_player.contains(mouse) {
            return GameMode::MultiPlayer;
        };
        GameMode::Waiting
    }
    fn display(&self, draw: &app::Draw, rect: &Rect) {
        match self.mode {
            GameMode::Waiting => {
                let x_single_player = rect.left() / 3.0;
                let y_single_player = 0.0;
                let x_multi_player = rect.right() / 3.0;
                let y_multi_player = 0.0;
                let width = 150.0;
                let height = width / 1.618;

                draw.rect()
                    .x_y(x_single_player, y_single_player)
                    .w_h(width, height)
                    .color(DARKGREY);
                draw.rect()
                    .x_y(x_multi_player, y_multi_player)
                    .w_h(width, height)
                    .color(DARKGREY);
                let single_player =
                    Rect::from_x_y_w_h(x_single_player, y_single_player, width, height);
                let multi_player =
                    Rect::from_x_y_w_h(x_multi_player, y_multi_player, width, height);

                let spt = text("Single Player").font_size(20).build(single_player);
                draw.path().fill().color(BLACK).events(spt.path_events());
                let mpt = text("Mutliplayer").font_size(20).build(multi_player);
                draw.path().fill().color(BLACK).events(mpt.path_events());
            }
            _ => {
                // right vertical line
                draw.line()
                    .start(pt2(rect.right() / 3.0, rect.top()))
                    .end(pt2(rect.right() / 3.0, rect.bottom()))
                    .stroke_weight(2.0)
                    .color(WHITE);
                // left vertical line
                draw.line()
                    .start(pt2(rect.left() / 3.0, rect.top()))
                    .end(pt2(rect.left() / 3.0, rect.bottom()))
                    .stroke_weight(2.0)
                    .color(WHITE);
                // bottom horizontal line
                draw.line()
                    .start(pt2(rect.left(), rect.bottom() / 3.0))
                    .end(pt2(rect.right(), rect.bottom() / 3.0))
                    .stroke_weight(2.0)
                    .color(WHITE);
                // top horizontal line
                draw.line()
                    .start(pt2(rect.left(), rect.top() / 3.0))
                    .end(pt2(rect.right(), rect.top() / 3.0))
                    .stroke_weight(2.0)
                    .color(WHITE);
                self.board.display(draw, &rect);
            }
        }
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .with_dimensions(600, 600)
        .event(window_event)
        .raw_event(raw_window_event)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .mouse_moved(mouse_moved)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .mouse_wheel(mouse_wheel)
        .mouse_entered(mouse_entered)
        .mouse_exited(mouse_exited)
        .touch(touch)
        .touchpad_pressure(touchpad_pressure)
        .moved(window_moved)
        .resized(window_resized)
        .hovered_file(hovered_file)
        .hovered_file_cancelled(hovered_file_cancelled)
        .dropped_file(dropped_file)
        .focused(window_focused)
        .unfocused(window_unfocused)
        .closed(window_closed)
        .build()
        .unwrap();
    Model {
        board: Board::new(app.window_rect()),
        mode: GameMode::Waiting,
    }
}

fn event(_app: &App, _model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            raw: _,
            simple: _,
        } => {}
        Event::DeviceEvent(_device_id, _event) => {}
        Event::Update(_dt) => {}
        Event::Awakened => {}
        Event::Suspended(_b) => {}
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: &Frame) {
    frame.clear(SKYBLUE);
    let draw = app.draw();
    model.display(&draw, &app.window_rect());
    draw.to_frame(app, &frame).unwrap();
}

fn check_new(app: &App, model: &mut Model) {
    if model.board.state != BoardState::InGame {
        model.mode = GameMode::Waiting;
        model.board = Board::new(app.window_rect());
    } else {
        model.board.register_click(&app);
    }
}

fn window_event(app: &App, model: &mut Model, event: WindowEvent) {
    if model.mode == GameMode::SinglePlayer {
        match model.board.current_player {
            player::Player::Computer => {
                model.board.computer_move();
            }
            _ => {}
        };
    }
    match event {
        KeyPressed(_key) => {}
        KeyReleased(_key) => {}
        MouseMoved(_pos) => {}
        MousePressed(_button) => match model.mode {
            GameMode::Waiting => {
                model.mode = model.check_mode(&app.window_rect(), app.mouse.position());
            }
            _ => {
                check_new(app, model);
            }
        },
        MouseReleased(_button) => {}
        MouseEntered => {}
        MouseExited => {}
        MouseWheel(_amount, _phase) => {}
        Moved(_pos) => {}
        Resized(_size) => {
            model.board = Board::new(app.window_rect());
        }
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}

fn raw_window_event(_app: &App, _model: &mut Model, _event: nannou::winit::WindowEvent) {
}

fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {}

fn key_released(_app: &App, _model: &mut Model, _key: Key) {}

fn mouse_moved(_app: &App, _model: &mut Model, _pos: Point2) {}

fn mouse_pressed(_app: &App, _model: &mut Model, _button: MouseButton) {}

fn mouse_released(_app: &App, _model: &mut Model, _button: MouseButton) {}

fn mouse_wheel(
    _app: &App,
    _model: &mut Model,
    _dt: MouseScrollDelta,
    _phase: TouchPhase,
) {
}

fn mouse_entered(_app: &App, _model: &mut Model) {}

fn mouse_exited(_app: &App, _model: &mut Model) {}

fn touch(_app: &App, _model: &mut Model, _touch: TouchEvent) {}

fn touchpad_pressure(_app: &App, _model: &mut Model, _pressure: TouchpadPressure) {}

fn window_moved(_app: &App, _model: &mut Model, _pos: Point2) {}

fn window_resized(_app: &App, _model: &mut Model, _dim: Vector2) {}

fn window_focused(_app: &App, _model: &mut Model) {}

fn window_unfocused(_app: &App, _model: &mut Model) {}

fn window_closed(_app: &App, _model: &mut Model) {}

fn hovered_file(_app: &App, _model: &mut Model, _path: std::path::PathBuf) {}

fn hovered_file_cancelled(_app: &App, _model: &mut Model) {}

fn dropped_file(_app: &App, _model: &mut Model, _path: std::path::PathBuf) {}
