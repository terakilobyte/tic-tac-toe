use crate::board::BoardState;
use crate::eval::Eval;
use crate::field::Field;
use crate::player::Player;

pub fn minimax(state: BoardState, board: &[Field], player: Player, depth: i64) -> Eval {
    match state {
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
                        let mut cloned_board = Vec::from(board).clone();
                        let new_field = match player {
                            Player::Human => Field::X,
                            Player::Computer => Field::O,
                        };
                        cloned_board[i] = new_field;
                        let score = minimax(
                            check_winner(&cloned_board),
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
    }
}
pub fn check_winner(board: &[Field]) -> BoardState {
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
                winning = Some(BoardState::Winner(Player::Human, (ts[0], ts[1], ts[2])));
                return true;
            } else if board[ts[0]] == Field::O {
                winning =
                    Some(BoardState::Winner(Player::Computer, (ts[0], ts[1], ts[2])));
                return true;
            }
        }
        false
    });

    if !board.contains(&Field::Empty) && winning.is_none() {
        winning = Some(BoardState::Tie);
    }
    winning.unwrap_or_else(|| BoardState::InGame)
}
