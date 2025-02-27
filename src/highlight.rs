use std::error::Error;
use tree_sitter_highlight::{HighlightConfiguration, Highlighter, HtmlRenderer};

use crate::types::Language;

fn get_highlight_query(language: &Language) -> String {
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

    match language {
        Language::Agda => tree_sitter_agda::HIGHLIGHTS_QUERY.to_string(),
        Language::C => tree_sitter_c::HIGHLIGHT_QUERY.to_string(),
        Language::Cpp => cpp,
        Language::Css => tree_sitter_css::HIGHLIGHTS_QUERY.to_string(),
        Language::Go => tree_sitter_go::HIGHLIGHTS_QUERY.to_string(),
        Language::Haskell => tree_sitter_haskell::HIGHLIGHTS_QUERY.to_string(),
        Language::Html => tree_sitter_html::HIGHLIGHTS_QUERY.to_string(),
        Language::Java => tree_sitter_java::HIGHLIGHTS_QUERY.to_string(),
        Language::Javascript => tree_sitter_javascript::HIGHLIGHT_QUERY.to_string(),
        Language::Jsx => jsx,
        Language::Jsdoc => tree_sitter_jsdoc::HIGHLIGHTS_QUERY.to_string(),
        Language::Json => tree_sitter_json::HIGHLIGHTS_QUERY.to_string(),
        Language::Ocaml | Language::OcamlInterface | Language::OcamlType => {
            tree_sitter_ocaml::HIGHLIGHTS_QUERY.to_string()
        }
        Language::Php | Language::PhpOnly => tree_sitter_php::HIGHLIGHTS_QUERY.to_string(),
        Language::Python => tree_sitter_python::HIGHLIGHTS_QUERY.to_string(),
        Language::Regexp => tree_sitter_regex::HIGHLIGHTS_QUERY.to_string(),
        Language::Ruby => tree_sitter_ruby::HIGHLIGHTS_QUERY.to_string(),
        Language::Rust => tree_sitter_rust::HIGHLIGHTS_QUERY.to_string(),
        Language::Scala => tree_sitter_scala::HIGHLIGHTS_QUERY.to_string(),
        Language::Shellscript => tree_sitter_bash::HIGHLIGHT_QUERY.to_string(),
        Language::Typescript => ts,
        Language::Tsx => tsx,
    }
}

fn get_locals_query(language: &Language) -> String {
    let ts = format!(
        "{}\n{}",
        tree_sitter_typescript::LOCALS_QUERY,
        tree_sitter_javascript::LOCALS_QUERY
    );
    let tsx = tree_sitter_javascript::LOCALS_QUERY;

    match language {
        Language::Haskell => tree_sitter_haskell::LOCALS_QUERY,
        Language::Javascript | Language::Jsx => tree_sitter_javascript::LOCALS_QUERY,
        Language::Ocaml | Language::OcamlInterface | Language::OcamlType => {
            tree_sitter_ocaml::LOCALS_QUERY
        }
        Language::Ruby => tree_sitter_ruby::LOCALS_QUERY,
        Language::Scala => tree_sitter_scala::LOCALS_QUERY,
        Language::Typescript => ts.as_str(),
        Language::Tsx => tsx,
        _ => "",
    }
    .to_string()
}

fn get_injections_query(language: &Language) -> String {
    let ts = tree_sitter_javascript::INJECTIONS_QUERY;

    match language {
        Language::Haskell => tree_sitter_haskell::INJECTIONS_QUERY,
        Language::Html => tree_sitter_html::INJECTIONS_QUERY,
        Language::Javascript | Language::Jsx => tree_sitter_javascript::INJECTIONS_QUERY,
        Language::Php | Language::PhpOnly => tree_sitter_php::INJECTIONS_QUERY,
        Language::Rust => tree_sitter_rust::INJECTIONS_QUERY,
        Language::Typescript | Language::Tsx => ts,
        _ => "",
    }
    .to_string()
}

fn get_language(language: Language) -> Result<HighlightConfiguration, Box<dyn Error>> {
    let ts_lang = match language {
        Language::Agda => tree_sitter_agda::LANGUAGE,
        Language::C => tree_sitter_c::LANGUAGE,
        Language::Cpp => tree_sitter_cpp::LANGUAGE,
        Language::Css => tree_sitter_css::LANGUAGE,
        Language::Go => tree_sitter_go::LANGUAGE,
        Language::Haskell => tree_sitter_haskell::LANGUAGE,
        Language::Html => tree_sitter_html::LANGUAGE,
        Language::Java => tree_sitter_java::LANGUAGE,
        Language::Javascript | Language::Jsx => tree_sitter_javascript::LANGUAGE,
        Language::Jsdoc => tree_sitter_jsdoc::LANGUAGE,
        Language::Json => tree_sitter_json::LANGUAGE,
        Language::Ocaml => tree_sitter_ocaml::LANGUAGE_OCAML,
        Language::OcamlInterface => tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE,
        Language::OcamlType => tree_sitter_ocaml::LANGUAGE_OCAML_TYPE,
        Language::Php => tree_sitter_php::LANGUAGE_PHP,
        Language::PhpOnly => tree_sitter_php::LANGUAGE_PHP_ONLY,
        Language::Python => tree_sitter_python::LANGUAGE,
        Language::Regexp => tree_sitter_regex::LANGUAGE,
        Language::Ruby => tree_sitter_ruby::LANGUAGE,
        Language::Rust => tree_sitter_rust::LANGUAGE,
        Language::Scala => tree_sitter_scala::LANGUAGE,
        Language::Shellscript => tree_sitter_bash::LANGUAGE,
        Language::Typescript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
        Language::Tsx => tree_sitter_typescript::LANGUAGE_TSX,
    };

    let highlight_query = get_highlight_query(&language);
    let locals_query = get_locals_query(&language);
    let injections_query = get_injections_query(&language);

    Ok(HighlightConfiguration::new(
        ts_lang.into(),
        language.to_string(),
        highlight_query.as_str(),
        injections_query.as_str(),
        locals_query.as_str(),
    )
    .unwrap())
}

pub fn highlight_code(
    highlight_names: Vec<String>,
    language: Language,
    code: String,
) -> Result<String, Box<dyn Error>> {
    let mut highlighter = Highlighter::new();

    let mut config = match get_language(language) {
        Ok(config) => config,
        Err(err) => return Err(err),
    };
    config.configure(&highlight_names);

    let highlight_classes: Vec<String> = highlight_names
        .iter()
        .map(|name| format!("class=\"{}\"", name))
        .collect();

    let highlights = highlighter
        .highlight(&config, code.as_bytes(), None, |capture| {
            let capture_lang = match Language::from_string(&capture.to_string()) {
                Some(lang) => lang,
                None => return None,
            };
            let mut nested_config = match get_language(capture_lang) {
                Ok(config) => config,
                Err(err) => {
                    eprintln!("{}", err);
                    return None;
                }
            };
            nested_config.configure(&highlight_names);
            // Leak the new configuration so that it has a 'static lifetime.
            Some(Box::leak(Box::new(nested_config)))
        })
        .unwrap();

    let mut html_renderer = HtmlRenderer::new();
    let _ = html_renderer.render(highlights, code.as_bytes(), &move |h, output| {
        output.extend(highlight_classes[h.0].as_bytes());
    });

    Ok(html_renderer.lines().collect::<Vec<&str>>().join(""))
}
