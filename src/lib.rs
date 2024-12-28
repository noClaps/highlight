use tree_sitter_highlight::{Highlight, HighlightConfiguration, Highlighter, HtmlRenderer};

#[macro_use]
extern crate napi_derive;

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
        "c" => tree_sitter_c::HIGHLIGHT_QUERY.to_string(),
        "codeql" => tree_sitter_ql::HIGHLIGHTS_QUERY.to_string(),
        "ql" => tree_sitter_ql::HIGHLIGHTS_QUERY.to_string(),
        "cpp" => cpp,
        "c++" => cpp,
        "css" => tree_sitter_css::HIGHLIGHTS_QUERY.to_string(),
        "ejs" => tree_sitter_embedded_template::HIGHLIGHTS_QUERY.to_string(),
        "erb" => tree_sitter_embedded_template::HIGHLIGHTS_QUERY.to_string(),
        "go" => tree_sitter_go::HIGHLIGHTS_QUERY.to_string(),
        "golang" => tree_sitter_go::HIGHLIGHTS_QUERY.to_string(),
        "haskell" => tree_sitter_haskell::HIGHLIGHTS_QUERY.to_string(),
        "hs" => tree_sitter_haskell::HIGHLIGHTS_QUERY.to_string(),
        "html" => tree_sitter_html::HIGHLIGHTS_QUERY.to_string(),
        "java" => tree_sitter_java::HIGHLIGHTS_QUERY.to_string(),
        "javascript" => tree_sitter_javascript::HIGHLIGHT_QUERY.to_string(),
        "js" => tree_sitter_javascript::HIGHLIGHT_QUERY.to_string(),
        "jsx" => jsx,
        "jsdoc" => tree_sitter_jsdoc::HIGHLIGHTS_QUERY.to_string(),
        "json" => tree_sitter_json::HIGHLIGHTS_QUERY.to_string(),
        "ocaml" => tree_sitter_ocaml::HIGHLIGHTS_QUERY.to_string(),
        "ml" => tree_sitter_ocaml::HIGHLIGHTS_QUERY.to_string(),
        "ocaml_interface" => tree_sitter_ocaml::HIGHLIGHTS_QUERY.to_string(),
        "ocaml_type" => tree_sitter_ocaml::HIGHLIGHTS_QUERY.to_string(),
        "php" => tree_sitter_php::HIGHLIGHTS_QUERY.to_string(),
        "php_only" => tree_sitter_php::HIGHLIGHTS_QUERY.to_string(),
        "python" => tree_sitter_python::HIGHLIGHTS_QUERY.to_string(),
        "py" => tree_sitter_python::HIGHLIGHTS_QUERY.to_string(),
        "regex" => tree_sitter_regex::HIGHLIGHTS_QUERY.to_string(),
        "regexp" => tree_sitter_regex::HIGHLIGHTS_QUERY.to_string(),
        "ruby" => tree_sitter_ruby::HIGHLIGHTS_QUERY.to_string(),
        "rb" => tree_sitter_ruby::HIGHLIGHTS_QUERY.to_string(),
        "rust" => tree_sitter_rust::HIGHLIGHTS_QUERY.to_string(),
        "rs" => tree_sitter_rust::HIGHLIGHTS_QUERY.to_string(),
        "scala" => tree_sitter_scala::HIGHLIGHTS_QUERY.to_string(),
        "shellscript" => tree_sitter_bash::HIGHLIGHT_QUERY.to_string(),
        "shell" => tree_sitter_bash::HIGHLIGHT_QUERY.to_string(),
        "bash" => tree_sitter_bash::HIGHLIGHT_QUERY.to_string(),
        "sh" => tree_sitter_bash::HIGHLIGHT_QUERY.to_string(),
        "typescript" => ts,
        "ts" => ts,
        "tsx" => tsx,
        _ => "".to_string(),
    }
}

fn get_language(language: String) -> HighlightConfiguration {
    let ts_lang = match language.as_str() {
        "c" => tree_sitter_c::LANGUAGE.into(),
        "codeql" => tree_sitter_ql::LANGUAGE.into(),
        "ql" => tree_sitter_ql::LANGUAGE.into(),
        "cpp" => tree_sitter_cpp::LANGUAGE.into(),
        "c++" => tree_sitter_cpp::LANGUAGE.into(),
        "csharp" => tree_sitter_c_sharp::LANGUAGE.into(),
        "c#" => tree_sitter_c_sharp::LANGUAGE.into(),
        "cs" => tree_sitter_c_sharp::LANGUAGE.into(),
        "css" => tree_sitter_css::LANGUAGE.into(),
        "ejs" => tree_sitter_embedded_template::LANGUAGE.into(),
        "erb" => tree_sitter_embedded_template::LANGUAGE.into(),
        "go" => tree_sitter_go::LANGUAGE.into(),
        "golang" => tree_sitter_go::LANGUAGE.into(),
        "haskell" => tree_sitter_haskell::LANGUAGE.into(),
        "hs" => tree_sitter_haskell::LANGUAGE.into(),
        "html" => tree_sitter_html::LANGUAGE.into(),
        "java" => tree_sitter_java::LANGUAGE.into(),
        "javascript" => tree_sitter_javascript::LANGUAGE.into(),
        "js" => tree_sitter_javascript::LANGUAGE.into(),
        "jsx" => tree_sitter_javascript::LANGUAGE.into(),
        "jsdoc" => tree_sitter_jsdoc::LANGUAGE.into(),
        "json" => tree_sitter_json::LANGUAGE.into(),
        "julia" => tree_sitter_julia::LANGUAGE.into(),
        "jl" => tree_sitter_julia::LANGUAGE.into(),
        "ml" => tree_sitter_ocaml::LANGUAGE_OCAML.into(),
        "ocaml" => tree_sitter_ocaml::LANGUAGE_OCAML.into(),
        "ocaml_interface" => tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE.into(),
        "ocaml_type" => tree_sitter_ocaml::LANGUAGE_OCAML_TYPE.into(),
        "php" => tree_sitter_php::LANGUAGE_PHP.into(),
        "php_only" => tree_sitter_php::LANGUAGE_PHP_ONLY.into(),
        "python" => tree_sitter_python::LANGUAGE.into(),
        "py" => tree_sitter_python::LANGUAGE.into(),
        "regex" => tree_sitter_regex::LANGUAGE.into(),
        "regexp" => tree_sitter_regex::LANGUAGE.into(),
        "ruby" => tree_sitter_ruby::LANGUAGE.into(),
        "rb" => tree_sitter_ruby::LANGUAGE.into(),
        "rust" => tree_sitter_rust::LANGUAGE.into(),
        "rs" => tree_sitter_rust::LANGUAGE.into(),
        "scala" => tree_sitter_scala::LANGUAGE.into(),
        "shellscript" => tree_sitter_bash::LANGUAGE.into(),
        "shell" => tree_sitter_bash::LANGUAGE.into(),
        "bash" => tree_sitter_bash::LANGUAGE.into(),
        "sh" => tree_sitter_bash::LANGUAGE.into(),
        "tsx" => tree_sitter_typescript::LANGUAGE_TSX.into(),
        "typescript" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        "ts" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        "verilog" => tree_sitter_verilog::LANGUAGE.into(),
        _ => panic!("Language not supported!"),
    };

    let highlight_query = get_highlight_query(language.clone());

    let injections_query = match language.as_str() {
        "ejs" => tree_sitter_embedded_template::INJECTIONS_EJS_QUERY,
        "erb" => tree_sitter_embedded_template::INJECTIONS_ERB_QUERY,
        "haskell" => tree_sitter_haskell::INJECTIONS_QUERY,
        "hs" => tree_sitter_haskell::INJECTIONS_QUERY,
        "html" => tree_sitter_html::INJECTIONS_QUERY,
        "javascript" => tree_sitter_javascript::INJECTIONS_QUERY,
        "js" => tree_sitter_javascript::INJECTIONS_QUERY,
        "jsx" => tree_sitter_javascript::INJECTIONS_QUERY,
        "php" => tree_sitter_php::INJECTIONS_QUERY,
        "php_only" => tree_sitter_php::INJECTIONS_QUERY,
        "rust" => tree_sitter_rust::INJECTIONS_QUERY,
        "rs" => tree_sitter_rust::INJECTIONS_QUERY,
        _ => "",
    };

    let locals_query = match language.as_str() {
        "haskell" => tree_sitter_haskell::LOCALS_QUERY,
        "hs" => tree_sitter_haskell::LOCALS_QUERY,
        "javascript" => tree_sitter_javascript::LOCALS_QUERY,
        "js" => tree_sitter_javascript::LOCALS_QUERY,
        "jsx" => tree_sitter_javascript::LOCALS_QUERY,
        "ml" => tree_sitter_ocaml::LOCALS_QUERY,
        "ocaml" => tree_sitter_ocaml::LOCALS_QUERY,
        "ocaml_interface" => tree_sitter_ocaml::LOCALS_QUERY,
        "ocaml_type" => tree_sitter_ocaml::LOCALS_QUERY,
        "ruby" => tree_sitter_ruby::LOCALS_QUERY,
        "rb" => tree_sitter_ruby::LOCALS_QUERY,
        "scala" => tree_sitter_scala::LOCALS_QUERY,
        "tsx" => tree_sitter_typescript::LOCALS_QUERY,
        "typescript" => tree_sitter_typescript::LOCALS_QUERY,
        "ts" => tree_sitter_typescript::LOCALS_QUERY,
        _ => "",
    };

    HighlightConfiguration::new(
        ts_lang,
        language,
        highlight_query.as_str(),
        injections_query,
        locals_query,
    )
    .unwrap()
}

#[napi]
pub fn highlight(highlight_names: Vec<String>, language: String, code: String) -> String {
    let mut highlighter = Highlighter::new();

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

    html_renderer.lines().collect::<Vec<&str>>().join("")
}
