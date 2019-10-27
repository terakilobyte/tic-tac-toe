use crate::field::Field;
use crate::model::PlayerMode;
use std::convert::From;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Player {
    Player1 = 1,
    Player2 = -1,
}

impl Default for Player {
    fn default() -> Self {
        Player::Player1
    }
}

impl From<PlayerMode> for Player {
    fn from(mode: PlayerMode) -> Self {
        match mode {
            PlayerMode::PlayX => Player::Player1,
            PlayerMode::PlayO => Player::Player2,
            PlayerMode::PlayUndecided => Player::Player2,
        }
    }
}

impl Player {
    pub fn get_sigil(&self) -> Field {
        match self {
            Player::Player1 => Field::X,
            Player::Player2 => Field::O,
        }
    }
}

impl std::ops::Neg for Player {
    type Output = Player;

    fn neg(self) -> Self::Output {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }
}
