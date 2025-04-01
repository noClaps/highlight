package language

import (
	tree_sitter "github.com/tree-sitter/go-tree-sitter"
)

type Language struct {
	Name     string
	Queries  string
	Language tree_sitter.Language
}

func New(lang string) (Language, error) {
	query, err := getQuery(lang)
	if err != nil {
		return Language{}, err
	}

	tsLang, err := getTreesitterLang(lang)
	if err != nil {
		return Language{}, err
	}

	return Language{Name: lang, Queries: string(query), Language: *tsLang}, nil
}
