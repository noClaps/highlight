# Highlight

A syntax highlighting library that uses [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for incredibly quick parsing and highlighting.

## Usage

Add it as a dependency to your project:

```sh
cargo add --git https://gitlab.com/noClaps/highlight.git
```

Then you can use it in your code:

```rs
use highlight::{highlight, Theme};

fn main() {
    let theme = match Theme::new(include_str!("path/to/theme.toml")) {
        Ok(theme) => theme,
        Err(err) => panic!("Handle error here: {err}")
    };

    // You can use any language supported by Highlight.
    let code = include_str!("path/to/code.rs").to_string();
    let language = "rs".to_string();

    let highlighted_html = highlight(code, language, theme);
    println!("{highlighted_html}"); // <pre class="ts-highlight" ... </pre>
}
```
