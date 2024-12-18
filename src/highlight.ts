import Parser from "tree-sitter";
import { languages, bundledLanguages, queries } from "./languages.ts";
import type { BundledLanguage, Theme, Token } from "./types.ts";
import fs from "node:fs";

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
  theme: Theme,
  returnVal: "tokens",
): Token[];
export function highlight(
  code: string,
  language: BundledLanguage,
  theme: Theme,
  returnVal?: "code",
): string;
export function highlight(
  code: string,
  language: BundledLanguage,
  theme?: Theme,
  returnVal: "code" | "tokens" = "code",
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
    return `<pre>${code}</pre>`;
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

  function findCaptures(node: Parser.SyntaxNode) {
    const matches = query.matches(node);
    for (const match of matches) {
      for (const capture of match.captures) {
        if (capture.node === node) return capture.name;
      }
    }
  }

  function traverseNode(node: Parser.SyntaxNode) {
    const highlightType = findCaptures(node) || node.type;

    if (node.childCount === 0) {
      if (node.startIndex > lastEndIndex && lastEndIndex !== 0) {
        tokens.push({
          value: code.slice(lastEndIndex, node.startIndex),
          type: "whitespace",
        });
      }

      tokens.push({
        value: node.text,
        type: highlightType,
      });
      lastEndIndex = node.endIndex;
    }

    for (let child of node.children) {
      traverseNode(child);
    }
  }

  traverseNode(rootNode);

  if (returnVal === "tokens") {
    return tokens;
  }

  let highlightedCode = "";
  for (const token of tokens) {
    const tokenStyle = theme?.highlights[token.type];
    highlightedCode += `<span class="${token.type}" ${tokenStyle ? ` style="${tokenStyle.backgroundColor ? `background-color:${tokenStyle.backgroundColor};` : ""}${tokenStyle.color ? `color:${tokenStyle.color};` : ""}${tokenStyle.fontStyle ? `font-style:${tokenStyle.fontStyle};` : ""}${tokenStyle.fontWeight ? `font-weight:${tokenStyle.fontWeight}` : ""}"` : ""}>${escapeHTML(token.value)}</span>`;
  }

  return `<pre${theme && (theme.bg || theme.fg) ? ` style="${theme.bg ? `background-color:${theme.bg};` : ""}${theme.fg ? `color:${theme.fg};` : ""}"` : ""}>${highlightedCode}</pre>`;
}
