import Parser from "tree-sitter";
import { Languages } from "./languages.ts";
import { type Token, type BundledLanguage, bundledLanguages } from "./types.ts";

function walkTree(
  node: Parser.SyntaxNode,
  language: keyof typeof Languages,
): Token[] {
  const tokens: Token[] = [];

  for (const child of node.children) {
    if (child.childCount === 0) {
      tokens.push({
        value: child.text,
        type: Languages[language].nodeTypeInfo.find(
          (t) => t.type === child.type,
        )?.type,
      });
    }
    tokens.push(...walkTree(child, language));
  }

  return tokens;
}

function highlightCode(
  sourceCode: string,
  language: BundledLanguage = "plaintext",
) {
  if (!bundledLanguages.includes(language)) {
    throw new Error(`Language "${language}" is not supported.`);
  }

  if (
    language === "plaintext" ||
    language === "plain" ||
    language === "text" ||
    language === "txt"
  ) {
    return sourceCode;
  }

  const parser = new Parser();
  parser.setLanguage(Languages[language]);
  const tree = parser.parse(sourceCode);
  return walkTree(tree.rootNode, language);
}

const sourceCode = `
const a = 2;
if (a%2 === 0) {
  console.log(a);
  return <h2>text</h2>
}
`;

console.log(highlightCode(sourceCode));
