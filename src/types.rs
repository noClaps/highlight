use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Highlight {
    pub color: Option<String>,
    pub font_weight: Option<u16>,
    pub font_style: Option<String>,
    pub background_color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineNumbers {
    pub color: String,
    pub right_space: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Theme {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub line_numbers: Option<LineNumbers>,
    pub highlights: HashMap<String, Highlight>,
}

impl Theme {
    pub fn new(theme: String) -> Result<Theme, toml::de::Error> {
        toml::from_str::<Theme>(theme.as_str())
    }

    pub fn blank() -> Theme {
        Theme::new("[highlights]".to_string()).unwrap()
    }
}

#[derive(Clone)]
pub enum Language {
    Agda,
    C,
    Cpp,
    Css,
    Go,
    Haskell,
    Html,
    Java,
    Javascript,
    Jsx,
    Jsdoc,
    Json,
    Ocaml,
    OcamlInterface,
    OcamlType,
    Php,
    PhpOnly,
    Python,
    Regexp,
    Ruby,
    Rust,
    Scala,
    Shellscript,
    Typescript,
    Tsx,
}

impl Language {
    pub fn from_string(lang: &String) -> Option<Language> {
        match lang.as_str() {
            "agda" => Some(Self::Agda),
            "c" => Some(Self::C),
            "cpp" | "c++" => Some(Self::Cpp),
            "css" => Some(Self::Css),
            "go" => Some(Self::Go),
            "haskell" | "hs" => Some(Self::Haskell),
            "html" => Some(Self::Html),
            "java" => Some(Self::Java),
            "javascript" | "js" => Some(Self::Javascript),
            "jsx" => Some(Self::Jsx),
            "jsdoc" => Some(Self::Jsdoc),
            "json" => Some(Self::Json),
            "ocaml" => Some(Self::Ocaml),
            "ocaml_interface" => Some(Self::OcamlInterface),
            "ocaml_type" => Some(Self::OcamlType),
            "php" => Some(Self::Php),
            "php_only" => Some(Self::PhpOnly),
            "python" | "py" => Some(Self::Python),
            "regexp" | "regex" => Some(Self::Regexp),
            "ruby" | "rb" => Some(Self::Ruby),
            "rust" | "rs" => Some(Self::Rust),
            "scala" => Some(Self::Scala),
            "shellscript" | "shell" | "bash" | "zsh" | "sh" => Some(Self::Shellscript),
            "typescript" | "ts" => Some(Self::Typescript),
            "tsx" => Some(Self::Tsx),
            _ => None,
        }
    }

    pub fn to_string(self) -> String {
        match self {
            Self::Agda => "agda",
            Self::C => "c",
            Self::Cpp => "cpp",
            Self::Css => "css",
            Self::Go => "go",
            Self::Haskell => "haskell",
            Self::Html => "html",
            Self::Java => "java",
            Self::Javascript => "javascript",
            Self::Jsx => "jsx",
            Self::Jsdoc => "jsdoc",
            Self::Json => "json",
            Self::Ocaml => "ocaml",
            Self::OcamlInterface => "ocaml_interface",
            Self::OcamlType => "ocaml_type",
            Self::Php => "php",
            Self::PhpOnly => "php_only",
            Self::Python => "python",
            Self::Regexp => "regexp",
            Self::Ruby => "ruby",
            Self::Rust => "rust",
            Self::Scala => "scala",
            Self::Shellscript => "bash",
            Self::Typescript => "typescript",
            Self::Tsx => "tsx",
        }
        .to_string()
    }
}
