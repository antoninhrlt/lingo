// This file is part of "lingo"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

pub mod alphabet;
pub mod countries;
pub mod languages;

use countries::CountryCode;
use languages::Language;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Locale(pub Language, pub CountryCode);

impl Locale {
    pub fn from_string(locale: &str) -> Locale {
        todo!()
    }

    pub fn to_code(&self) -> String {
        // Where `self.0` is the language and `self.1` is the country code

        if self.1 != CountryCode::None {
            format!("{}_{}", self.0.to_code(), self.1.to_code())
        } else {
            format!("{}", self.0.to_code())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alphabet::Alphabet;
    use languages::LanguageCode;

    #[test]
    pub fn locales() {
        println!(
            "{} = {}",
            Locale(Language::new(LanguageCode::ar), CountryCode::None).to_code(),
            LanguageCode::ar.to_full_name()
        );

        println!(
            "{}",
            Locale(Language::new(LanguageCode::ar), CountryCode::DZ).to_code()
        );

        println!(
            "{}",
            Locale(
                Language::with_alphabet(LanguageCode::az, Alphabet::Cyrillic),
                CountryCode::AZ
            )
            .to_code()
        );
    }
}
