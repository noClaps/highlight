package highlighting

import (
	"fmt"

	"github.com/noclaps/highlight/language"
	tree_sitter "github.com/tree-sitter/go-tree-sitter"
)

func HighlightCode(lang string, code string, highlightNames []string) (string, error) {
	language, err := language.New(lang)
	if err != nil {
		return "", err
	}

	query, queryErr := tree_sitter.NewQuery(&language.Language, language.Queries)
	if queryErr != nil {
		return "", queryErr
	}
	defer query.Close()

	parser := tree_sitter.NewParser()
	if parser == nil {
		return "", fmt.Errorf("Error initialising parser for language %s", lang)
	}
	defer parser.Close()
	parser.SetLanguage(&language.Language)

	tree := parser.Parse([]byte(code), nil)
	if tree == nil {
		return "", fmt.Errorf("Error initialising tree for language %s", lang)
	}
	defer tree.Close()

	rootNode := tree.RootNode()

	queryCursor := tree_sitter.NewQueryCursor()
	defer queryCursor.Close()

	captureNames := query.CaptureNames()
	captures := queryCursor.Captures(query, rootNode, []byte(code))
	capture, _ := captures.Next()

	output := ""
	var lastEnd uint = 0
	for capture != nil {
		for _, cap := range capture.Captures {
			start, end := cap.Node.ByteRange()
			if start < lastEnd {
				continue
			}

			if start > lastEnd {
				output += fmt.Sprint(code[lastEnd:start])
			}

			codeSlice := []byte(code)[start:end]
			captureName := captureNames[cap.Index]

			for _, name := range highlightNames {
				if len(captureName) >= len(name) && captureName[:len(name)] == name {
					captureName = name
				}
			}

			output += fmt.Sprintf(`<span class="%s">%s</span>`, captureName, codeSlice)
			lastEnd = end
		}
		capture, _ = captures.Next()
	}
	output += fmt.Sprintf("%s", []byte(code[lastEnd:]))

	return fmt.Sprintf(`<pre class="ts-highlight"><code>%s</code></pre>`, output), nil
}
