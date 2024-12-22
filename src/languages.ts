import C from "tree-sitter-c";
import Cpp from "tree-sitter-cpp";
import Csharp from "tree-sitter-c-sharp";
import Css from "tree-sitter-css";
import Glsl from "tree-sitter-glsl";
import Go from "tree-sitter-go";
import Haskell from "tree-sitter-haskell";
import Html from "tree-sitter-html";
import Java from "tree-sitter-java";
import JavaScript from "tree-sitter-javascript";
import Jsdoc from "tree-sitter-jsdoc";
import Json from "tree-sitter-json";
import Julia from "tree-sitter-julia";
import Ocaml from "tree-sitter-ocaml";
import Php from "tree-sitter-php";
import Python from "tree-sitter-python";
import Regex from "tree-sitter-regex";
import Ruby from "tree-sitter-ruby";
import Rust from "tree-sitter-rust";
import Scala from "tree-sitter-scala";
import Shell from "tree-sitter-bash";
import TypeScript from "tree-sitter-typescript";

export const languages = {
  // C
  c: C,
  // C++
  cpp: Cpp,
  "c++": Cpp,
  // C#
  csharp: Csharp,
  "c#": Csharp,
  cs: Csharp,
  // CSS
  css: Css,
  // GLSL
  glsl: Glsl,
  // Go
  go: Go,
  // Haskell
  haskell: Haskell,
  hs: Haskell,
  // HTML
  html: Html,
  // Java
  java: Java,
  // JavaScript
  javascript: JavaScript,
  js: JavaScript,
  jsx: JavaScript,
  // JSDoc
  jsdoc: Jsdoc,
  // JSON
  json: Json,
  // Julia
  julia: Julia,
  jl: Julia,
  // OCaml
  ml: Ocaml.ocaml,
  ocaml: Ocaml.ocaml,
  ocaml_interface: Ocaml.ocaml_interface,
  ocaml_type: Ocaml.ocaml_type,
  // PHP
  php: Php.php,
  php_only: Php.php_only,
  // Python
  python: Python,
  py: Python,
  // Regex
  regexp: Regex,
  regex: Regex,
  // Ruby
  ruby: Ruby,
  rb: Ruby,
  // Rust
  rust: Rust,
  rs: Rust,
  // Scala
  scala: Scala,
  // Shell
  shellscript: Shell,
  bash: Shell,
  sh: Shell,
  // TypeScript
  tsx: TypeScript.tsx,
  typescript: TypeScript.typescript,
  ts: TypeScript.typescript,
};

/** A list of the languages supported by Highlight. */
export const bundledLanguages: string[] = [
  ...Object.keys(languages),
  "plaintext",
  "plain",
  "text",
  "txt",
];

export const queries = {
  // C
  c: "c",
  // C++
  cpp: "cpp",
  "c++": "cpp",
  // C#
  csharp: "c-sharp",
  "c#": "c-sharp",
  cs: "c-sharp",
  // CSS
  css: "css",
  // GLSL
  glsl: "glsl",
  // Go
  go: "go",
  // Haskell
  haskell: "haskell",
  hs: "haskell",
  // HTML
  html: "html",
  // Java
  java: "java",
  // JavaScript
  javascript: "javascript",
  js: "javascript",
  jsx: "javascript",
  // JSDoc
  jsdoc: "jsdoc",
  // JSON
  json: "json",
  // Julia
  julia: "julia",
  jl: "julia",
  // OCaml
  ml: "ocaml",
  ocaml: "ocaml",
  ocaml_interface: "ocaml",
  ocaml_type: "ocaml",
  // PHP
  php: "php",
  php_only: "php",
  // Python
  python: "python",
  py: "python",
  // Regex
  regexp: "regex",
  regex: "regex",
  // Ruby
  ruby: "ruby",
  rb: "ruby",
  // Rust
  rust: "rust",
  rs: "rust",
  // Scala
  scala: "scala",
  // Shell
  shellscript: "bash",
  bash: "bash",
  sh: "bash",
  // TypeScript
  tsx: "tsx",
  typescript: "typescript",
  ts: "typescript",
};
