// This file is part of "lingo"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

pub mod locales;
pub mod strings;

use locales::Locale;

pub fn hello() {
    println!("hello from the lingo library !");
}

pub fn translate(string: &str, to_locale: Locale) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::locales::{
        countries::CountryCode,
        languages::{Language, LanguageCode},
    };

    #[test]
    fn translate() {
        println!(
            "'{}' (en) -> '{}' (fr_FR)",
            "hello",
            crate::translate(
                "hello",
                Locale(Language::new(LanguageCode::fr), CountryCode::FR)
            )
        );
    }
}
