use std::rc::Rc;

use owlchess::{board::FenParseError, Board, DrawReason, File, Move, Rank};
use slint::{ModelRc, SharedString, VecModel};

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum MoveResult {
    Done,
    IsPromotion,
    Failure,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)] // TODO: remove
pub enum Outcome {
    WhiteWon,
    BlackWon,
    Stalemate,
    ThreeFoldRepetitionDraw,
    InsufficientMaterialDraw,
    FiftyMovesRuleDraw,
}

#[derive(Clone)]
pub struct Game {
    position: Board,
    reversed: bool,
}

#[allow(dead_code)]
impl Game {
    pub fn default() -> Self {
        let board = Board::initial();
        Self {
            position: board,
            reversed: false,
        }
    }

    pub fn new(input_fen: String) -> Result<Self, FenParseError> {
        let board = Board::from_fen(&input_fen)?;
        Ok(Self {
            position: board,
            reversed: false,
        })
    }

    pub fn get_board(&self) -> &Board {
        &self.position
    }

    pub fn get_pieces(&self) -> ModelRc<ModelRc<SharedString>> {
        let mut result = Vec::<ModelRc<SharedString>>::new();

        for row in 0..8 {
            let mut line_result = Vec::new();
            for col in 0..8 {
                let current_piece = if self.reversed {
                    self.position
                        .get2(File::from_index(7 - col), Rank::from_index(7 - row))
                } else {
                    self.position
                        .get2(File::from_index(col), Rank::from_index(row))
                };
                let new_string = SharedString::from(format!("{}", current_piece));
                line_result.push(new_string);
            }
            let line_result = ModelRc::from(Rc::new(VecModel::from(line_result)));
            result.push(line_result);
        }

        ModelRc::from(Rc::new(VecModel::from(result)))
    }

    pub fn is_white_turn(&self) -> bool {
        let fen = self.position.as_fen();
        fen.split_ascii_whitespace().nth(1) == Some("w")
    }

    pub fn try_making_move(
        &mut self,
        start_file: u8,
        start_rank: u8,
        end_file: u8,
        end_rank: u8,
    ) -> MoveResult {
        let moved_piece = self
            .position
            .get2(
                File::from_index(start_file as usize),
                Rank::from_index(start_rank as usize),
            )
            .as_char();
        let is_promotion_move = (moved_piece == 'p' && start_rank == 1 && end_rank == 0)
            || (moved_piece == 'P' && start_rank == 6 && end_rank == 7);
        if is_promotion_move {
            MoveResult::IsPromotion
        } else {
            let start_file_char = (('a' as u8) + start_file) as char;
            let start_rank_char = (('1' as u8) + start_rank) as char;
            let end_file_char = (('a' as u8) + end_file) as char;
            let end_rank_char = (('1' as u8) + end_rank) as char;
            let move_to_apply_uci = format!(
                "{}{}{}{}",
                start_file_char, start_rank_char, end_file_char, end_rank_char
            );
            let move_to_apply = Move::from_uci_legal(move_to_apply_uci.as_str(), &self.position);
            match move_to_apply {
                Ok(move_to_apply) => match self.position.make_move(move_to_apply) {
                    Ok(board) => {
                        self.position = board;
                        MoveResult::Done
                    }
                    _ => MoveResult::Failure,
                },
                _ => MoveResult::Failure,
            }
        }
    }

    pub fn try_commiting_promotion(&mut self, move_uci: String) -> MoveResult {
        let move_to_apply = Move::from_uci_legal(move_uci.as_str(), &self.position);
        match move_to_apply {
            Ok(move_to_apply) => match self.position.make_move(move_to_apply) {
                Ok(board) => {
                    self.position = board;
                    MoveResult::Done
                }
                _ => MoveResult::Failure,
            },
            _ => MoveResult::Failure,
        }
    }

    pub fn get_outcome(&self) -> Option<Outcome> {
        match self.position.calc_outcome() {
            Some(owlchess::Outcome::Win {
                side: owlchess::Color::White,
                ..
            }) => Some(Outcome::WhiteWon),
            Some(owlchess::Outcome::Win {
                side: owlchess::Color::Black,
                ..
            }) => Some(Outcome::BlackWon),
            Some(owlchess::Outcome::Draw(DrawReason::Stalemate)) => Some(Outcome::Stalemate),
            Some(owlchess::Outcome::Draw(DrawReason::InsufficientMaterial)) => {
                Some(Outcome::InsufficientMaterialDraw)
            }
            Some(owlchess::Outcome::Draw(DrawReason::Moves50)) => Some(Outcome::FiftyMovesRuleDraw),
            Some(owlchess::Outcome::Draw(DrawReason::Repeat3)) => {
                Some(Outcome::ThreeFoldRepetitionDraw)
            }
            _ => None,
        }
    }
}
