import Parser from "tree-sitter";
import { languages, bundledLanguages, queries } from "./languages.ts";
import type { BundledLanguage, Theme, Token } from "./types.ts";
import fs from "node:fs";

function typeCorrections(node: Parser.SyntaxNode, language: BundledLanguage) {
  switch (language) {
    case "bash":
    case "shellscript":
    case "sh": {
      if (node.type === "word" && node.text.startsWith("-")) return "constant";
      if (node.type === "word" && node.parent?.type === "command_name") return;
      if (node.type === "word") return "";

      if (
        node.text === "=" &&
        node.parent &&
        ["variable_assignment", "binary_expression"].includes(node.parent.type)
      )
        return "operator";
      break;
    }

    case "c": {
      break;
    }
  }

  // if (node.type === "shorthand_property_identifier") {
  //   return "property";
  // }

  // if (
  //   node.type === "identifier" &&
  //   (node.parent?.type.includes("function") ||
  //     node.parent?.type.includes("call"))
  // ) {
  //   return "function";
  // }

  // if (node.type === "identifier" && node.parent?.type === "type") {
  //   return "type";
  // }

  // if (
  //   node.type === "property_identifier" &&
  //   node.parent?.parent?.type.includes("call")
  // ) {
  //   return "function";
  // }

  // if (
  //   node.type === "property_identifier" &&
  //   node.parent?.parent?.type.includes("new")
  // ) {
  //   return "constructor";
  // }

  // if (node.type === ":") {
  //   return node.type;
  // }

  // if (
  //   node.parent?.type === "template_substitution" &&
  //   node.text === node.type
  // ) {
  //   return "punctuation.special";
  // }

  if ("()[]{}".includes(node.type)) return "punctuation.bracket";
  if (!isNaN(Number(node.text))) return "number";
}

function escapeHTML(input: string): string {
  return input
    .replaceAll(`&`, `&amp;`)
    .replaceAll(`"`, `&quot;`)
    .replaceAll(`'`, `&#x27;`)
    .replaceAll(`<`, `&lt;`)
    .replaceAll(`>`, `&gt;`);
}

export function highlight(
  code: string,
  language: BundledLanguage,
  theme?: Theme,
) {
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
        tokens.push({
          value: code.slice(lastEndIndex, node.startIndex),
          type: "whitespace",
        });
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

      type = typeCorrections(node, language) ?? type;

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

  return `<pre${theme && (theme.bg || theme.fg) ? ` style="${theme.bg ? `background-color:${theme.bg};` : ""}${theme.fg ? `color:${theme.fg};` : ""}"` : ""}>${highlightedCode}</pre>`;
}
