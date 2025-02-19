mod highlight;
mod types;

use clap::Parser;
use clio::Input;
use highlight::highlight;
use std::{io::Read, process::exit};
use types::Theme;

fn escape_html(input: String) -> String {
    input
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace("'", "&#x27;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// The code to syntax highlight
    #[arg(value_parser)]
    code: Option<String>,

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

    let code = match args.code {
        Some(code) => code,
        None => {
            eprintln!("Code is required");
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
    let mut theme: String = String::new();

    match theme_arg.read_to_string(&mut theme) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error reading theme from file: {err}");
            exit(1)
        }
    };

    let theme = match toml::from_str::<Theme>(theme.as_str()) {
        Ok(theme) => theme,
        Err(err) => {
            eprintln!("Error parsing theme: {err}");
            exit(1)
        }
    };

    let mut global_style = "".to_string();
    match theme.bg {
        Some(bg) => {
            global_style += format!("background-color:{};", bg).as_str();
        }
        None => (),
    }
    match theme.fg {
        Some(fg) => {
            global_style += format!("color:{};", fg).as_str();
        }
        None => (),
    }

    if lang == "plaintext" || lang == "plain" || lang == "text" || lang == "txt" {
        println!(
            "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
            global_style,
            escape_html(code)
        );
        return;
    }

    let highlight_names = theme.highlights.keys().map(|k| k.to_owned()).collect();
    let mut highlighted_text = highlight(highlight_names, lang, code);

    for (key, val) in theme.highlights {
        let mut style = "".to_string();
        match val.color {
            Some(color) => {
                style += format!("color:{};", color).as_str();
            }
            None => (),
        }
        match val.font_weight {
            Some(fw) => {
                style += format!("font-weight:{};", fw).as_str();
            }
            None => (),
        }
        match val.font_style {
            Some(fs) => {
                style += format!("font-style:{};", fs).as_str();
            }
            None => (),
        }
        match val.background_color {
            Some(bg) => {
                style += format!("background-color:{};", bg).as_str();
            }
            None => (),
        }

        highlighted_text = highlighted_text.replace(
            format!("<span class=\"{key}\"").as_str(),
            format!("<span class=\"{key}\" style=\"{style}\"").as_str(),
        );
    }

    match theme.line_numbers {
        Some(line_numbers) => {
            let max_line_num = (highlighted_text.lines().count() + 1).to_string().len();
            let right_space = match line_numbers.right_space {
                Some(r) => r,
                None => 1,
            } as usize;

            highlighted_text = highlighted_text
                .lines()
                .enumerate()
                .map(|(index, line)| {
                    format!(
                        "<span class=\"line-number\" style=\"color:{};margin-right:{}ch\">{}</span>{}",
                        line_numbers.color,
                        max_line_num + right_space - (index + 1).to_string().len(),
                        index+1,
                        line
                    )
                })
                .collect::<Vec<String>>().join("\n");
        }
        None => (),
    }

    print!(
        "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code>",
        global_style, highlighted_text
    )
}
