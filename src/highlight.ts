import Parser from "tree-sitter";
import { languages, bundledLanguages, queries } from "./languages.ts";
import type { BundledLanguage, Theme, Token } from "./types.ts";
import fs from "node:fs";
import {
  bashCorrections,
  cCorrections,
  cppCorrections,
  cSharpCorrections,
  cssCorrections,
  goCorrections,
  htmlCorrections,
  javaCorrections,
  jsCorrections,
  ocamlCorrections,
  phpCorrections,
  pythonCorrections,
  regexCorrections,
  rubyCorrections,
  rustCorrections,
  scalaCorrections,
  tsCorrections,
  tsxCorrections,
} from "./corrections.ts";

function typeCorrections(
  node: Parser.SyntaxNode,
  captures: string[],
  language: BundledLanguage,
): string | undefined {
  switch (language) {
    case "bash":
    case "shellscript":
    case "sh": {
      const c = bashCorrections(node);
      if (c !== undefined) return c;
      break;
    }

    case "c": {
      const c = cCorrections(node, captures);
      if (c !== undefined) return c;
      break;
    }

    case "c#":
    case "cs":
    case "csharp": {
      const c = cSharpCorrections(node);
      if (c !== undefined) return c;
      break;
    }

    case "cpp":
    case "c++": {
      const c = cppCorrections(node, captures);
      if (c !== undefined) return c;
      break;
    }

    case "css": {
      const c = cssCorrections(node);
      if (c !== undefined) return c;
      break;
    }

    case "go": {
      const c = goCorrections(node);
      if (c !== undefined) return c;
      break;
    }

    case "html": {
      const c = htmlCorrections(node);
      if (c !== undefined) return c;
      break;
    }

    case "java": {
      const c = javaCorrections(node);
      if (c !== undefined) return c;
      break;
    }

    case "js":
    case "jsx":
    case "javascript": {
      const c = jsCorrections(node, captures);
      if (c !== undefined) return c;
      break;
    }

    case "ml":
    case "ocaml":
    case "ocaml_type":
    case "ocaml_interface": {
      const c = ocamlCorrections(node);
      if (c !== undefined) return c;
      break;
    }

    case "php_only":
    case "php": {
      const c = phpCorrections(node, captures);
      if (c !== undefined) return c;

      break;
    }

    case "py":
    case "python": {
      const c = pythonCorrections(node, captures);
      if (c !== undefined) return c;

      break;
    }

    case "regex":
    case "regexp": {
      const c = regexCorrections(node);
      if (c !== undefined) return c;

      break;
    }

    case "rb":
    case "ruby": {
      const c = rubyCorrections(node, captures);
      if (c !== undefined) return c;

      break;
    }

    case "rs":
    case "rust": {
      const c = rustCorrections(node, captures);
      if (c !== undefined) return c;

      break;
    }

    case "scala": {
      const c = scalaCorrections(node, captures);
      if (c !== undefined) return c;

      break;
    }

    case "ts":
    case "typescript": {
      const c = tsCorrections(node, captures);
      if (c !== undefined) return c;

      break;
    }

    case "tsx": {
      const c = tsxCorrections(node, captures);
      if (c !== undefined) return c;

      break;
    }
  }

  if (
    node.type === "identifier" &&
    (node.parent?.type.includes("function") ||
      node.parent?.type.includes("call"))
  ) {
    return "function";
  }
}

function escapeHTML(input: string): string {
  return input
    .replaceAll(`&`, `&amp;`)
    .replaceAll(`"`, `&quot;`)
    .replaceAll(`'`, `&#x27;`)
    .replaceAll(`<`, `&lt;`)
    .replaceAll(`>`, `&gt;`);
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
    throw new Error(`Language "${language}" is not supported`);
  }

  if (
    language === "plain" ||
    language === "plaintext" ||
    language === "text" ||
    language === "txt"
  ) {
    return `<pre>${escapeHTML(code)}</pre>`;
  }

  const parser = new Parser();

  parser.setLanguage(languages[language]);
  const rootNode = parser.parse(code).rootNode;
  const query = new Parser.Query(
    languages[language],
    fs.readFileSync(`highlights/${queries[language]}.scm`),
  );
  const tokens: Token[] = [];
  let lastEndIndex = 0;

  function traverseNode(node: Parser.SyntaxNode) {
    if (node.childCount === 0) {
      const captures = query.captures(node).map((c) => c.name);

      if (node.startIndex > lastEndIndex && lastEndIndex !== 0) {
        const codeSection = code.slice(lastEndIndex, node.startIndex);
        if (!codeSection.trim()) {
          tokens.push({
            value: codeSection,
            type: "whitespace",
          });
        } else {
          let type = "";
          if (language === "css") {
            if (!isNaN(Number(codeSection))) type = "number";
            if (node.previousSibling?.type === "color_value") {
              type = "string.special";
            }
          } else if (["rust", "rs"].includes(language)) {
            let currentNode = node;
            for (
              let lim = 0;
              node.parent?.type !== "line_comment" && node.parent && lim < 5;
              lim++
            ) {
              currentNode = node.parent;
            }
            if (currentNode) {
              type = "comment";
            }
          }

          tokens.push({
            value: codeSection,
            type,
          });
        }
      }

      let type = "";
      if (captures.length > 0) {
        type = captures[0];
      } else {
        if (node.parent && node.parent.childCount > 0) {
          const parentCaptures = query.captures(node.parent).map((c) => c.name);
          if (parentCaptures.length > 0) {
            type = parentCaptures[0];
          }
        }
      }

      type = typeCorrections(node, captures, language) ?? type;

      tokens.push({
        value: node.text,
        type,
      });
      lastEndIndex = node.endIndex;
    }

    for (let child of node.children) {
      traverseNode(child);
    }
  }

  traverseNode(rootNode);

  let highlightedCode = "";
  for (const token of tokens) {
    const tokenStyle = theme?.highlights[token.type];
    highlightedCode += `<span class="${token.type}"${tokenStyle ? ` style="${tokenStyle.backgroundColor ? `background-color:${tokenStyle.backgroundColor};` : ""}${tokenStyle.color ? `color:${tokenStyle.color};` : ""}${tokenStyle.fontStyle ? `font-style:${tokenStyle.fontStyle};` : ""}${tokenStyle.fontWeight ? `font-weight:${tokenStyle.fontWeight}` : ""}"` : ""}>${escapeHTML(token.value)}</span>`;
  }

  return `<pre${theme && (theme.bg || theme.fg) ? ` style="${theme.bg ? `background-color:${theme.bg};` : ""}${theme.fg ? `color:${theme.fg};` : ""}"` : ""}><code>${highlightedCode}</code></pre>`;
}
