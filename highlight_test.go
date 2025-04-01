package main

import (
	"fmt"
	"os"
	"testing"

	"github.com/noclaps/highlight/highlighting"
	"github.com/noclaps/highlight/language"
	"github.com/noclaps/highlight/theme"
)

func TestReadTheme(t *testing.T) {
	themeFile, err := os.ReadFile("theme.toml")
	if err != nil {
		t.Error(err)
	}

	_, err = theme.New(string(themeFile))
	if err != nil {
		t.Error(err)
	}
}

func TestGetLanguage(t *testing.T) {
	_, err := language.FromString("agda")
	if err != nil {
		t.Error(err)
	}
}

func TestHighlightCode(t *testing.T) {
	input, err := os.ReadFile("test/test.agda")
	if err != nil {
		t.Error(err)
	}

	_, err = highlighting.HighlightCode("agda", string(input), []string{})
	if err != nil {
		t.Error(err)
	}

}

func TestHighlight(t *testing.T) {
	input, err := os.ReadFile("test/test.agda")
	if err != nil {
		t.Error(err)
	}
	themeFile, err := os.ReadFile("theme.toml")
	if err != nil {
		t.Error(err)
	}

	theme, err := theme.New(string(themeFile))
	if err != nil {
		t.Error(err)
	}

	output, err := Highlight(string(input), "agda", *theme)
	if err != nil {
		t.Error(err)
	}

	output = fmt.Sprintf(`<head><meta charset="utf8"/></head>%s`, output)

	err = os.WriteFile("out.html", []byte(output), 0644)
	if err != nil {
		t.Error(err)
	}
}
