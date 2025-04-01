package main

import (
	"fmt"
	"html"
	"maps"
	"slices"
	"strings"

	"github.com/noclaps/highlight/highlighting"
	"github.com/noclaps/highlight/theme"
)

func Highlight(code string, language string, theme theme.Theme) (string, error) {
	globalStyle := ""
	if theme.Bg != nil {
		globalStyle += fmt.Sprintf("background-color:%s;", *theme.Bg)
	}
	if theme.Fg != nil {
		globalStyle += fmt.Sprintf("color:%s;", *theme.Fg)
	}

	if language == "plaintext" || language == "plain" || language == "text" || language == "txt" {
		return fmt.Sprintf(`<pre class="ts-highlight" style="%s"><code>%s</code></pre>`, globalStyle, html.EscapeString(code)), nil
	}

	highlightNames := slices.Collect(maps.Keys(theme.Highlights))
	highlightedText, err := highlighting.HighlightCode(language, code, highlightNames)
	if err != nil {
		return "", err
	}

	for key, val := range theme.Highlights {
		style := ""
		if val.Color != nil {
			style += fmt.Sprintf("color:%s;", *val.Color)
		}
		if val.FontWeight != nil {
			style += fmt.Sprintf("font-weight:%d;", *val.FontWeight)
		}
		if val.FontStyle != nil {
			style += fmt.Sprintf("font-style:%s", *val.FontStyle)
		}
		if val.BackgroundColor != nil {
			style += fmt.Sprintf("background-color:%s;", *val.BackgroundColor)
		}

		highlightedText = strings.ReplaceAll(
			highlightedText,
			fmt.Sprintf(`<span class="%s"`, key),
			fmt.Sprintf(`<span class="%s" style="%s"`, key, style))
	}

	if theme.LineNumbers != nil {
		lines := slices.Collect(strings.Lines(highlightedText))
		maxLineNum := len(fmt.Sprint(len(lines) + 1))

		rightSpace := 1
		if theme.LineNumbers.RightSpace != nil {
			rightSpace = int(*theme.LineNumbers.RightSpace)
		}

		textWithLineNos := ""
		for index, line := range lines {
			textWithLineNos += fmt.Sprintf(
				`<span class="line-number" style="color:%s;margin-right:%dch">%d</span>%s`,
				theme.LineNumbers.Color,
				maxLineNum+rightSpace-len(fmt.Sprint(index+1)),
				index+1,
				line,
			)
		}
	}

	return fmt.Sprintf(
		`<pre class="ts-highlight" style="%s"><code>%s</code></pre>`,
		globalStyle, strings.TrimSpace(highlightedText)), nil
}
