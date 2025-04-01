package theme

import "github.com/BurntSushi/toml"

type highlight struct {
	Color           *string
	FontWeight      *uint
	FontStyle       *string
	BackgroundColor *string
}

type lineNumbers struct {
	Color      string
	RightSpace *uint
}

type Theme struct {
	Fg          *string
	Bg          *string
	LineNumbers *lineNumbers
	Highlights  map[string]highlight
}

func New(theme string) (*Theme, error) {
	newTheme := Theme{}
	_, err := toml.Decode(theme, &newTheme)
	if err != nil {
		return nil, err
	}

	return &newTheme, nil
}

func Blank() Theme {
	return Theme{}
}
