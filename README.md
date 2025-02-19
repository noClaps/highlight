# Highlight

A syntax highlighting CLI tool that uses [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for incredibly quick parsing and highlighting.

## Build instructions

You'll need Rust to build this project.

1. Clone the repository.

   ```sh
   git clone https://gitlab.com/noClaps/highlight.git
   cd highlight
   ```

2. Build the project.

   ```sh
   make
   ```

   You can then run it using `./highlight`.

## Usage

```
Usage: highlight [OPTIONS] [CODE]

Arguments:
  [CODE]  The code to syntax highlight

Options:
  -t, --theme <THEME>  The theme file to highlight with, required if code is passed in
  -l <LANGUAGE>        The language that the code is on, required if code is passed in
      --list-langs     List the languages supported by Highlight, values separated by `|` are alternate names for
                       the same language
  -h, --help           Print help
  -V, --version        Print version
```

Create a TOML file with your theme. See [Themes](#themes) for details on how to do this.

Then, you can run

```sh
highlight [your code] --theme path/to/theme.toml -l [language]
```

and the syntax highlighted HTML code will be written to `stdout`. If you want to write it to a file, you can use the `>` operator. For example,

```sh
highlight "const a = 1" --theme ./theme.toml -l js > out.html
```

will highlight `const a = 1` and write the output to `out.html`.

You can list the supported languages with:

```sh
highlight --list-langs
```

You can see the help menu with:

```sh
highlight --help
```
