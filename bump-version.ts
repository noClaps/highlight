import packageJSON from "./package.json";

const version = packageJSON.version;

const packages = new Bun.Glob("**/package.json").scanSync("packages");
for (const pkg of packages) {
  const json = await Bun.file(`packages/${pkg}`).json();
  json.version = version;
  Bun.write(`packages/${pkg}`, JSON.stringify(json));
}
