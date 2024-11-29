import { Languages } from "./languages.ts";

export interface Token {
  value: string;
  type?: string;
}

export type BundledLanguage =
  | keyof typeof Languages
  | "plaintext"
  | "plain"
  | "text"
  | "txt";
export const bundledLanguages = [
  ...Object.keys(Languages),
  "plaintext",
  "plain",
  "text",
  "txt",
];
