mod highlight;
mod types;

use crate::highlight::highlight_code;
use crate::types::Language;

pub use crate::types::Theme;

fn escape_html(input: String) -> String {
    input
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace("'", "&#x27;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn highlight(code: String, language: String, theme: Theme) -> String {
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

    let ts_language = match Language::from_string(&language) {
        Some(lang) => lang,
        None => {
            eprintln!("Language not supported: {language}, continuing as plaintext");
            return format!(
                "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
                global_style,
                escape_html(code)
            );
        }
    };

    if language == "plaintext" || language == "plain" || language == "text" || language == "txt" {
        return format!(
            "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
            global_style,
            escape_html(code)
        );
    }

    let highlight_names = theme.highlights.keys().map(|k| k.to_owned()).collect();
    let mut highlighted_text = highlight_code(highlight_names, ts_language, code.clone());

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

    format!(
        "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
        global_style,
        highlighted_text.trim()
    )
}

#[cfg(test)]
use std::fs::{read, write};
#[cfg(test)]
const LANGUAGE: &str = "c";

#[test]
fn test() {
    let code = match read(format!("test/test.{LANGUAGE}")) {
        Ok(code) => match String::from_utf8(code) {
            Ok(string) => string,
            Err(err) => panic!("ERROR: Error reading test file: {err}"),
        },
        Err(err) => panic!("{err}"),
    };
    let theme = match Theme::new(include_str!("../theme.toml").to_string()) {
        Ok(theme) => theme,
        Err(err) => panic!("ERROR: Error parsing theme: {err}"),
    };
    match write("out.html", highlight(code, LANGUAGE.to_string(), theme)) {
        Ok(_) => (),
        Err(err) => panic!("ERROR: Error writing output: {err}"),
    };
}

#[test]
fn test_blank() {
    let code = match read(format!("test/test.{LANGUAGE}")) {
        Ok(code) => match String::from_utf8(code) {
            Ok(string) => string,
            Err(err) => panic!("ERROR: Error reading test file: {err}"),
        },
        Err(err) => panic!("{err}"),
    };
    let theme = Theme::blank();
    match write(
        "out-blank.html",
        highlight(code, LANGUAGE.to_string(), theme),
    ) {
        Ok(_) => (),
        Err(err) => panic!("ERROR: Error writing output: {err}"),
    };
}
