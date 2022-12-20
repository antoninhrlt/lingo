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
        Lingo::with_system_context_locale(locale!("en"), strings!($($strings),*))
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

    /// Resets the context locale to the operating system locale
    pub fn reset_context_locale(&mut self) {
        self.context_locale = Locale::from_string(&sys_locale::get_locale().unwrap(), '-').unwrap();
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

/// The way to have a Lingo object in the whole application
pub trait LingoApp {
    fn init_lingo() -> Lingo;
    fn lingo(&self) -> &Lingo;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct MainApp {
        lingo: Lingo,
    }

    impl MainApp {
        pub fn new() -> MainApp {
            MainApp { 
                lingo: Self::init_lingo()
            }
        }

        pub fn run(&self) {
            println!("{}", self.lingo.string("hello").unwrap());
        } 
    }

    impl LingoApp for MainApp {
        fn init_lingo() -> Lingo {
            let mut lingo = lingo![
                (
                    "hello", 
                    strings![
                        s!("de", "hallo Welt"),
                        s!("en", "hello world")
                        // ...
                    ]
                )
                // ...
            ];

            lingo.set_context_locale(locale!("de_DE"));
            // Useless because already the default locale :
            //  lingo.set_default_locale(locale!("en"));
            lingo
        }

        fn lingo(&self) -> &Lingo {
            &self.lingo
        }
    }

    #[test]
    fn main_app() {
        let app = MainApp::new();
        app.run();

        println!("{}", app.lingo().string("hello").unwrap());

        // Output :
        //  hallo Welt
        //  hallo Welt
    }
}
