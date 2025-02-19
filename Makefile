build:
	@cargo build -r
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
