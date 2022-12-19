// This file is part of "lingo"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

pub mod locales;
pub mod strings;

use locales::Locale;
use strings::LingoStrings;

pub fn hello() {
    println!("hello from the lingo library !");
}

pub struct Lingo {
    context_locale: Locale,
    default_locale: Locale,
    strings: LingoStrings,
}

impl Lingo {
    pub fn new(context_locale: Locale, default_locale: Locale, strings: LingoStrings) -> Lingo {
        Lingo {
            context_locale,
            default_locale,
            strings,
        }
    }

    pub fn context_locale(&self) -> &Locale {
        &self.context_locale
    }

    pub fn default_locale(&self) -> &Locale {
        &self.default_locale
    }

    pub fn strings(&self) -> &LingoStrings {
        &self.strings
    }
}
