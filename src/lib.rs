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

    let svelte = format!(
        "{}\n{}",
        tree_sitter_html::HIGHLIGHTS_QUERY,
        tree_sitter_svelte_ng::HIGHLIGHTS_QUERY
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
        "cpp" | "c++" => cpp,
        "css" => tree_sitter_css::HIGHLIGHTS_QUERY.to_string(),
        "diff" => tree_sitter_diff::HIGHLIGHTS_QUERY.to_string(),
        "elixir" | "ex" | "exs" => tree_sitter_elixir::HIGHLIGHTS_QUERY.to_string(),
        "fsharp" | "f#" | "fs" | "fsx" | "fsi" => tree_sitter_fsharp::HIGHLIGHTS_QUERY.to_string(),
        "go" | "golang" => tree_sitter_go::HIGHLIGHTS_QUERY.to_string(),
        "haskell" | "hs" => tree_sitter_haskell::HIGHLIGHTS_QUERY.to_string(),
        "html" => tree_sitter_html::HIGHLIGHTS_QUERY.to_string(),
        "java" => tree_sitter_java::HIGHLIGHTS_QUERY.to_string(),
        "javascript" | "js" => tree_sitter_javascript::HIGHLIGHT_QUERY.to_string(),
        "jsx" => jsx,
        "jsdoc" => tree_sitter_jsdoc::HIGHLIGHTS_QUERY.to_string(),
        "json" => tree_sitter_json::HIGHLIGHTS_QUERY.to_string(),
        "kotlin" | "kt" | "kts" => tree_sitter_kotlin_sg::HIGHLIGHTS_QUERY.to_string(),
        "lua" => tree_sitter_lua::HIGHLIGHTS_QUERY.to_string(),
        "luau" => tree_sitter_luau::HIGHLIGHTS_QUERY.to_string(),
        "make" | "makefile" => tree_sitter_make::HIGHLIGHTS_QUERY.to_string(),
        "objc" | "objective-c" => tree_sitter_objc::HIGHLIGHTS_QUERY.to_string(),
        "ocaml" | "ml" | "ocaml_interface" | "ocaml_type" => {
            tree_sitter_ocaml::HIGHLIGHTS_QUERY.to_string()
        }
        "odin" => tree_sitter_odin::HIGHLIGHTS_QUERY.to_string(),
        "php" | "php_only" => tree_sitter_php::HIGHLIGHTS_QUERY.to_string(),
        "python" | "py" => tree_sitter_python::HIGHLIGHTS_QUERY.to_string(),
        "r" => tree_sitter_r::HIGHLIGHTS_QUERY.to_string(),
        "ruby" | "rb" => tree_sitter_ruby::HIGHLIGHTS_QUERY.to_string(),
        "rust" | "rs" => tree_sitter_rust::HIGHLIGHTS_QUERY.to_string(),
        "scala" => tree_sitter_scala::HIGHLIGHTS_QUERY.to_string(),
        "shellscript" | "shell" | "bash" | "zsh" | "sh" => {
            tree_sitter_bash::HIGHLIGHT_QUERY.to_string()
        }
        "sql" => tree_sitter_sequel::HIGHLIGHTS_QUERY.to_string(),
        "svelte" => svelte,
        "swift" => tree_sitter_swift::HIGHLIGHTS_QUERY.to_string(),
        "toml" => tree_sitter_toml_ng::HIGHLIGHTS_QUERY.to_string(),
        "typescript" | "ts" => ts,
        "tsx" => tsx,
        "xml" | "svg" | "xsd" | "xslt" | "xsl" | "rng" => {
            tree_sitter_xml::XML_HIGHLIGHT_QUERY.to_string()
        }
        "dtd" => tree_sitter_xml::DTD_HIGHLIGHT_QUERY.to_string(),
        "yaml" | "yml" => tree_sitter_yaml::HIGHLIGHTS_QUERY.to_string(),
        "zig" => tree_sitter_zig::HIGHLIGHTS_QUERY.to_string(),
        _ => "".to_string(),
    }
}

fn get_language(language: String) -> HighlightConfiguration {
    let ts_lang = match language.as_str() {
        "c" => tree_sitter_c::LANGUAGE,
        "cpp" | "c++" => tree_sitter_cpp::LANGUAGE,
        "css" => tree_sitter_css::LANGUAGE,
        "diff" => tree_sitter_diff::LANGUAGE,
        "elixir" | "ex" | "exs" => tree_sitter_elixir::LANGUAGE,
        "fsharp" | "f#" | "fs" | "fsx" => tree_sitter_fsharp::LANGUAGE_FSHARP,
        "fsi" => tree_sitter_fsharp::LANGUAGE_SIGNATURE,
        "go" | "golang" => tree_sitter_go::LANGUAGE,
        "haskell" | "hs" => tree_sitter_haskell::LANGUAGE,
        "html" => tree_sitter_html::LANGUAGE,
        "java" => tree_sitter_java::LANGUAGE,
        "javascript" | "js" | "jsx" => tree_sitter_javascript::LANGUAGE,
        "jsdoc" => tree_sitter_jsdoc::LANGUAGE,
        "json" => tree_sitter_json::LANGUAGE,
        "kotlin" | "kt" | "kts" => tree_sitter_kotlin_sg::LANGUAGE,
        "lua" => tree_sitter_lua::LANGUAGE,
        "luau" => tree_sitter_luau::LANGUAGE,
        "make" | "makefile" => tree_sitter_make::LANGUAGE,
        "objc" | "objective-c" => tree_sitter_objc::LANGUAGE,
        "ocaml" | "ml" => tree_sitter_ocaml::LANGUAGE_OCAML,
        "ocaml_interface" => tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE,
        "ocaml_type" => tree_sitter_ocaml::LANGUAGE_OCAML_TYPE,
        "odin" => tree_sitter_odin::LANGUAGE,
        "php" => tree_sitter_php::LANGUAGE_PHP,
        "php_only" => tree_sitter_php::LANGUAGE_PHP_ONLY,
        "python" | "py" => tree_sitter_python::LANGUAGE,
        "r" => tree_sitter_r::LANGUAGE,
        "ruby" | "rb" => tree_sitter_ruby::LANGUAGE,
        "rust" | "rs" => tree_sitter_rust::LANGUAGE,
        "scala" => tree_sitter_scala::LANGUAGE,
        "shellscript" | "shell" | "bash" | "zsh" | "sh" => tree_sitter_bash::LANGUAGE,
        "sql" => tree_sitter_sequel::LANGUAGE,
        "svelte" => tree_sitter_svelte_ng::LANGUAGE,
        "swift" => tree_sitter_swift::LANGUAGE,
        "toml" => tree_sitter_toml_ng::LANGUAGE,
        "typescript" | "ts" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
        "tsx" => tree_sitter_typescript::LANGUAGE_TSX,
        "xml" | "svg" | "xsd" | "xslt" | "xsl" | "rng" => tree_sitter_xml::LANGUAGE_XML,
        "dtd" => tree_sitter_xml::LANGUAGE_DTD,
        "yaml" | "yml" => tree_sitter_yaml::LANGUAGE,
        "zig" => tree_sitter_zig::LANGUAGE,
        _ => panic!("Language not supported!"),
    };

    let highlight_query = get_highlight_query(language.clone());

    let injections_query = match language.as_str() {
        "elixir" | "ex" | "exs" => tree_sitter_elixir::INJECTIONS_QUERY,
        "fsharp" | "f#" | "fs" | "fsx" | "fsi" => tree_sitter_fsharp::INJECTIONS_QUERY,
        "haskell" | "hs" => tree_sitter_haskell::INJECTIONS_QUERY,
        "html" => tree_sitter_html::INJECTIONS_QUERY,
        "javascript" | "js" | "jsx" => tree_sitter_javascript::INJECTIONS_QUERY,
        "lua" => tree_sitter_lua::INJECTIONS_QUERY,
        "luau" => tree_sitter_luau::INJECTIONS_QUERY,
        "objc" | "objective-c" => tree_sitter_objc::INJECTIONS_QUERY,
        "odin" => tree_sitter_odin::INJECTIONS_QUERY,
        "php" | "php_only" => tree_sitter_php::INJECTIONS_QUERY,
        "rust" | "rs" => tree_sitter_rust::INJECTIONS_QUERY,
        "svelte" => tree_sitter_svelte_ng::INJECTIONS_QUERY,
        "swift" => tree_sitter_swift::INJECTIONS_QUERY,
        "zig" => tree_sitter_zig::INJECTIONS_QUERY,
        _ => "",
    };

    let locals_query = match language.as_str() {
        "fsharp" | "f#" | "fs" | "fsx" | "fsi" => tree_sitter_fsharp::LOCALS_QUERY,
        "haskell" | "hs" => tree_sitter_haskell::LOCALS_QUERY,
        "javascript" | "js" | "jsx" => tree_sitter_javascript::LOCALS_QUERY,
        "lua" => tree_sitter_lua::LOCALS_QUERY,
        "luau" => tree_sitter_luau::LOCALS_QUERY,
        "objc" | "objective-c" => tree_sitter_objc::LOCALS_QUERY,
        "ocaml" | "ml" | "ocaml_interface" | "ocaml_type" => tree_sitter_ocaml::LOCALS_QUERY,
        "odin" => tree_sitter_odin::LOCALS_QUERY,
        "r" => tree_sitter_r::LOCALS_QUERY,
        "ruby" | "rb" => tree_sitter_ruby::LOCALS_QUERY,
        "scala" => tree_sitter_scala::LOCALS_QUERY,
        "svelte" => tree_sitter_svelte_ng::LOCALS_QUERY,
        "swift" => tree_sitter_swift::LOCALS_QUERY,
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
