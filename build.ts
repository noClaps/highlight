import { $ } from "bun";

Bun.build({
  entrypoints: ["highlight.ts"],
  target: "node",
  outdir: "dist",
  external: [
    "@noclaps/highlight-darwin-arm64",
    "@noclaps/highlight-darwin-x64",
    "@noclaps/highlight-linux-arm64-gnu",
    "@noclaps/highlight-linux-arm64-musl",
    "@noclaps/highlight-linux-x64-gnu",
    "@noclaps/highlight-linux-x64-musl",
    "@noclaps/highlight-win32-arm64-msvc",
    "@noclaps/highlight-win32-x64-msvc",
  ],
});

await $`tsc --project tsconfig.json`;
