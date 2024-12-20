import { highlight } from "./src/highlight.ts";
import type { Theme } from "./src/types.ts";

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

const lang = "c";

const code = await Bun.file(`test/test.${lang}`).text();
const highlightedCode = highlight(code, lang, theme);
Bun.write("out.html", highlightedCode);
