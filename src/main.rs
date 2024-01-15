#![windows_subsystem = "windows"]
slint::include_modules!();

#[macro_use]
extern crate rust_i18n;

i18n!("i18n");

use std::{cell::RefCell, rc::Rc};

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
    ui.set_gameOver(false);
    let game = Rc::new(RefCell::new(game));
    let game_1 = Rc::clone(&game);
    let ui_handle = ui.as_weak();
    ui.on_moveSubmitted(move |start_file: i32, start_rank, end_file, end_rank| {
        let ui = ui_handle.unwrap();

        let result = game_1.borrow_mut().try_making_move(
            start_file as u8,
            start_rank as u8,
            end_file as u8,
            end_rank as u8,
        );
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
        let start_file_char = (('a' as u8) + pending_promotion.startFile as u8) as char;
        let start_rank_char = (('1' as u8) + pending_promotion.startRank as u8) as char;
        let end_file_char = (('a' as u8) + pending_promotion.endFile as u8) as char;
        let end_rank_char = (('1' as u8) + pending_promotion.endRank as u8) as char;
        let move_to_apply_uci = format!(
            "{}{}{}{}{}",
            start_file_char, start_rank_char, end_file_char, end_rank_char, promotion_type,
        );
        let promotion_result = game_2.borrow_mut().try_commiting_promotion(move_to_apply_uci);
        match promotion_result
        {
            MoveResult::Done => {
                ui.set_pieces(game_2.borrow().get_pieces());
                ui.set_whiteTurn(game_2.borrow().is_white_turn());
                let outcome = game_2.borrow().get_outcome();
            },
            _ => ()
        }
    });

    ui.run()
}
