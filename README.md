# Highlight

A syntax highlighting library that uses [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for incredibly quick parsing and highlighting.

**NOTE**: There are probably bugs in the outputs of some of the languages. This is because I'm not really familiar with all the different languages and my fixes were best-effort, or I simply missed it in my testing. If you spot a bug in the language you want to use, you can open an issue and I'll try to fix it, or you can open a merge request with the fix yourself.

## Usage

Install the package as a dependency to your project:

```sh
# Use the command for your package manager
npx jsr add @noclaps/highlight
yarn dlx jsr add @noclaps/highlight
pnpm dlx jsr add @noclaps/highlight
bunx jsr add @noclaps/highlight
deno add jsr:@noclaps/highlight
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
