# lingo
Internationalise a Rust program and translate strings quickly and simply. Make any software open to multiple languages

## Installation
In your "Cargo.toml" file :
```toml
[dependencies]
lingo_lib = "*"
```
Check the current version on [crates.io](https://crates.io/crates/lingo_lib).

> Unfortunately, the crate "lingo" already exists in [crates.io](https://crates.io/crates/lingo). Waiting to get the name, be careful to not import "lingo" but "lingo_lib".

## How to use
```rust
let mut lingo = lingo![
    (
        "<identifier>", 
        strings![
            s!("<locale>", "<translation>"),
            s!("<locale>", "<translation>")
            // ...
        ]
    )
    // ...
];

// If not set, it's the operating system locale that will be used.
lingo.set_context_locale(/* app locale */);
// If not set, it's `locale!("en")`.
lingo.set_default_locale(/* locale */);

println!("{}", lingo.string("<identifier>").unwrap());
```

- ## Locale
    ```rust
    locale!("en"); // English (no country code)
    locale!("en_US"); // English (United States)

    Locale::from_string("en_US", '_'); // English (United States)
    Locale::from_string("en-US", '-'); // English (United States)

    // English (no country code)
    Locale(Language::new(LanguageCode::en), CountryCode::None);
    // English (United States) 
    Locale(Language::new(LanguageCode::en), CountryCode::US);
    ```

## Example
A French application using `lingo`.
```rust
let mut lingo = lingo![
    (
        "hello_world", 
        strings![
            s!("fr_BE", "Bonjour le monde !"), // Belgium
            s!("en_GB", "Hello world!")
        ]
    ),
    (
        "love", 
        strings![
            s!("fr_FR", "J'adore Lingo"), // France
            s!("en_GB", "I love Lingo")
        ]
    )
];

lingo.set_context_locale(locale!("fr_FR"));
lingo.set_default_locale(locale!("fr_FR"));

println!("{}", lingo.string("hello_world").unwrap());
println!("{}", lingo.string("love").unwrap());
```

```
Bonjour le monde !
J'adore Lingo
```

If there were no `fr` string, it would use : `s!("en_GB", "Hello world!")`.

<details>

<summary>Imports for the example</summary>

```rust
use std::collections::HashMap;

use lingo_lib::locales::Locale;
use lingo_lib::strings::get_string;
use lingo_lib::Lingo;
use lingo_lib::{lingo, locale, s, strings};
```

</details>
