use std::{
    rc::Rc,
    str::FromStr,
};

use chess::{Board, ChessMove, Color, Error, File, MoveGen, Rank, Square};
use slint::{ModelRc, SharedString, VecModel};

#[derive(Debug)]
#[allow(dead_code)]
pub enum PositionParseError {
    MisformedFen(String),
    PositionError(String),
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum MoveResult {
    Done,
    IsPromotion,
    Failure,
}

#[derive(Default, Clone)]
pub struct Game {
    position: Board,
    reversed: bool,
}

#[allow(dead_code)]
impl Game {
    pub fn new(input_fen: String) -> Result<Self, Error> {
        let position = Board::from_str(input_fen.as_str())?;
        Ok(Self {
            position,
            reversed: false,
        })
    }

    pub fn get_board(&self) -> Board {
        self.position
    }

    pub fn get_pieces(&self) -> ModelRc<ModelRc<SharedString>> {
        let mut result = Vec::<ModelRc<SharedString>>::new();

        for row in 0..8 {
            let mut line_result = Vec::new();
            for col in 0..8 {
                let current_piece = if self.reversed {
                    self.position.piece_on(Square::make_square(
                        Rank::from_index(row),
                        File::from_index(7 - col),
                    ))
                } else {
                    self.position.piece_on(Square::make_square(
                        Rank::from_index(7 - row),
                        File::from_index(col),
                    ))
                };
                match current_piece {
                    Some(piece) => {
                        let current_color = if self.reversed {
                            self.position.color_on(Square::make_square(
                                Rank::from_index(row),
                                File::from_index(7 - col),
                            ))
                        } else {
                            self.position.color_on(Square::make_square(
                                Rank::from_index(7 - row),
                                File::from_index(col),
                            ))
                        };
                        match current_color {
                            Some(color) => {
                                let new_string = SharedString::from(format!(
                                    "{}",
                                    piece.clone().to_string(color)
                                ));
                                line_result.push(new_string);
                            }
                            _ => {}
                        }
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
        self.position.side_to_move() == Color::White
    }

    pub fn try_making_move(
        &mut self,
        start_file: i32,
        start_rank: i32,
        end_file: i32,
        end_rank: i32,
    ) -> MoveResult {
        let legal_moves = MoveGen::new_legal(&self.position);
        let matching_moves: Vec<_> = legal_moves
            .into_iter()
            .filter(|current| {
                let from = current.get_source();
                let to = current.get_dest();
                return from.get_file().to_index() == start_file as usize
                    && from.get_rank().to_index() == start_rank as usize
                    && to.get_file().to_index() == end_file as usize
                    && to.get_rank().to_index() == end_rank as usize;
            })
            .collect();
        if !matching_moves.is_empty() {
            let move_to_process = matching_moves[0];
            if move_to_process.get_promotion().is_some() {
                MoveResult::IsPromotion
            } else {
                let mut result = Board::default();
                self.position.make_move(move_to_process, &mut result);
                self.position = result;
                MoveResult::Done
            }
        } else {
            MoveResult::Failure
        }
    }

    pub fn make_move(&mut self, move_to_do: ChessMove) {
        let mut result = Board::default();
        self.position.make_move(move_to_do, &mut result);
        self.position = result;
    }
}
