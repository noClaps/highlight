import { highlight as tsHighlight } from "./dist/index.js";

/** A list of the languages supported by Highlight. */
const bundledLanguages = [
  "c",
  "codeql",
  "ql",
  "cpp",
  "c++",
  "csharp",
  "c#",
  "cs",
  "css",
  "ejs",
  "erb",
  "go",
  "golang",
  "haskell",
  "hs",
  "html",
  "java",
  "javascript",
  "js",
  "jsx",
  "jsdoc",
  "json",
  "julia",
  "jl",
  "ml",
  "ocaml",
  "ocaml_interface",
  "ocaml_type",
  "php",
  "php_only",
  "python",
  "py",
  "regex",
  "regexp",
  "ruby",
  "rb",
  "rust",
  "rs",
  "scala",
  "shellscript",
  "shell",
  "bash",
  "sh",
  "tsx",
  "typescript",
  "ts",
  "verilog",
  "plaintext",
  "plain",
  "text",
  "txt",
] as const;

/** The type of bundled language used by the Highlight package */
type BundledLanguage = (typeof bundledLanguages)[number];

/** The type of theme accepted by the Highlight package. */
interface Theme {
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

const theme: Theme = {
  fg: "#fff",
  bg: "#111",
  highlights: {
    attribute: {
      color: "#ac8e68",
      fontWeight: 500,
    },
    boolean: {
      color: "#ff375f",
      fontWeight: 700,
    },
    comment: {
      color: "#98989d",
      fontWeight: 500,
    },
    "comment.doc": {
      color: "#98989d",
    },
    constant: {
      color: "#6ac4dc",
      fontWeight: 500,
    },
    constructor: {
      color: "#66d4cf",
      fontWeight: 500,
    },
    enum: {
      color: "#66d4cf",
      fontWeight: 500,
    },
    emphasis: {
      color: "#5ac8f5",
      fontStyle: "italic",
    },
    "emphasis.strong": {
      color: "#5ac8f5",
      fontWeight: 700,
    },
    function: {
      color: "#66d4cf",
      fontWeight: 500,
    },
    keyword: {
      color: "#ff375f",
      fontWeight: 700,
    },
    label: {
      color: "#98989d",
      fontWeight: 700,
    },
    "link.text": {
      color: "#0a84ff",
      fontWeight: 500,
    },
    "link.uri": {
      color: "#0a84ff",
      fontWeight: 500,
    },
    link_text: {
      color: "#ff9f0a",
      fontWeight: 500,
    },
    link_uri: {
      color: "#0a84ff",
      fontWeight: 500,
    },
    number: {
      color: "#ffd60a",
      fontWeight: 500,
    },
    operator: {
      color: "#98989d",
      fontWeight: 500,
    },
    preproc: {
      color: "#ff9f0a",
      fontWeight: 500,
    },
    property: {
      color: "#6ac4dc",
      fontWeight: 500,
    },
    "punctuation.bracket": {
      color: "#98989d",
      fontWeight: 500,
    },
    "punctuation.list_marker": {
      color: "#ac8e68",
      fontWeight: 500,
    },
    string: {
      color: "#ff9f0a",
      fontWeight: 500,
    },
    "string.special": {
      color: "#ff9f0a",
      fontWeight: 500,
    },
    "string.special.symbol": {
      color: "#ffd60a",
      fontWeight: 500,
    },
    tag: {
      color: "#ff375f",
      fontWeight: 700,
    },
    "text.literal": {
      color: "#ff9f0a",
      fontWeight: 500,
    },
    title: {
      color: "#98989d",
      fontWeight: 700,
    },
    type: {
      color: "#5ac8f5",
      fontWeight: 500,
    },
    "type.builtin": {
      color: "#5ac8f5",
      fontWeight: 500,
    },
    variable: {
      color: "#fff",
      fontWeight: 500,
    },
  },
};

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
) {
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

console.log(highlight("int a = 5;", "c"));
