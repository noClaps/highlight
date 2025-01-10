# Highlight

A syntax highlighting library that uses [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for incredibly quick parsing and highlighting.

## Usage

Install the package as a dependency to your project:

```sh
bun add @noclaps/highlight
```

and then import it into your code:

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
