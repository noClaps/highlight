const completedPaths: string[] = [];

const treeSitterJSONs = new Bun.Glob("*/tree-sitter.json").scanSync(
  "node_modules",
);
for (const treeSitterJSON of treeSitterJSONs) {
  const json: {
    grammars: {
      name: string;
      highlights: string | string[];
    }[];
  } = JSON.parse(await Bun.file(`node_modules/${treeSitterJSON}`).text());
  const { grammars } = json;

  const path = treeSitterJSON.split("/")[0];
  completedPaths.push(path);

  for (const grammar of grammars) {
    let highlightsQuery = "";
    if (!grammar.highlights) {
      if (
        await Bun.file(`node_modules/${path}/queries/highlights.scm`).exists()
      ) {
        highlightsQuery += await Bun.file(
          `node_modules/${path}/queries/highlights.scm`,
        ).text();
      }
    } else if (typeof grammar.highlights === "string") {
      highlightsQuery += await Bun.file(
        `node_modules/${path}/${grammar.highlights}`,
      ).text();
    } else {
      for (const highlight of grammar.highlights) {
        highlightsQuery += await Bun.file(
          highlight.startsWith("node_modules")
            ? highlight
            : `node_modules/${path}/${highlight}`,
        ).text();
        highlightsQuery += "\n";
      }
    }

    Bun.write(`highlights/${grammar.name}.scm`, highlightsQuery);
  }
}

const schemes = new Bun.Glob("**/highlights.scm").scanSync("node_modules");

for (const scheme of schemes) {
  const path = scheme.split("/")[0];
  if (completedPaths.includes(path)) continue;

  const language = path.replace("tree-sitter-", "");
  Bun.write(`highlights/${language}.scm`, Bun.file(`node_modules/${scheme}`));
}
