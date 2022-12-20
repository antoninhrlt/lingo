// This file is part of "lingo"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::collections::HashMap;

use crate::Lingo;

use crate::locales::Locale;

pub type InternationalString = HashMap<Locale, &'static str>;
pub type LingoStrings = HashMap<&'static str, InternationalString>;

/// Returns the string in the requested locale.
/// 
/// When there is no string for that locale, tries to return a string for the 
/// same language, no matter the country code.
pub fn get_localised_string(string: &InternationalString, locale: &Locale) -> Option<String> {
    match string.get(&locale) /* Option<&&str> */ {
        // A string was found for this exact locale (same language and same 
        // country code).
        Some(x) => return Some(x.to_string()),
        None => {}
    }

    // The localised string was not found for the requested locale.

    // Perhaps a string exists for the same language, no matter the country
    // code.
    for (locale, string) in string.iter() {
        // Does not look at the country code, just compare the language.
        if locale.language() == locale.language() {
            return Some(string.to_string());
        }
    }

    // No string for the requested language.
    None
}

impl Lingo {
    /// Returns an `InternationalString` object from a `Lingo` object.
    /// 
    /// Note that it is wrapped into an `Option<T>` since the string could not 
    /// be found for the given id.
    pub fn get_international_string(&self, id: &str) -> Option<InternationalString> {
        let international_string: Option<&InternationalString> = self.strings().get(id);
        match international_string {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }

    /// Retrieves a string from the lingo's strings.
    
    /// Returns the localised string for the context locale. If it doesn't 
    /// exist, returns the string for the default locale.
    pub fn string(&self, id: &str) -> Option<String> {
        let international_string: Option<InternationalString> = self.get_international_string(id);

        if international_string == None {
            return None;
        }

        let international_string = international_string.unwrap();

        // Tries to get the string in the context locale
        let localised_string: Option<String> =
            get_localised_string(&international_string, self.context_locale());

        if localised_string != None {
            return localised_string;
        }

        // The string is not available in the context locale

        // Tries to get the string in the default locale
        get_localised_string(&international_string, &self.default_locale())
    }
}

#[cfg(test)]
mod tests {
    use crate::locales::{
        countries::CountryCode,
        languages::{Language, LanguageCode},
    };

    use super::*;

    #[test]
    fn app() {
        let ar = Locale(Language::new(LanguageCode::ar), CountryCode::None);
        let fr = Locale(Language::new(LanguageCode::fr), CountryCode::None);
        let en = Locale(Language::new(LanguageCode::en), CountryCode::None);
        #[allow(non_snake_case)]
        let en_GB = Locale(Language::new(LanguageCode::en), CountryCode::GB);
        let de = Locale(Language::new(LanguageCode::de), CountryCode::None);

        let lingo_strings: LingoStrings = HashMap::from([(
            "hello_world",
            HashMap::from([
                (fr, "Bonjour le monde !"),
                (en_GB, "Hello world!"),
                (de, "Hallo Welt!"),
            ]),
        )]);

        let lingo = Lingo::new(ar, en, lingo_strings);

        // Well, there is no Arabic string for "hello_world".
        // The default locale is "en", so the "en_GB" string will be chosen.
        println!("{}", lingo.string("hello_world").unwrap());
    }

    #[test]
    fn from_system_locale() {
        let fr = Locale(Language::new(LanguageCode::fr), CountryCode::None);
        let en = Locale(Language::new(LanguageCode::en), CountryCode::None);
        #[allow(non_snake_case)]
        let en_GB = Locale(Language::new(LanguageCode::en), CountryCode::GB);
        let de = Locale(Language::new(LanguageCode::de), CountryCode::None);

        let lingo_strings: LingoStrings = HashMap::from([(
            "hello_world",
            HashMap::from([
                (fr.clone(), "Bonjour le monde !"),
                (en_GB, "Hello world!"),
                (de, "Hallo Welt!"),
            ]),
        )]);

        let lingo = Lingo::with_system_context_locale(fr, lingo_strings);

        println!("{}", lingo.string("hello_world").unwrap());
    }
}
