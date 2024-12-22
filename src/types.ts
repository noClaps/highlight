import { languages } from "./languages.ts";

export interface Token {
  value: string;
  type: string;
}

export type TSLanguage = keyof typeof languages;

/** The type of bundled language used by the Highlight package */
export type BundledLanguage =
  | TSLanguage
  | "plaintext"
  | "plain"
  | "text"
  | "txt";

/** The type of theme accepted by the Highlight package. */
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
