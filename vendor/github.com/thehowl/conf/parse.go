package conf

// FieldValue is a field=value pair in the configuration.
type FieldValue struct {
	Field string
	Value string
}

const (
	currentName = iota
	currentValue
	currentComment
)

// Parse converts some bytes into various FieldValue pairs.
func Parse(data []byte) []FieldValue {
	fvs := []FieldValue{}
	var (
		fieldName  string
		fieldValue string
		nextNormal bool
		current    byte
	)

	for _, c := range data {
		if current == currentComment && c != '\n' {
			continue
		}
		if nextNormal {
			switch current {
			case currentName:
				fieldName += string(c)
			case currentValue:
				fieldValue += string(c)
			}
			nextNormal = false
			continue
		}
		switch c {
		case '=':
			switch current {
			// if we are still at the name, let's switch to a value.
			case currentName:
				current = currentValue
			// if we are already at the value, treat the = character like any other sign
			case currentValue:
				fieldValue += string(c)
			}
		case ';':
			current = currentComment
		case '\n':
			if fieldName != "" && fieldValue != "" {
				fvs = append(fvs, FieldValue{
					Field: fieldName,
					Value: removeTrailingCR(fieldValue),
				})
			}
			fieldName = ""
			fieldValue = ""
			current = currentName
		case '\\':
			nextNormal = true
		default:
			switch current {
			case currentName:
				fieldName += string(c)
			case currentValue:
				fieldValue += string(c)
			}
			nextNormal = false
		}
	}
	if fieldName != "" && fieldValue != "" {
		fvs = append(fvs, FieldValue{
			Field: fieldName,
			Value: removeTrailingCR(fieldValue),
		})
	}
	return fvs
}
func removeTrailingCR(s string) string {
	if len(s) == 0 {
		return s
	}
	if s[len(s)-1] == '\r' {
		return s[:len(s)-1]
	}
	return s
}
