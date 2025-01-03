use napi_derive::napi;
use std::collections::HashMap;
use tree_sitter_highlight::{Highlight, HighlightConfiguration, Highlighter, HtmlRenderer};

fn get_highlight_query(language: String) -> String {
    let cpp = format!(
        "{}\n{}",
        tree_sitter_c::HIGHLIGHT_QUERY,
        tree_sitter_cpp::HIGHLIGHT_QUERY
    );

    let jsx = format!(
        "{}\n{}",
        tree_sitter_javascript::HIGHLIGHT_QUERY,
        tree_sitter_javascript::JSX_HIGHLIGHT_QUERY
    );

    let ts = format!(
        "{}\n{}",
        tree_sitter_javascript::HIGHLIGHT_QUERY,
        tree_sitter_typescript::HIGHLIGHTS_QUERY
    );

    let tsx = format!(
        "{}\n{}\n{}",
        tree_sitter_javascript::HIGHLIGHT_QUERY,
        tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
        tree_sitter_typescript::HIGHLIGHTS_QUERY
    );

    match language.as_str() {
        "agda" => tree_sitter_agda::HIGHLIGHTS_QUERY.to_string(),
        "c" => tree_sitter_c::HIGHLIGHT_QUERY.to_string(),
        "cpp" | "c++" => cpp,
        "css" => tree_sitter_css::HIGHLIGHTS_QUERY.to_string(),
        "go" | "golang" => tree_sitter_go::HIGHLIGHTS_QUERY.to_string(),
        "haskell" | "hs" => tree_sitter_haskell::HIGHLIGHTS_QUERY.to_string(),
        "html" => tree_sitter_html::HIGHLIGHTS_QUERY.to_string(),
        "java" => tree_sitter_java::HIGHLIGHTS_QUERY.to_string(),
        "javascript" | "js" => tree_sitter_javascript::HIGHLIGHT_QUERY.to_string(),
        "jsx" => jsx,
        "jsdoc" => tree_sitter_jsdoc::HIGHLIGHTS_QUERY.to_string(),
        "json" => tree_sitter_json::HIGHLIGHTS_QUERY.to_string(),
        "ocaml" | "ml" | "ocaml_interface" | "ocaml_type" => {
            tree_sitter_ocaml::HIGHLIGHTS_QUERY.to_string()
        }
        "php" | "php_only" => tree_sitter_php::HIGHLIGHTS_QUERY.to_string(),
        "python" | "py" => tree_sitter_python::HIGHLIGHTS_QUERY.to_string(),
        "ruby" | "rb" => tree_sitter_ruby::HIGHLIGHTS_QUERY.to_string(),
        "rust" | "rs" => tree_sitter_rust::HIGHLIGHTS_QUERY.to_string(),
        "scala" => tree_sitter_scala::HIGHLIGHTS_QUERY.to_string(),
        "shellscript" | "shell" | "bash" | "zsh" | "sh" => {
            tree_sitter_bash::HIGHLIGHT_QUERY.to_string()
        }
        "typescript" | "ts" => ts,
        "tsx" => tsx,
        _ => "".to_string(),
    }
}

fn get_language(language: String) -> HighlightConfiguration {
    let ts_lang = match language.as_str() {
        "agda" => tree_sitter_agda::LANGUAGE,
        "c" => tree_sitter_c::LANGUAGE,
        "cpp" | "c++" => tree_sitter_cpp::LANGUAGE,
        "css" => tree_sitter_css::LANGUAGE,
        "go" | "golang" => tree_sitter_go::LANGUAGE,
        "haskell" | "hs" => tree_sitter_haskell::LANGUAGE,
        "html" => tree_sitter_html::LANGUAGE,
        "java" => tree_sitter_java::LANGUAGE,
        "javascript" | "js" | "jsx" => tree_sitter_javascript::LANGUAGE,
        "jsdoc" => tree_sitter_jsdoc::LANGUAGE,
        "json" => tree_sitter_json::LANGUAGE,
        "ocaml" | "ml" => tree_sitter_ocaml::LANGUAGE_OCAML,
        "ocaml_interface" => tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE,
        "ocaml_type" => tree_sitter_ocaml::LANGUAGE_OCAML_TYPE,
        "php" => tree_sitter_php::LANGUAGE_PHP,
        "php_only" => tree_sitter_php::LANGUAGE_PHP_ONLY,
        "python" | "py" => tree_sitter_python::LANGUAGE,
        "ruby" | "rb" => tree_sitter_ruby::LANGUAGE,
        "rust" | "rs" => tree_sitter_rust::LANGUAGE,
        "scala" => tree_sitter_scala::LANGUAGE,
        "shellscript" | "shell" | "bash" | "zsh" | "sh" => tree_sitter_bash::LANGUAGE,
        "typescript" | "ts" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
        "tsx" => tree_sitter_typescript::LANGUAGE_TSX,
        _ => panic!("Language not supported!"),
    };

    let highlight_query = get_highlight_query(language.clone());

    let injections_query = match language.as_str() {
        "haskell" | "hs" => tree_sitter_haskell::INJECTIONS_QUERY,
        "html" => tree_sitter_html::INJECTIONS_QUERY,
        "javascript" | "js" | "jsx" => tree_sitter_javascript::INJECTIONS_QUERY,
        "php" | "php_only" => tree_sitter_php::INJECTIONS_QUERY,
        "rust" | "rs" => tree_sitter_rust::INJECTIONS_QUERY,
        _ => "",
    };

    let locals_query = match language.as_str() {
        "haskell" | "hs" => tree_sitter_haskell::LOCALS_QUERY,
        "javascript" | "js" | "jsx" => tree_sitter_javascript::LOCALS_QUERY,
        "ocaml" | "ml" | "ocaml_interface" | "ocaml_type" => tree_sitter_ocaml::LOCALS_QUERY,
        "ruby" | "rb" => tree_sitter_ruby::LOCALS_QUERY,
        "scala" => tree_sitter_scala::LOCALS_QUERY,
        "typescript" | "ts" | "tsx" => tree_sitter_typescript::LOCALS_QUERY,
        _ => "",
    };

    HighlightConfiguration::new(
        ts_lang.into(),
        language,
        highlight_query.as_str(),
        injections_query,
        locals_query,
    )
    .unwrap()
}

fn escape_html(input: String) -> String {
    input
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace("'", "&#x27;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub const BUNDLED_LANGUAGES: [&str; 41] = [
    // Agda
    "agda",
    // C
    "c",
    // C++
    "cpp",
    "c++",
    // CSS
    "css",
    // Go
    "go",
    "golang",
    // Haskell
    "haskell",
    "hs",
    // HTML
    "html",
    // Java
    "java",
    // JavaScript
    "javascript",
    "js",
    "jsx",
    // JSDoc
    "jsdoc",
    // JSON
    "json",
    // OCaml
    "ocaml",
    "ml",
    "ocaml_interface",
    "ocaml_type",
    // PHP
    "php",
    "php_only",
    // Python
    "python",
    "py",
    // Ruby
    "ruby",
    "rb",
    // Rust
    "rust",
    "rs",
    // Scala
    "scala",
    // Shell
    "shellscript",
    "shell",
    "bash",
    "zsh",
    "sh",
    // TypeScript
    "typescript",
    "ts",
    "tsx",
    // Plain
    "plaintext",
    "plain",
    "text",
    "txt",
];

#[napi(object)]
#[derive(Clone)]
pub struct HighlightType {
    pub color: Option<String>,
    #[napi(js_name = "fontStyle")]
    pub font_style: Option<String>,
    #[napi(js_name = "fontWeight")]
    pub font_weight: Option<u16>, // "italic" | "normal" | "oblique"
    #[napi(js_name = "backgroundColor")]
    pub background_color: Option<String>,
}

/// The type of theme accepted by the Highlight package.
#[napi(object)]
pub struct Theme {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub highlights: Option<HashMap<String, HighlightType>>,
}

/**
A function that takes in code and highlights it.

@param {string} code The code to highlight.
@param {BundledLanguage} language The programming language the code is written in, must be one of the languages supported by Highlight.
@param {Theme} [theme] A theme to syntax highlight with. There is no theme provided by default, so without one, no highlighting will be present.

@returns An HTML string with the syntax highlighted colors inlined in `style` attributes.

@example
```ts
import { highlight, type Theme } from "@noclaps/highlight";

const theme: Theme = {
... // Your theme here
}

const myCode = `
console.log("Hello World");
`

const htmlOutput = highlight(code, "ts", theme);
// <pre>...</pre> HTML output
```
*/
#[napi]
pub fn highlight(
    code: String,
    #[napi(ts_arg_type = "BundledLanguage")] language: String,
    theme: Option<Theme>,
) -> String {
    if !BUNDLED_LANGUAGES.contains(&language.as_str()) {
        panic!("Language {} is not supported by Highlight", language);
    }
    let theme = theme.unwrap_or(Theme {
        fg: None,
        bg: None,
        highlights: None,
    });

    let mut global_style = "".to_string();
    if theme.bg.is_some() {
        global_style += format!("background-color:{};", theme.bg.unwrap_or_default()).as_str()
    }
    if theme.fg.is_some() {
        global_style += format!("color:{};", theme.fg.unwrap_or_default()).as_str()
    }

    if language == "plaintext" || language == "plain" || language == "text" || language == "txt" {
        return format!(
            "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
            global_style,
            escape_html(code)
        );
    }

    let mut highlighter = Highlighter::new();
    let highlight_names = theme
        .highlights
        .clone()
        .unwrap_or_default()
        .keys()
        .cloned()
        .collect::<Vec<String>>();

    let mut config = get_language(language);
    config.configure(&highlight_names);

    let highlight_classes: Vec<String> = highlight_names
        .iter()
        .map(|name| format!("class=\"{}\"", name))
        .collect();
    let attributes_callback = |h: Highlight| highlight_classes[h.0].as_bytes();

    let highlights = highlighter
        .highlight(&config, code.as_bytes(), None, |_| None)
        .unwrap();

    let mut html_renderer = HtmlRenderer::new();
    let _ = html_renderer.render(highlights, code.as_bytes(), &attributes_callback);

    let mut highlighted_text = html_renderer.lines().collect::<Vec<&str>>().join("");

    for key in theme.highlights.clone().unwrap_or_default().keys().cloned() {
        let mut style = "".to_string();
        let highlights = theme
            .highlights
            .clone()
            .unwrap_or_default()
            .get(&key)
            .unwrap_or(&HighlightType {
                color: None,
                font_style: None,
                font_weight: None,
                background_color: None,
            })
            .clone();

        match highlights.color {
            Some(color) => {
                style += format!("color:{};", color).as_str();
            }
            None => (),
        }
        match highlights.font_style {
            Some(font_style) => {
                style += format!("font-style:{};", font_style).as_str();
            }
            None => (),
        }
        match highlights.font_weight {
            Some(font_weight) => {
                style += format!("font-weight:{};", font_weight).as_str();
            }
            None => (),
        }
        match highlights.background_color {
            Some(background_color) => {
                style += format!("background-color:{};", background_color).as_str();
            }
            None => (),
        }

        highlighted_text = highlighted_text.replace(
            format!("<span class=\"{}\"", key).as_str(),
            format!("<span class=\"{}\" style=\"{}\"", key, style).as_str(),
        )
    }

    format!(
        "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
        global_style, highlighted_text
    )
}
