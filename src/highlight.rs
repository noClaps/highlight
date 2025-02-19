use std::process::exit;

use tree_sitter_highlight::{HighlightConfiguration, Highlighter, HtmlRenderer};

fn get_highlight_query(language: String) -> String {
    let cpp = format!(
        "{}\n{}",
        tree_sitter_c::HIGHLIGHT_QUERY,
        tree_sitter_cpp::HIGHLIGHT_QUERY
    );

    let jsx = format!(
        "{}\n{}",
        tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
        tree_sitter_javascript::HIGHLIGHT_QUERY
    );

    let ts = format!(
        "{}\n{}",
        tree_sitter_typescript::HIGHLIGHTS_QUERY,
        tree_sitter_javascript::HIGHLIGHT_QUERY
    );

    let tsx = format!(
        "{}\n{}\n{}",
        tree_sitter_typescript::HIGHLIGHTS_QUERY,
        tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
        tree_sitter_javascript::HIGHLIGHT_QUERY
    );

    match language.as_str() {
        "agda" => tree_sitter_agda::HIGHLIGHTS_QUERY.to_string(),
        "c" => tree_sitter_c::HIGHLIGHT_QUERY.to_string(),
        "cpp" | "c++" => cpp,
        "css" => tree_sitter_css::HIGHLIGHTS_QUERY.to_string(),
        "go" => tree_sitter_go::HIGHLIGHTS_QUERY.to_string(),
        "haskell" | "hs" => tree_sitter_haskell::HIGHLIGHTS_QUERY.to_string(),
        "html" => tree_sitter_html::HIGHLIGHTS_QUERY.to_string(),
        "java" => tree_sitter_java::HIGHLIGHTS_QUERY.to_string(),
        "javascript" | "js" => tree_sitter_javascript::HIGHLIGHT_QUERY.to_string(),
        "jsx" => jsx,
        "jsdoc" => tree_sitter_jsdoc::HIGHLIGHTS_QUERY.to_string(),
        "json" => tree_sitter_json::HIGHLIGHTS_QUERY.to_string(),
        "ocaml" | "ocaml_interface" | "ocaml_type" => {
            tree_sitter_ocaml::HIGHLIGHTS_QUERY.to_string()
        }
        "php" | "php_only" => tree_sitter_php::HIGHLIGHTS_QUERY.to_string(),
        "python" | "py" => tree_sitter_python::HIGHLIGHTS_QUERY.to_string(),
        "regexp" | "regex" => tree_sitter_regex::HIGHLIGHTS_QUERY.to_string(),
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

fn get_locals_query(language: String) -> String {
    let ts = format!(
        "{}\n{}",
        tree_sitter_typescript::LOCALS_QUERY,
        tree_sitter_javascript::LOCALS_QUERY
    );
    let tsx = tree_sitter_javascript::LOCALS_QUERY.to_string();

    match language.as_str() {
        "haskell" | "hs" => tree_sitter_haskell::LOCALS_QUERY.to_string(),
        "javascript" | "js" | "jsx" => tree_sitter_javascript::LOCALS_QUERY.to_string(),
        "ocaml" | "ocaml_interface" | "ocaml_type" => tree_sitter_ocaml::LOCALS_QUERY.to_string(),
        "ruby" | "rb" => tree_sitter_ruby::LOCALS_QUERY.to_string(),
        "scala" => tree_sitter_scala::LOCALS_QUERY.to_string(),
        "typescript" | "ts" => ts,
        "tsx" => tsx,
        _ => "".to_string(),
    }
}

fn get_injections_query(language: String) -> String {
    let ts = tree_sitter_javascript::INJECTIONS_QUERY.to_string();

    match language.as_str() {
        "haskell" | "hs" => tree_sitter_haskell::INJECTIONS_QUERY.to_string(),
        "html" => tree_sitter_html::INJECTIONS_QUERY.to_string(),
        "javascript" | "js" | "jsx" => tree_sitter_javascript::INJECTIONS_QUERY.to_string(),
        "php" | "php_only" => tree_sitter_php::INJECTIONS_QUERY.to_string(),
        "rust" | "rs" => tree_sitter_rust::INJECTIONS_QUERY.to_string(),
        "typescript" | "ts" | "tsx" => ts,
        _ => "".to_string(),
    }
}

fn get_language(language: String) -> HighlightConfiguration {
    let ts_lang = match language.as_str() {
        "agda" => tree_sitter_agda::LANGUAGE,
        "c" => tree_sitter_c::LANGUAGE,
        "cpp" | "c++" => tree_sitter_cpp::LANGUAGE,
        "css" => tree_sitter_css::LANGUAGE,
        "go" => tree_sitter_go::LANGUAGE,
        "haskell" | "hs" => tree_sitter_haskell::LANGUAGE,
        "html" => tree_sitter_html::LANGUAGE,
        "java" => tree_sitter_java::LANGUAGE,
        "javascript" | "js" | "jsx" => tree_sitter_javascript::LANGUAGE,
        "jsdoc" => tree_sitter_jsdoc::LANGUAGE,
        "json" => tree_sitter_json::LANGUAGE,
        "ocaml" => tree_sitter_ocaml::LANGUAGE_OCAML,
        "ocaml_interface" => tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE,
        "ocaml_type" => tree_sitter_ocaml::LANGUAGE_OCAML_TYPE,
        "php" => tree_sitter_php::LANGUAGE_PHP,
        "php_only" => tree_sitter_php::LANGUAGE_PHP_ONLY,
        "python" | "py" => tree_sitter_python::LANGUAGE,
        "regexp" | "regex" => tree_sitter_regex::LANGUAGE,
        "ruby" | "rb" => tree_sitter_ruby::LANGUAGE,
        "rust" | "rs" => tree_sitter_rust::LANGUAGE,
        "scala" => tree_sitter_scala::LANGUAGE,
        "shellscript" | "shell" | "bash" | "zsh" | "sh" => tree_sitter_bash::LANGUAGE,
        "typescript" | "ts" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
        "tsx" => tree_sitter_typescript::LANGUAGE_TSX,
        _ => {
            eprintln!("Language {language} is not supported by Highlight");
            exit(1)
        }
    };

    let highlight_query = get_highlight_query(language.clone());
    let locals_query = get_locals_query(language.clone());
    let injections_query = get_injections_query(language.clone());

    HighlightConfiguration::new(
        ts_lang.into(),
        language,
        highlight_query.as_str(),
        injections_query.as_str(),
        locals_query.as_str(),
    )
    .unwrap()
}

pub fn highlight_code(highlight_names: Vec<String>, language: String, code: String) -> String {
    let mut highlighter = Highlighter::new();

    let mut config = get_language(language);
    config.configure(&highlight_names);

    let highlight_classes: Vec<String> = highlight_names
        .iter()
        .map(|name| format!("class=\"{}\"", name))
        .collect();

    let highlights = highlighter
        .highlight(&config, code.as_bytes(), None, |capture| {
            let mut nested_config = get_language(capture.to_string());
            nested_config.configure(&highlight_names);
            // Leak the new configuration so that it has a 'static lifetime.
            Some(Box::leak(Box::new(nested_config)))
        })
        .unwrap();

    let mut html_renderer = HtmlRenderer::new();
    let _ = html_renderer.render(highlights, code.as_bytes(), &move |h, output| {
        output.extend(highlight_classes[h.0].as_bytes());
    });

    html_renderer.lines().collect::<Vec<&str>>().join("")
}
