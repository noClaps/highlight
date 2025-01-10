import packageJSON from "./package.json";

const version = packageJSON.version;
const newPackageJSON = packageJSON;

for (const dep in packageJSON.optionalDependencies) {
  // @ts-ignore I'm looping through the deps, of course it's gonna be valid keys
  newPackageJSON.optionalDependencies[dep] = version;
}

Bun.write("./package.json", JSON.stringify(newPackageJSON));
