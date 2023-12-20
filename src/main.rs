#![windows_subsystem = "windows"]
slint::include_modules!();

#[macro_use]
extern crate rust_i18n;

i18n!("i18n");

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
use game::Game;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    ui.global::<Translator>().on_translate(move |input, args| {
        let args = args
            .iter()
            .map(|elt| (elt.name.to_string(), elt.value.to_string()))
            .collect::<Vec<_>>();
        tr(input.as_str(), args).into()
    });
    
    let mut game = Game::default();
    ui.set_pieces(game.get_pieces());
    ui.set_whiteTurn(game.is_white_turn());
    let ui_handle = ui.as_weak();
    ui.on_moveSubmitted(move |start_file, start_rank, end_file, end_rank| {
        let ui = ui_handle.unwrap();

        game.try_making_move(start_file, start_rank, end_file, end_rank);
        ui.set_pieces(game.get_pieces());
        ui.set_whiteTurn(game.is_white_turn());
    });

    ui.run()
}
