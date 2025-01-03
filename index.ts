export { highlight, type Theme } from "./dist/index.js";

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
