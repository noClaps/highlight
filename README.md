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
    let theme = match Theme::new(include_str!("path/to/theme.toml").to_string()) {
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

You can use `Theme::new()` to create a theme from a string, or `Theme::blank()` to create a theme with no syntax highlighting. See [Themes](#themes) for details on how to create a theme.

## Themes

A theme is a TOML file with the following properties:

- `fg`: The default text color, optional. This is set on the parent `<pre>` element and is used if no valid highlight is present, or no highlight color is provided for that syntax.

  ```toml
  fg = "#fff"
  # fg = <color-value>
  ```

- `bg`: The default background color, optional. This is the background color of the code block.

  ```toml
  bg = "#111"
  # bg = <color-value>
  ```

- `line_numbers`: An object to configure line numbers, optional. If it is left out, no line numbers will be present. The properties of this object are:
  - `color`: The color of the line numbers, required.

    ```toml
    [line_numbers]
    color = "#888"
    # color = <color-value>
    ```

  - `right_space`: The number of spaces between the line numbers and the code, in units of `ch`, optional. The default value is 1.

    ```toml
    [line_numbers]
    right_space = 2
    # right_space = <number>
    ```

- `highlights`: An object to configure highlights, required. This is a map, with the keys being the syntax types, and the values being the configuration object. If you don't want to have inline styles, you can have the keys be the syntax types you want to select, and the configuration object empty for each one. The properties of the configuration object are:
  - `color`: The text color of the syntax type, optional.

    ```toml
    [highlights]
    type = { color = "#5ac8f5" }
    # <syntax_name> = { color = "<color-value>" }
    ```

  - `font_weight`: The font weight for the syntax type, optional. This should be a number between 1 and 1000.

    ```toml
    [highlights]
    type = { font_weight = 500 }
    # <syntax_name> = { font_weight = <font-weight> }
    ```

  - `font_style`: The font style for the syntax type, optional. One of "italic", "normal" (default), or "oblique".

    ```toml
    [highlights]
    type = { font_style = "italic" }
    # <syntax_name> = { font_style = <"italic" | "normal" | "oblique"> }
    ```

  - `background_color`: The background color for the syntax type, optional.

    ```toml
    [highlights]
    type = { background_color = "#111" }
    # <syntax_name> = { background_color = <color-value> }
    ```

All of the color values are CSS colors, so you can use hex (`#rrggbbaa`), OKLCH (`oklch(lightness% chroma hue / alpha)`), etc.

You can look at [`theme.toml`](./theme.toml) for an example theme.
