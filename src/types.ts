import { languages } from "./languages.ts";

export interface Token {
  value: string;
  type: string;
}

export type TSLanguage = keyof typeof languages;

export type BundledLanguage =
  | TSLanguage
  | "plaintext"
  | "plain"
  | "text"
  | "txt";

export type Theme = {
  fg: string;
  bg: string;
  highlights: Record<
    string,
    {
      color?: string;
      fontWeight?: number;
      fontStyle?: "italic" | "normal" | "oblique";
      backgroundColor?: string;
    }
  >;
};
