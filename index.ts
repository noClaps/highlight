import { highlight as tsHighlight } from "./dist/index.js";

/** A list of the languages supported by Highlight. */
export const bundledLanguages = [
  // Agda
  "agda",
  // C
  "c",
  // C++
  "cpp",
  "c++",
  // CSS
  "css",
  // Go
  "go",
  "golang",
  // Haskell
  "haskell",
  "hs",
  // HTML
  "html",
  // Java
  "java",
  // JavaScript
  "javascript",
  "js",
  "jsx",
  // JSDoc
  "jsdoc",
  // JSON
  "json",
  // OCaml
  "ocaml",
  "ml",
  "ocaml_interface",
  "ocaml_type",
  // PHP
  "php",
  "php_only",
  // Python
  "python",
  "py",
  // Ruby
  "ruby",
  "rb",
  // Rust
  "rust",
  "rs",
  // Scala
  "scala",
  // Shell
  "shellscript",
  "shell",
  "bash",
  "zsh",
  "sh",
  // TypeScript
  "typescript",
  "ts",
  "tsx",
  // Plain
  "plaintext",
  "plain",
  "text",
  "txt",
] as const;

/** The type of bundled language used by the Highlight package */
export type BundledLanguage = (typeof bundledLanguages)[number];

/** The type of theme accepted by the Highlight package. */
export interface Theme {
  fg?: string;
  bg?: string;
  highlights?: Record<
    string,
    {
      color?: string;
      fontWeight?: number;
      fontStyle?: "italic" | "normal" | "oblique";
      backgroundColor?: string;
    }
  >;
}

/**
A function that takes in code and highlights it.

@param {string} code The code to highlight.
@param {BundledLanguage} language The programming language the code is written in, must be one of the languages supported by Highlight.
@param {Theme} [theme] A theme to syntax highlight with. There is no theme provided by default, so without one, no highlighting will be present.

@returns An HTML string with the syntax highlighted colors inlined in `style` attributes.

@example
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
*/
export function highlight(
  code: string,
  language: BundledLanguage,
  theme?: Theme,
): string {
  if (!bundledLanguages.includes(language)) {
    throw new Error(`Language ${language} is not supported by Highlight`);
  }

  let globalStyle = "";
  if (theme?.bg) globalStyle += `background-color:${theme.bg};`;
  if (theme?.fg) globalStyle += `color:${theme.fg};`;

  if (["plaintext", "plain", "text", "txt"].includes(language)) {
    return `<pre class="ts-highlight" style="${globalStyle}"><code>${code}</code></pre>`;
  }

  let highlightedText = tsHighlight(
    Object.keys(theme?.highlights ?? {}),
    language,
    code,
  );

  for (const key in theme?.highlights) {
    let style = "";
    if (theme.highlights[key].color)
      style += `color:${theme.highlights[key].color};`;
    if (theme.highlights[key].fontWeight)
      style += `font-weight:${theme.highlights[key].fontWeight};`;
    if (theme.highlights[key].fontStyle)
      style += `font-style:${theme.highlights[key].fontStyle};`;
    if (theme.highlights[key].backgroundColor)
      style += `background-color:${theme.highlights[key].backgroundColor};`;

    highlightedText = highlightedText.replaceAll(
      `<span class="${key}"`,
      `<span class="${key}" style="${style}"`,
    );
  }

  return `<pre class="ts-highlight" style="${globalStyle}"><code>${highlightedText}</code></pre>`;
}
