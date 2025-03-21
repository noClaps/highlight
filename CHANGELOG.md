# Changelog

## v0.7.0

[compare changes](https://github.com/noClaps/highlight/compare/v0.6.1...v0.7.0)

- Remove CLI. This is now only a Rust library.
- Add blank theme function. You can now create a blank theme with no syntax highlighting using `Theme::blank()`.

## v0.6.1

[compare changes](https://github.com/noClaps/highlight/compare/v0.6.0...v0.6.1)

### 🩹 Fixes

- Add PROJECT_NAME variable ([7824e89](https://github.com/noClaps/highlight/commit/7824e89))
- Add TAP_REPO variable ([4649deb](https://github.com/noClaps/highlight/commit/4649deb))
- Update project name ([6e18a6a](https://github.com/noClaps/highlight/commit/6e18a6a))
- Close `</pre>` element ([c356b70](https://github.com/noClaps/highlight/commit/c356b70))

### 📖 Documentation

- Update installation docs ([e5f484c](https://github.com/noClaps/highlight/commit/e5f484c))

### 🤖 CI

- Check downloaded file ([6faedf0](https://github.com/noClaps/highlight/commit/6faedf0))
- Remove ls and manual trigger ([07191d4](https://github.com/noClaps/highlight/commit/07191d4))
- Re-add manual trigger ([c496b34](https://github.com/noClaps/highlight/commit/c496b34))
- Move all build steps to homebrew-tap repo ([5bfec55](https://github.com/noClaps/highlight/commit/5bfec55))
- Use smaller image and fix commands ([059574a](https://github.com/noClaps/highlight/commit/059574a))

### ❤️ Contributors

- NoClaps <git@zerolimits.dev>

## v0.6.0

[compare changes](https://github.com/noClaps/highlight/compare/v0.5.2...v0.6.0)

### 🚀 Enhancements

- Implement Clone for Theme ([4afaef6](https://github.com/noClaps/highlight/commit/4afaef6))

### 🩹 Fixes

- Trim highlighted text ([8e5e941](https://github.com/noClaps/highlight/commit/8e5e941))

### 📦 Build

- Move CLI to a different crate in workspace ([7b440ed](https://github.com/noClaps/highlight/commit/7b440ed))

### 🏡 Chore

- Update dependencies ([f6d7237](https://github.com/noClaps/highlight/commit/f6d7237))

### 🤖 CI

- Add Homebrew tap pipeline ([b6f7a29](https://github.com/noClaps/highlight/commit/b6f7a29))
- Update GitLab CI file ([bc781ca](https://github.com/noClaps/highlight/commit/bc781ca))

### ❤️ Contributors

- NoClaps <git@zerolimits.dev>

## v0.5.2

[compare changes](https://github.com/noClaps/highlight/compare/v0.5.1...v0.5.2)

### 🚀 Enhancements

- Return error when encountering an unsupported language ([de63677](https://github.com/noClaps/highlight/commit/de63677))

### 🏡 Chore

- Update dependencies ([c722ce6](https://github.com/noClaps/highlight/commit/c722ce6))
- Move src/main.rs to src/bin/highlight.rs ([575700a](https://github.com/noClaps/highlight/commit/575700a))

### ❤️ Contributors

- NoClaps <git@zerolimits.dev>

## v0.5.1

[compare changes](https://github.com/noClaps/highlight/compare/v0.5.0...v0.5.1)

### 📦 Build

- Reduce dependencies for library ([95ceb12](https://github.com/noClaps/highlight/commit/95ceb12))

### 🏡 Chore

- Update dependencies ([68bd827](https://github.com/noClaps/highlight/commit/68bd827))

### ❤️ Contributors

- NoClaps <git@zerolimits.dev>

## v0.5.0

[compare changes](https://github.com/noClaps/highlight/compare/v0.4.0...v0.5.0)

### 🚀 Enhancements

- Make into a library as well ([92e304a](https://github.com/noClaps/highlight/commit/92e304a))
- ⚠️  Make CLI code argument a file input ([13cd54d](https://github.com/noClaps/highlight/commit/13cd54d))

### 📖 Documentation

- Add theme docs ([a1c250f](https://github.com/noClaps/highlight/commit/a1c250f))

### 🏡 Chore

- Update dependencies ([37650a7](https://github.com/noClaps/highlight/commit/37650a7))
- Add bench script to Makefile ([f660b14](https://github.com/noClaps/highlight/commit/f660b14))

#### ⚠️ Breaking Changes

- ⚠️  Make CLI code argument a file input ([13cd54d](https://github.com/noClaps/highlight/commit/13cd54d))

### ❤️ Contributors

- NoClaps <git@zerolimits.dev>

## v0.4.0

[compare changes](https://github.com/noClaps/highlight/compare/v0.3.6...v0.4.0)

### 🚀 Enhancements

- Add support for line numbers ([eec85a5](https://github.com/noClaps/highlight/commit/eec85a5))
- ⚠️  Convert into a CLI tool ([6fb019f](https://github.com/noClaps/highlight/commit/6fb019f))

### 📖 Documentation

- Add JSDoc for the Theme type ([09861ba](https://github.com/noClaps/highlight/commit/09861ba))
- Remove CLI about text ([71338d2](https://github.com/noClaps/highlight/commit/71338d2))
- Add build and usage instructions to README ([26e8edf](https://github.com/noClaps/highlight/commit/26e8edf))

### 🎨 Styles

- Use `use` instead of `#[macro_use]` for napi_derive macro ([2860201](https://github.com/noClaps/highlight/commit/2860201))

#### ⚠️ Breaking Changes

- ⚠️  Convert into a CLI tool ([6fb019f](https://github.com/noClaps/highlight/commit/6fb019f))

### ❤️ Contributors

- NoClaps <git@zerolimits.dev>

## v0.3.6

[compare changes](https://github.com/noClaps/highlight/compare/v0.3.5...v0.3.6)

### 🚀 Enhancements

- Highlight language injections ([7db74b1](https://github.com/noClaps/highlight/commit/7db74b1))

### 🩹 Fixes

- Update HTMLRenderer with new attributes callback ([df8d65a](https://github.com/noClaps/highlight/commit/df8d65a))

### 🏡 Chore

- Update dependencies ([b5f8d8b](https://github.com/noClaps/highlight/commit/b5f8d8b))
- Update dependencies ([32300a3](https://github.com/noClaps/highlight/commit/32300a3))
- Update dependencies and switch to text lockfile ([a4f2109](https://github.com/noClaps/highlight/commit/a4f2109))
- Update dependencies ([02bf97b](https://github.com/noClaps/highlight/commit/02bf97b))
- Change repo URL to GitHub ([6679fcb](https://github.com/noClaps/highlight/commit/6679fcb))
- Update dependencies ([a88c5b5](https://github.com/noClaps/highlight/commit/a88c5b5))
- Update dependencies ([2343c29](https://github.com/noClaps/highlight/commit/2343c29))

### ❤️ Contributors

- NoClaps <git@zerolimits.dev>

## v0.3.5

[compare changes](https://github.com/noClaps/highlight/compare/v0.3.4...v0.3.5)

### 📦 Build

- Add build script ([03dc7e3](https://github.com/noClaps/highlight/commit/03dc7e3))

### 🏡 Chore

- Remove packages directory ([db7b795](https://github.com/noClaps/highlight/commit/db7b795))
- Update bump-version script and remove it from package.json ([ba37f72](https://github.com/noClaps/highlight/commit/ba37f72))
- Fix bump script ([5935b88](https://github.com/noClaps/highlight/commit/5935b88))

### 🤖 CI

- Setup output directory in CI ([6c3c747](https://github.com/noClaps/highlight/commit/6c3c747))
- Use npm/ instead of packages/ as cwd ([8ae4e34](https://github.com/noClaps/highlight/commit/8ae4e34))

### ❤️ Contributors

- NoClaps <git@zerolimits.dev>

## v0.3.4

[compare changes](https://github.com/noClaps/highlight/compare/v0.3.3...v0.3.4)

### 📖 Documentation

- Update install command in README ([791b216](https://github.com/noClaps/highlight/commit/791b216))

### 🏡 Chore

- Update lockfile ([cd9e438](https://github.com/noClaps/highlight/commit/cd9e438))
- Prepare to publish on npmjs.com ([c83ea76](https://github.com/noClaps/highlight/commit/c83ea76))
- Update dependencies ([81d5381](https://github.com/noClaps/highlight/commit/81d5381))

### 🤖 CI

- Use registry-url variable ([92bfd79](https://github.com/noClaps/highlight/commit/92bfd79))

### ❤️ Contributors

- NoClaps <git@zerolimits.dev>

## v0.3.3

[compare changes](https://github.com/noClaps/highlight/compare/v0.3.2...v0.3.3)

### 🚀 Enhancements

- Use release build and strip flag ([2be6bf7](https://github.com/noClaps/highlight/commit/2be6bf7))

### 🏡 Chore

- Update dependencies ([ed64719](https://github.com/noClaps/highlight/commit/ed64719))
- Update dependencies ([eac075e](https://github.com/noClaps/highlight/commit/eac075e))
- Update dependencies ([ee9fedb](https://github.com/noClaps/highlight/commit/ee9fedb))
- Use exact version for optionalDependencies ([a9a8707](https://github.com/noClaps/highlight/commit/a9a8707))

### 🤖 CI

- Use napi artifacts command to move files into place ([e317b43](https://github.com/noClaps/highlight/commit/e317b43))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.3.2

[compare changes](https://github.com/noClaps/highlight/compare/v0.3.1...v0.3.2)

### 🩹 Fixes

- Escape HTML if no language is used ([605109e](https://github.com/noClaps/highlight/commit/605109e))

### 📖 Documentation

- Remove incorrect outputs note ([bacbf2c](https://github.com/noClaps/highlight/commit/bacbf2c))

### 🏡 Chore

- Remove Bun lockfile from bundle ([1f2f2d9](https://github.com/noClaps/highlight/commit/1f2f2d9))
- Add bump version script ([7a93742](https://github.com/noClaps/highlight/commit/7a93742))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.3.1

[compare changes](https://github.com/noClaps/highlight/compare/v0.3.0...v0.3.1)

### 🏡 Chore

- Update lockfile ([e0bfb74](https://github.com/noClaps/highlight/commit/e0bfb74))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.3.0

[compare changes](https://github.com/noClaps/highlight/compare/v0.2.1...v0.3.0)

### 🚀 Enhancements

- Use NAPI-RS for package ([cdc6e0d](https://github.com/noClaps/highlight/commit/cdc6e0d))
- ⚠️  Remove languages without highlight queries ([0fc7a37](https://github.com/noClaps/highlight/commit/0fc7a37))
- Add support for more languages ([ec5ac4b](https://github.com/noClaps/highlight/commit/ec5ac4b))
- Add support for more languages ([0eebe10](https://github.com/noClaps/highlight/commit/0eebe10))
- Add support for more languages ([dc826f8](https://github.com/noClaps/highlight/commit/dc826f8))
- ⚠️  Remove languages to reduce output size ([d76b852](https://github.com/noClaps/highlight/commit/d76b852))
- ⚠️  Cut down on supported languages ([6981230](https://github.com/noClaps/highlight/commit/6981230))

### 🩹 Fixes

- Export symbols from index.ts ([11fd454](https://github.com/noClaps/highlight/commit/11fd454))
- Add return type for highlight function ([8d7dd40](https://github.com/noClaps/highlight/commit/8d7dd40))

### 📖 Documentation

- Update install command ([a80704e](https://github.com/noClaps/highlight/commit/a80704e))

### 🏡 Chore

- Remove highlights directory ([c62a86f](https://github.com/noClaps/highlight/commit/c62a86f))
- Ignore generated directories ([800d0b3](https://github.com/noClaps/highlight/commit/800d0b3))
- Update dependencies ([030eda5](https://github.com/noClaps/highlight/commit/030eda5))
- Remove highlights directory from .gitattributes ([457435d](https://github.com/noClaps/highlight/commit/457435d))
- Remove dist directory ([11bb2cd](https://github.com/noClaps/highlight/commit/11bb2cd))
- Update Cargo.lock ([b18e59f](https://github.com/noClaps/highlight/commit/b18e59f))

### 🤖 CI

- Remove bun install step ([9a06067](https://github.com/noClaps/highlight/commit/9a06067))
- Remove some versions ([3af75b9](https://github.com/noClaps/highlight/commit/3af75b9))
- Remove some versions ([6921b5e](https://github.com/noClaps/highlight/commit/6921b5e))
- Publish to self-hosted registry ([34421cf](https://github.com/noClaps/highlight/commit/34421cf))
- Fix cd path ([d645089](https://github.com/noClaps/highlight/commit/d645089))
- Make dist directory before running build ([0cdd1d4](https://github.com/noClaps/highlight/commit/0cdd1d4))
- Fix action parameter ([16b1cf2](https://github.com/noClaps/highlight/commit/16b1cf2))
- Remove Node ([b94b48e](https://github.com/noClaps/highlight/commit/b94b48e))
- Change from cd to working-directory ([7904c91](https://github.com/noClaps/highlight/commit/7904c91))
- Run whoami to ensure login state ([46fbcee](https://github.com/noClaps/highlight/commit/46fbcee))
- Add scope to setup-bun ([a9f50a9](https://github.com/noClaps/highlight/commit/a9f50a9))
- Try printing bunfig.toml ([e8504d5](https://github.com/noClaps/highlight/commit/e8504d5))
- Cd, cat and publish all in one step ([65af98c](https://github.com/noClaps/highlight/commit/65af98c))
- Pass registry as command line flag ([a71913a](https://github.com/noClaps/highlight/commit/a71913a))
- Use Node action ([4f89534](https://github.com/noClaps/highlight/commit/4f89534))
- Check if packages are building ([caac867](https://github.com/noClaps/highlight/commit/caac867))
- Re-enable publishing ([d80dd9e](https://github.com/noClaps/highlight/commit/d80dd9e))
- Use npm to publish ([5ad1efe](https://github.com/noClaps/highlight/commit/5ad1efe))
- Don't stop everything else if one target fails ([42e2dd2](https://github.com/noClaps/highlight/commit/42e2dd2))

#### ⚠️ Breaking Changes

- ⚠️  Remove languages without highlight queries ([0fc7a37](https://github.com/noClaps/highlight/commit/0fc7a37))
- ⚠️  Remove languages to reduce output size ([d76b852](https://github.com/noClaps/highlight/commit/d76b852))
- ⚠️  Cut down on supported languages ([6981230](https://github.com/noClaps/highlight/commit/6981230))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.2.1

[compare changes](https://github.com/noClaps/highlight/compare/v0.2.0...v0.2.1)

### 🚀 Enhancements

- Add class name to `pre` element for easier selection ([f3a6c71](https://github.com/noClaps/highlight/commit/f3a6c71))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.2.0

[compare changes](https://github.com/noClaps/highlight/compare/v0.1.5...v0.2.0)

### 🩹 Fixes

- ⚠️  Add spans for each line of output ([f3dd91a](https://github.com/noClaps/highlight/commit/f3dd91a))

#### ⚠️ Breaking Changes

- ⚠️  Add spans for each line of output ([f3dd91a](https://github.com/noClaps/highlight/commit/f3dd91a))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.1.5

[compare changes](https://github.com/noClaps/highlight/compare/v0.1.4...v0.1.5)

### 🩹 Fixes

- Change query import path to be relative to file ([17ada54](https://github.com/noClaps/highlight/commit/17ada54))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.1.4

[compare changes](https://github.com/noClaps/highlight/compare/v0.1.3...v0.1.4)

### 🩹 Fixes

- Move doc comments to source files ([f0d1cdf](https://github.com/noClaps/highlight/commit/f0d1cdf))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.1.3

[compare changes](https://github.com/noClaps/highlight/compare/v0.1.2...v0.1.3)

### 🩹 Fixes

- Add types to `highlight()` and `bundledLanguages` ([504f935](https://github.com/noClaps/highlight/commit/504f935))

### 🤖 CI

- Remove build script ([ba20850](https://github.com/noClaps/highlight/commit/ba20850))
- Add build queries step ([eafb4a6](https://github.com/noClaps/highlight/commit/eafb4a6))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.1.2

[compare changes](https://github.com/noClaps/highlight/compare/v0.1.1...v0.1.2)

### 🤖 CI

- Fix publish pipeline ([ec6818b](https://github.com/noClaps/highlight/commit/ec6818b))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## v0.1.1

[compare changes](https://github.com/noClaps/highlight/compare/v0.1.0...v0.1.1)

### 🤖 CI

- Fix build pipeline ([d61c461](https://github.com/noClaps/highlight/commit/d61c461))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>

## 0.1.0


### 🚀 Enhancements

- Initial (semi-)working syntax highlighter ([e3d0f15](https://github.com/noClaps/highlight/commit/e3d0f15))
- Initial working version of syntax highlighter ([c9babdc](https://github.com/noClaps/highlight/commit/c9babdc))
- Add generated highlights and test files ([52dd8a8](https://github.com/noClaps/highlight/commit/52dd8a8))

### 🩹 Fixes

- Fix grammars and add corrections for all supported languages ([43b4240](https://github.com/noClaps/highlight/commit/43b4240))

### 📖 Documentation

- Update README with usage documentation ([a4c22c1](https://github.com/noClaps/highlight/commit/a4c22c1))

### 🏡 Chore

- Add license ([ee24cce](https://github.com/noClaps/highlight/commit/ee24cce))

### ❤️ Contributors

- NoClaps <04plugs-bios@icloud.com>
