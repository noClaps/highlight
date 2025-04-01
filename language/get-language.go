package language

import (
	"embed"
	"fmt"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_agda "github.com/tree-sitter/tree-sitter-agda/bindings/go"
)

//go:embed "queries/*.scm"
var queries embed.FS

// queries are:
// 1. injections.scm
// 2. locals.scm
// 3. highlights.scm

func getQuery(lang string) ([]byte, error) {
	switch lang {
	case "agda":
		return queries.ReadFile("queries/agda.scm")
	}

	return nil, fmt.Errorf("Language not supported: %s", lang)
}

func getTreesitterLang(lang string) (*tree_sitter.Language, error) {
	switch lang {
	case "agda":
		return tree_sitter.NewLanguage(tree_sitter_agda.Language()), nil
	}

	return nil, fmt.Errorf("Language not supported: %s", lang)
}
