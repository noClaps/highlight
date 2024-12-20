import { highlight } from "./src/highlight.ts";
import type { Theme, BundledLanguage } from "./src/types.ts";
import { bundledLanguages } from "./src/languages";

export {
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
  highlight,

  /** A list of the languages supported by Highlight. */
  bundledLanguages,

  /** The type of theme accepted by the Highlight package. */
  type Theme,

  /** The type of bundled language used by the Highlight package */
  type BundledLanguage,
};
