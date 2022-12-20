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
struct MyFrenchApp {
    lingo: Lingo,
}

impl MyFrenchApp {
    pub fn new() -> Self {
        Self { 
            lingo: Self::init_lingo()
        }
    }

    pub fn run(&self) {
        println!("{}", self.lingo.string("welcome").unwrap());
    } 
}

impl LingoApp for MyFrenchApp {
    fn init_lingo() -> Lingo {
        let mut lingo = lingo![
            (
                "welcome", 
                strings![
                    s!("fr", "bienvenue sur l'app !"),
                    s!("en", "welcome to the app!")
                    // ...
                ]
            )
            // ...
        ];

        lingo.set_context_locale(locale!("fr_FR"));
        lingo
    }

    fn lingo(&self) -> &Lingo {
        &self.lingo
    }
}

fn main() {
    let app = MyFrenchApp::new();
    
    println!("{}", app.lingo().string("welcome").unwrap());
    
    app.run();
}
```

```
bienvenue sur l'app !
bienvenue sur l'app !
```

<details>

<summary>Imports for the example</summary>

```rust
use lingo_lib::{ lingo, locale, strings, s };
use lingo_lib::{ Lingo, LingoApp };
use lingo_lib::locales::Locale;
```

</details>
