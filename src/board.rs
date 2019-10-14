use crate::eval::Eval;
use crate::player::Player;
use crate::Field;
use nannou::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BoardState {
    Winner(Player, (usize, usize, usize)),
    Tie,
    InGame,
}

#[derive(Debug)]
pub struct Board {
    rect: geom::Rect,
    board: Vec<Field>,
    player_1: Player,
    player_2: Player,
    pub current_player: Player,
    pub state: BoardState,
}

impl Board {
    pub fn new(rect: geom::Rect) -> Self {
        Board {
            rect,
            board: (0..9).map(|_| Field::Empty).collect(),
            player_1: Player::Human,
            player_2: Player::Computer,
            current_player: Player::Human,
            state: BoardState::InGame,
        }
    }
    pub fn computer_move(&mut self) {
        if self.state == BoardState::InGame {
            if self.current_player == Player::Computer {
                let eval =
                    Self::minimax(self.state, &self.board, self.current_player, 0);
                self.board[eval.position] = Field::O;
                self.made_move();
            }
        }
    }

    fn check_winner(board: &Vec<Field>) -> BoardState {
        let winning_boards = vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
            vec![0, 3, 6],
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![0, 4, 8],
            vec![2, 4, 6],
        ];
        let mut winning = None;
        winning_boards.iter().any(|ts| {
            if board[ts[0]] == board[ts[1]] && board[ts[1]] == board[ts[2]] {
                if board[ts[0]] == Field::X {
                    winning =
                        Some(BoardState::Winner(Player::Human, (ts[0], ts[1], ts[2])));
                } else if board[ts[0]] == Field::O {
                    winning = Some(BoardState::Winner(
                        Player::Computer,
                        (ts[0], ts[1], ts[2]),
                    ));
                } else if board[ts[0]] == Field::Empty {
                    return false;
                }
                return true;
            }
            return false;
        });
        if !board.contains(&Field::Empty) {
            winning = Some(BoardState::Tie);
        }
        winning.unwrap_or_else(|| BoardState::InGame)
    }

    fn made_move(&mut self) {
        self.state = Self::check_winner(&self.board);
        self.current_player = -self.current_player;
    }

    fn left_column(&self, mouse_x: f32) -> bool {
        mouse_x < self.rect.left() / 3.0
    }
    fn right_column(&self, mouse_x: f32) -> bool {
        mouse_x > self.rect.right() / 3.0
    }
    fn center_column(&self, mouse_x: f32) -> bool {
        !self.left_column(mouse_x) && !self.right_column(mouse_x)
    }
    fn top_row(&self, mouse_y: f32) -> bool {
        mouse_y > self.rect.top() / 3.0
    }
    fn bottom_row(&self, mouse_y: f32) -> bool {
        mouse_y < self.rect.bottom() / 3.0
    }
    fn center_row(&self, mouse_y: f32) -> bool {
        !self.top_row(mouse_y) && !self.bottom_row(mouse_y)
    }
    pub fn register_click(&mut self, app: &App) {
        match self.state {
            BoardState::InGame => {
                let (x, y) = (app.mouse.position().x, app.mouse.position().y);
                let location = match (x, y) {
                    (x, y) if self.left_column(x) && self.top_row(y) => 0,
                    (x, y) if self.center_column(x) && self.top_row(y) => 1,
                    (x, y) if self.right_column(x) && self.top_row(y) => 2,

                    (x, y) if self.left_column(x) && self.center_row(y) => 3,
                    (x, y) if self.center_column(x) && self.center_row(y) => 4,
                    (x, y) if self.right_column(x) && self.center_row(y) => 5,

                    (x, y) if self.left_column(x) && self.bottom_row(y) => 6,
                    (x, y) if self.center_column(x) && self.bottom_row(y) => 7,
                    (x, y) if self.right_column(x) && self.bottom_row(y) => 8,

                    _ => unreachable!(),
                };
                if self.board[location] != Field::Empty {
                    return;
                }
                match self.current_player {
                    Player::Human => self.board[location] = Field::X,
                    Player::Computer => self.board[location] = Field::O,
                }
                self.made_move();
            }
            _ => {
                let new_self = Self::new(self.rect);
                std::mem::replace(self, new_self);
                return;
            }
        };
    }

    pub fn show_selections(&self, draw: &app::Draw, rect: &Rect) {
        let dims = (rect.right() - rect.left()) / 3.0;
        let draw_text = |sigil: &str, location: &Rect| {
            let text = text(sigil).font_size(dims as u32).build(*location);
            draw.path().fill().color(BLACK).events(text.path_events());
        };
        self.board.iter().enumerate().for_each(|(i, v)| {
            if *v != Field::Empty {
                match (i, v) {
                    (0, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.left() + dims / 2.0,
                            rect.top() - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    (1, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.right() / 3.0 - dims / 2.0,
                            rect.top() - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    (2, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.right() - dims / 2.0,
                            rect.top() - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    (3, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.left() + dims / 2.0,
                            rect.top() / 3.0 - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    (4, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.left() / 3.0 + dims / 2.0,
                            rect.top() / 3.0 - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    (5, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.right() - dims / 2.0,
                            rect.top() / 3.0 - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    (6, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.left() + dims / 2.0,
                            rect.bottom() / 3.0 - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    (7, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.right() / 3.0 - dims / 2.0,
                            rect.bottom() / 3.0 - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    (8, v) => {
                        let location = Rect::from_x_y_w_h(
                            rect.right() - dims / 2.0,
                            rect.bottom() / 3.0 - dims / 3.0,
                            dims,
                            dims,
                        );
                        draw_text(&v.to_string(), &location);
                    }
                    _ => unimplemented!(),
                };
            }
        });
    }
    fn show_winner(&self, draw: &app::Draw, rect: &Rect, win: (usize, usize, usize)) {
        let start_x;
        let start_y;
        let end_x;
        let end_y;
        let (top, _, bottom) = win;
        match (top, bottom) {
            // top row
            (0, 2) => {
                start_x = rect.left();
                start_y = rect.top() - rect.top() / 3.0;
                end_x = rect.right();
                end_y = rect.top() - rect.top() / 3.0;
            }
            // left column
            (0, 6) => {
                start_x = rect.left() - rect.left() / 3.0;
                start_y = rect.top();
                end_x = rect.left() - rect.left() / 3.0;
                end_y = rect.bottom();
            }
            // middle row
            (3, 5) => {
                start_x = rect.left();
                start_y = 0.0;
                end_x = rect.right();
                end_y = 0.0;
            }
            // middle column
            (1, 7) => {
                start_x = 0.0;
                start_y = rect.top();
                end_x = 0.0;
                end_y = rect.bottom();
            }
            // bottom row
            (6, 8) => {
                start_x = rect.left();
                start_y = rect.bottom() - rect.bottom() / 3.0;
                end_x = rect.right();
                end_y = rect.bottom() - rect.bottom() / 3.0;
            }
            // right column
            (2, 8) => {
                start_x = rect.right() - rect.right() / 3.0;
                start_y = rect.top();
                end_x = rect.right() - rect.right() / 3.0;
                end_y = rect.bottom();
            }
            // left diag
            (0, 8) => {
                start_x = rect.left();
                start_y = rect.top();
                end_x = rect.right();
                end_y = rect.bottom();
            }
            // right diag
            (2, 6) => {
                start_x = rect.right();
                start_y = rect.top();
                end_x = rect.left();
                end_y = rect.bottom();
            }
            _ => unreachable!(),
        }
        draw.line()
            .start(pt2(start_x, start_y))
            .end(pt2(end_x, end_y))
            .stroke_weight(2.0)
            .color(BLACK);
    }
    pub fn display(&self, draw: &app::Draw, rect: &Rect) {
        match &self.state {
            BoardState::Tie => {
                self.show_selections(draw, rect);
                let location = rect.pad(20.0);
                let wins = format!("Tie!");
                let text = text(&wins).font_size(75).build(location);
                draw.path().fill().color(WHITE).events(text.path_events());
            }
            BoardState::InGame => {
                self.show_selections(draw, rect);
            }
            winner => {
                self.show_selections(draw, rect);
                let (winning_player, winning_pos) = match winner {
                    BoardState::Winner(Player::Human, x) => (Field::X, x),
                    BoardState::Winner(Player::Computer, x) => (Field::O, x),
                    _ => unreachable!(),
                };
                self.show_winner(draw, rect, *winning_pos);
                let location = rect.pad(20.0);
                let wins = format!("{} Wins!", &winning_player.to_string());
                let text = text(&wins).font_size(75).build(location);
                draw.path().fill().color(WHITE).events(text.path_events());
            }
        }
    }
    fn minimax(
        state: BoardState,
        board: &Vec<Field>,
        player: Player,
        depth: i64,
    ) -> Eval {
        return match state {
            BoardState::Tie => Eval {
                position: 0,
                score: 0,
            },
            BoardState::InGame => {
                let evaluated_moves: Vec<Eval> = board
                    .iter()
                    .enumerate()
                    .filter_map(|(i, v)| match v {
                        Field::Empty => {
                            let mut cloned_board = board.clone();
                            let new_field = match player {
                                Player::Human => Field::X,
                                Player::Computer => Field::O,
                            };
                            cloned_board[i] = new_field;
                            let score = Board::minimax(
                                Self::check_winner(&cloned_board),
                                &cloned_board,
                                -player,
                                depth + 1,
                            )
                            .score;
                            Some(Eval::new(i, score))
                        }
                        _ => None,
                    })
                    .collect();
                let mut cloned_evals = evaluated_moves.clone();
                cloned_evals.sort();
                match player {
                    Player::Human => {
                        let last = cloned_evals.last();
                        *last.unwrap()
                    }
                    Player::Computer => {
                        let first = cloned_evals.first();
                        *first.unwrap()
                    }
                }
            }
            winner => match winner {
                BoardState::Winner(Player::Human, _) => Eval {
                    position: 0,
                    score: depth - 10,
                },
                BoardState::Winner(Player::Computer, _x) => Eval {
                    position: 0,
                    score: 10 - depth,
                },
                _ => unreachable!(),
            },
        };
    }
}
