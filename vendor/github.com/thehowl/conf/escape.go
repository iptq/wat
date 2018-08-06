package conf

import (
	"strings"
)

// Escape escapes characters for then putting it into conf field/values without issues.
func Escape(s string) string {
	return strings.NewReplacer(
		"\n", "\\\n",
		`\`, `\\`,
		`;`, `\;`,
		`=`, `\=`,
	).Replace(s)
}
