use clap::Parser;
use clio::Input;
use highlight::{highlight, Theme};
use std::{io::Read, process::exit};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// The code to syntax highlight
    #[arg(value_parser)]
    code: Option<Input>,

    /// The theme file to highlight with, required if code is passed in
    #[arg(short, long, value_parser)]
    theme: Option<Input>,

    /// The language that the code is on, required if code is passed in
    #[arg(short)]
    language: Option<String>,

    /// List the languages supported by Highlight, values separated by `|` are alternate
    /// names for the same language
    #[arg(long = "list-langs")]
    list_languages: bool,
}

fn main() {
    let args = Args::parse();

    if args.list_languages {
        println!(
            "
agda
c
cpp | c++
css
go
haskell | hs
html
java
javascript | js
jsx
jsdoc
json
ocaml
ocaml_interface
ocaml_type
php
php_only
python | py
regexp | regex
ruby | rb
rust | rs
scala
shellscript | shell | bash | zsh | sh
typescript | ts
tsx
"
        );
        return;
    }

    let mut code_arg = match args.code {
        Some(code) => code,
        None => {
            eprintln!("Code is required");
            exit(1)
        }
    };

    let mut code = String::new();
    match code_arg.read_to_string(&mut code) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error reading from file: {}", err);
            exit(1)
        }
    };

    let lang = match args.language {
        Some(lang) => lang,
        None => {
            eprintln!("Language is required. You can use `highlight --list-langs` to see available languages");
            exit(1)
        }
    };

    let mut theme_arg = match args.theme {
        Some(t) => t,
        None => {
            eprintln!("Theme is required");
            exit(1);
        }
    };
    let mut theme = String::new();

    match theme_arg.read_to_string(&mut theme) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error reading theme from file: {err}");
            exit(1)
        }
    };

    let theme = match Theme::new(theme) {
        Ok(theme) => theme,
        Err(err) => {
            eprintln!("Error parsing theme: {}", err);
            exit(1)
        }
    };

    match highlight(code, lang, theme) {
        Some(highlighted_text) => println!("{}", highlighted_text),
        None => eprintln!("Error highlighting code"),
    }
}
