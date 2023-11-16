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

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    ui.global::<Fluent>().on_translate(move |input, args| {
        let args = args
            .iter()
            .map(|elt| (elt.name.to_string(), elt.value.to_string()))
            .collect::<Vec<_>>();
        tr(input.as_str(), args).into()
    });

    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });

    ui.run()
}
