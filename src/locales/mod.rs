// This file is part of "lingo"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

pub mod alphabet;
pub mod countries;
pub mod languages;

use std::str::FromStr;

use alphabet::Alphabet;
use countries::CountryCode;
use languages::{ Language, LanguageCode };

#[macro_export]
macro_rules! locale {
    ($a:expr) => {
        Locale::from_string($a, '_').unwrap()
    };
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Locale(pub Language, pub CountryCode);

impl Locale {
    pub fn from_string(locale: &str, separator: char) -> Result<Locale, String> {
        let mut parts = locale.split(separator);

        // Retrieves the language code
        let language_code = parts.next();
        
        // Locale without language code is impossible
        if language_code == None {
            return Err(format!("invalid locale '{locale}'"));
        }
        
        // Creates a `LanguageCode` object from its string code
        let language_code = LanguageCode::from_str(language_code.unwrap());
        match language_code {
            Ok(_) => {}
            Err(_) => return Err(format!("invalid language code in locale '{locale}'")),
        }

        // Now creates the language object for the locale
        let language = Language::new(language_code.clone().unwrap());

        // Retrieves the country code (or maybe the alphabet)
        let country_code_or_alphabet = parts.next();
        
        // Locale without alphabet and without country code
        if country_code_or_alphabet == None {
            return Ok(Locale(language, CountryCode::None));
        }

        // See "locales/alphabet.rs"
        let is_alphabet = match country_code_or_alphabet.unwrap() {
            "Latn" => true,
            "Cyrl" => true,
            "Hans" => true,
            "Hant" => true,
            _ => false,
        };
        
        // `Alphabet::Unspecified` when it's the country code
        let alphabet: Alphabet = if is_alphabet {
            let alphabet = Alphabet::from_code(country_code_or_alphabet.unwrap());
            match alphabet {
                Ok(_) => {}
                Err(_) => return Err(format!("invalid alphabet in locale '{locale}'")),
            }

            alphabet.unwrap()
        } else {
            Alphabet::Unspecified
        };

        // This language variable can be used both if it's the country code or 
        // the alphabet, because it can be a `Language` with an `Unspecified` 
        // alphabet, which would mean it's the country code and not the 
        // alphabet.
        let language = Language::with_alphabet(language_code.unwrap(), alphabet);

        let country_code = if is_alphabet {
            let country_code = parts.next();
            
            // Locale with language with alphabet but without country code
            if country_code == None {
                return Ok(Locale(language, CountryCode::None));
            }

            country_code
        } else {
            country_code_or_alphabet
        };
        
        // Creates a `CountryCode` object from its string code
        let country_code = CountryCode::from_str(country_code.unwrap());
        
        match country_code {
            Ok(_) => {}
            Err(_) => return Err(format!("invalid country code in locale '{locale}'")),
        }

        // Returns a locale with a language + a country code
        Ok(Locale(language, country_code.unwrap()))
    }

    pub fn to_code(&self) -> String {
        // Where `self.0` is the language and `self.1` is the country code

        if self.1 != CountryCode::None {
            format!("{}_{}", self.0.to_code(), self.1.to_code())
        } else {
            format!("{}", self.0.to_code())
        }
    }

    pub fn language(&self) -> &Language {
        &self.0
    }

    pub fn country_code(&self) -> &CountryCode {
        &self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alphabet::Alphabet;
    use languages::LanguageCode;

    #[test]
    pub fn locale_from_string() {
        assert_eq!(
            Locale::from_string("ar", '\0').unwrap(),
            Locale(Language::new(LanguageCode::ar), CountryCode::None)
        );

        assert_eq!(
            Locale::from_string("fr_FR", '_').unwrap(),
            Locale(Language::new(LanguageCode::fr), CountryCode::FR)
        );

        assert_eq!(
            Locale::from_string("en-US", '-').unwrap(),
            Locale(Language::new(LanguageCode::en), CountryCode::US)
        );

        assert_eq!(
            Locale::from_string("zh_Hans_HK", '_').unwrap(),
            Locale(Language::with_alphabet(LanguageCode::zh, Alphabet::Simplified), CountryCode::HK)
        );

        match Locale::from_string("xx_XX", '_') {
            Ok(_) => panic!("should not be ok, 'xx_XX' is not a valid locale"),
            Err(message) => println!("{message}"),
        }

        match Locale::from_string("fr_XX", '_') {
            Ok(_) => panic!("should not be ok, 'fr_XX' is not a valid locale"),
            Err(message) => println!("{message}"),
        }
    }

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
