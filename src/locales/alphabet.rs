// This file is part of "lingo"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Alphabet {
    Latin,
    Cyrillic,
    Simplified,
    Traditional,
    Unspecified,
}

impl Alphabet {
    pub fn from_code(code: &str) -> Result<Alphabet, String> {
        match code {
            "Latn" => Ok(Alphabet::Latin),
            "Cyrl" => Ok(Alphabet::Cyrillic),
            "Hans" => Ok(Alphabet::Simplified),
            "Hant" => Ok(Alphabet::Traditional),
            "" => Ok(Alphabet::Unspecified),
            _ => Err(format!("no corresponding alphabet for code : {code}")),
        }
    }

    pub fn to_code(&self) -> String {
        match self {
            Alphabet::Latin => "Latn",
            Alphabet::Cyrillic => "Cyrl",
            Alphabet::Simplified => "Hans",
            Alphabet::Traditional => "Hant",
            Alphabet::Unspecified => panic!("unspecified alphabet has no associated string code"),
        }
        .to_string()
    }
}
