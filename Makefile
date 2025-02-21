build:
	@cargo build -r --features binary --bin highlight
	@cp target/release/highlight .

install: build
	@install ./highlight ~/.local/bin
	@echo "Installed highlight to ~/.local/bin"

LANG="html"
CODE="test/test.${LANG}"

.PHONY: test
test: build
	@./highlight $(CODE) -t theme.toml -l ${LANG} > out.html
	@open out.html

bench: build
	@hyperfine "./highlight $(CODE) -t theme.toml -l ${LANG}" -w 50
