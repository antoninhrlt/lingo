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

#[macro_export]
macro_rules! strings {
    ($string:expr) => {
        HashMap::from([$string])
    };

    ($($strings:expr),*) => {
        HashMap::from([
            $($strings),*
        ])
    };
}

#[macro_export]
macro_rules! s {
    ($locale_str:expr, $string:expr) => {
        (crate::locale!($locale_str), $string)
    };
}

#[macro_export]
macro_rules! lingo {
    ($($strings:expr),*) => {
        Lingo::with_system_context_locale(locale!("en"), strings!($($strings),*));
    };
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

    pub fn with_system_context_locale(default_locale: Locale, strings: LingoStrings) -> Lingo {
        Lingo {
            context_locale: Locale::from_string(&sys_locale::get_locale().unwrap(), '-').unwrap(),
            default_locale,
            strings,
        }
    }

    pub fn set_context_locale(&mut self, context_locale: Locale) {
        self.context_locale = context_locale;
    }

    pub fn set_default_locale(&mut self, default_locale: Locale) {
        self.default_locale = default_locale;
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
