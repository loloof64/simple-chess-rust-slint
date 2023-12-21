#![windows_subsystem = "windows"]
slint::include_modules!();

#[macro_use]
extern crate rust_i18n;

i18n!("i18n");

use std::{rc::Rc, cell::RefCell};

use chess::MoveGen;
use slint::Model;

extern crate current_locale;
use current_locale::current_locale;

fn tr(key: &str, args: Vec<(String, String)>) -> String {
    let locale = current_locale();
    let locale = match locale {
        Ok(l) => String::from(&l[..2]),
        _ => String::from("en"),
    };
    let locale = if ["en", "es", "fr"]
        .map(|elt| String::from(elt))
        .contains(&locale)
    {
        locale
    } else {
        String::from("en")
    };

    let mut translation = _rust_i18n_translate(locale.as_str(), key);
    for pair in args.iter() {
        translation = translation.replace(format!("%{{{}}}", pair.0).as_str(), pair.1.as_str());
    }
    translation
}

mod game;
use game::{Game, MoveResult};

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    ui.global::<Translator>().on_translate(move |input, args| {
        let args = args
            .iter()
            .map(|elt| (elt.name.to_string(), elt.value.to_string()))
            .collect::<Vec<_>>();
        tr(input.as_str(), args).into()
    });

    let game = Game::default();
    ui.set_pieces(game.get_pieces());
    ui.set_whiteTurn(game.is_white_turn());
    let game = Rc::new(RefCell::new(game));
    let game_1 = Rc::clone(&game);
    let ui_handle = ui.as_weak();
    ui.on_moveSubmitted(move |start_file, start_rank, end_file, end_rank| {
        let ui = ui_handle.unwrap();

        let result = game_1.borrow_mut().try_making_move(start_file, start_rank, end_file, end_rank);
        match result {
            MoveResult::Done => {
                ui.set_pieces(game_1.borrow().get_pieces());
                ui.set_whiteTurn(game_1.borrow().is_white_turn());
            }
            MoveResult::IsPromotion => {
                ui.set_pendingPromotion(PendingPromotion {
                    isActive: true,
                    startFile: start_file,
                    startRank: start_rank,
                    endFile: end_file,
                    endRank: end_rank,
                });
            }
            _ => {}
        }
    });
    let ui_handle = ui.as_weak();
    let game_2 = Rc::clone(&game);
    ui.on_promotionValidated(move |promotion_type| {
        let ui = ui_handle.unwrap();
        let pending_promotion = ui.get_pendingPromotion();
        let legal_moves = MoveGen::new_legal(&game_2.borrow().get_board());
        let matching_moves: Vec<_> = legal_moves
            .into_iter()
            .filter(|current| {
                let from = current.get_source();
                let to = current.get_dest();
                return from.get_file().to_index() == pending_promotion.startFile as usize
                    && from.get_rank().to_index() == pending_promotion.startRank as usize
                    && to.get_file().to_index() == pending_promotion.endFile as usize
                    && to.get_rank().to_index() == pending_promotion.endRank as usize
                    && current.get_promotion().is_some()
                    && current
                        .get_promotion()
                        .unwrap()
                        .to_string(chess::Color::Black)
                        .as_str()
                        == promotion_type.as_str();
            })
            .collect();
        if !matching_moves.is_empty() {
            let move_to_process = matching_moves[0];
            game_2.borrow_mut().make_move(move_to_process);
            ui.set_pieces(game_2.borrow().get_pieces());
            ui.set_whiteTurn(game_2.borrow().is_white_turn());
        }
    });

    ui.run()
}
