// This file is part of "lingo"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::collections::HashMap;

use crate::locales::Locale;

pub type LingoStrings<'a> = HashMap<&'a str, HashMap<Locale, &'a str>>;

pub fn get_string(strings: LingoStrings, id: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::locales::{
        languages::{ Language, LanguageCode },
        countries::CountryCode,
    };

    use super::*;

    #[test]
    fn app() {
        let fr = Locale(Language::new(LanguageCode::fr), CountryCode::None);
        #[allow(non_snake_case)]
        let en_GB = Locale(Language::new(LanguageCode::en), CountryCode::GB);
        let de = Locale(Language::new(LanguageCode::de), CountryCode::None);

        let lingo_strings: LingoStrings = HashMap::from([
            ("hello", HashMap::from([
                (fr, "salut"),
                (en_GB, "hello"),
                (de, "hallo"),
            ]))
        ]);

        println!("{}", get_string(lingo_strings, "hello"));
    } 
}
