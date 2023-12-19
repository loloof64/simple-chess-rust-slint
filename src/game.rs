use std::rc::Rc;

use shakmaty::{
    fen::Fen,
    CastlingMode, Chess, Color, Position, Square,
};
use slint::{ModelRc, SharedString, VecModel};


#[derive(Debug)]
pub enum PositionParseError {
    MisformedFen(String),
    PositionError(String),
}

#[derive(Default)]
pub struct Game {
    position: Chess,
    reversed: bool,
}

impl Game {
    pub fn new(input_fen: String) -> Result<Self, PositionParseError> {
        let fen: Result<Fen, _> = input_fen.as_str().parse();
        match fen {
            Ok(fen) => {
                let position = fen.into_position(CastlingMode::Standard);
                match position {
                    Ok(position) => Ok(Self {
                        position,
                        reversed: false,
                    }),
                    _ => Err(PositionParseError::MisformedFen(input_fen)),
                }
            }
            _ => Err(PositionParseError::PositionError(input_fen)),
        }
    }

    pub fn get_pieces(&self) -> ModelRc<ModelRc<SharedString>> {
        let mut result = Vec::<ModelRc<SharedString>>::new();
        let pieces = self.position.board();

        for row in 0..8 {
            let mut line_result = Vec::new();
            for col in 0..8 {
                let current_piece = if self.reversed {
                    pieces.piece_at(Square::new((8 * row + (7 - col)) as u32))
                } else {
                    pieces.piece_at(Square::new((col + (7 - row) * 8) as u32))
                };
                match current_piece {
                    Some(piece) => {
                        let new_string = SharedString::from(format!("{}", piece.clone().char()));
                        line_result.push(new_string);
                    }
                    _ => line_result.push(SharedString::from("")),
                }
            }
            let line_result = ModelRc::from(Rc::new(VecModel::from(line_result)));
            result.push(line_result);
        }

        ModelRc::from(Rc::new(VecModel::from(result)))
    }

    pub fn is_white_turn(&self) -> bool {
        self.position.turn() == Color::White
    }
}
