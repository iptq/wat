// Package baseconv converts a string in an arbitrary base to any other
// arbitrary base.
package baseconv

import (
	"errors"
	"fmt"
	"unicode/utf8"
)

// Convert num from specified base to a different base.
func Convert(num, fromBase, toBase string) (string, error) {
	if num == "" {
		return "", errors.New("invalid number")
	}

	if len(fromBase) < 2 {
		return "", errors.New("invalid fromBase")
	}

	if len(toBase) < 2 {
		return "", errors.New("invalid toBase")
	}

	// rune counts
	fromLenRunes := utf8.RuneCountInString(fromBase)
	toLenRunes := utf8.RuneCountInString(toBase)
	numLen := utf8.RuneCountInString(num)

	// loop over unicode runes in original string and store representative
	// values in number -- number[i] = index(num[i], fromBase)
	number, ipos := make([]int, numLen), 0
	for i, r := range num {
		jpos, found := 0, false
		for _, s := range fromBase {
			if r == s {
				number[ipos] = jpos
				found = true
				break
			}

			jpos++
		}

		// if character wasn't found in fromBase, then error
		if !found {
			return "", fmt.Errorf("invalid character '%c' at position %d (%d)", r, ipos, i)
		}

		ipos++
	}

	// split the runes in toBase
	todigits, idx := make([]rune, toLenRunes), 0
	for _, r := range toBase {
		todigits[idx] = r
		idx++
	}

	// loop until whole number is converted
	var result []rune
	for {
		divide, newlen := 0, 0

		// perform division manually (which is why this works with big numbers)
		for i := 0; i < numLen; i++ {
			divide = divide*fromLenRunes + number[i]
			if divide >= toLenRunes {
				number[newlen] = int(divide / toLenRunes)
				divide = divide % toLenRunes
				newlen++
			} else if newlen > 0 {
				number[newlen] = 0
				newlen++
			}
		}

		numLen = newlen
		result = append(result, todigits[divide])

		if newlen == 0 {
			break
		}
	}

	// reverse result
	for i, j := 0, len(result)-1; i < j; i, j = i+1, j-1 {
		result[i], result[j] = result[j], result[i]
	}

	return string(result), nil
}

const (
	// DigitsBin represents binary digits
	DigitsBin = "01"

	// DigitsOct represents octal Digits
	DigitsOct = "01234567"

	// DigitsDec represents decimal digits
	DigitsDec = "0123456789"

	// DigitsHex represents hex digits
	DigitsHex = "0123456789ABCDEF"

	// Digits36 represents base36 digits
	Digits36 = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"

	// Digits62 represents base62 digits
	Digits62 = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"

	// Digits64 represents base64 digits
	Digits64 = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_"

	// ASCII represents all ascii printable characters
	ASCII = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"
)

// EncodeBin encodes a string into DigitsBin with optional specified base (default: DigitsDec).
func EncodeBin(num string, base ...string) (string, error) {
	from := DigitsDec
	if len(base) > 0 {
		from = base[0]
	}

	return Convert(num, from, DigitsBin)
}

// DecodeBin decodes a string from DigitsBin with optional specified base (default: DigitsDec).
func DecodeBin(num string, base ...string) (string, error) {
	to := DigitsDec
	if len(base) > 0 {
		to = base[0]
	}

	return Convert(num, DigitsBin, to)
}

// EncodeOct encodes a string into DigitsOct with optional specified base (default: DigitsDec).
func EncodeOct(num string, base ...string) (string, error) {
	from := DigitsDec
	if len(base) > 0 {
		from = base[0]
	}

	return Convert(num, from, DigitsOct)
}

// DecodeOct decodes a string from DigitsOct with optional specified base (default: DigitsDec).
func DecodeOct(num string, base ...string) (string, error) {
	to := DigitsDec
	if len(base) > 0 {
		to = base[0]
	}

	return Convert(num, DigitsOct, to)
}

// EncodeHex encodes a string into DigitsHex with optional specified base (default: DigitsDec).
func EncodeHex(num string, base ...string) (string, error) {
	from := DigitsDec
	if len(base) > 0 {
		from = base[0]
	}

	return Convert(num, from, DigitsHex)
}

// DecodeHex decodes a string from DigitsHex with optional specified base (default: DigitsDec).
func DecodeHex(num string, base ...string) (string, error) {
	to := DigitsDec
	if len(base) > 0 {
		to = base[0]
	}

	return Convert(num, DigitsHex, to)
}

// Encode36 encodes a string into Digits36 with optional specified base (default: DigitsDec).
func Encode36(num string, base ...string) (string, error) {
	from := DigitsDec
	if len(base) > 0 {
		from = base[0]
	}

	return Convert(num, from, Digits36)
}

// Decode36 decodes a string from Digits36 with optional specified base (default: DigitsDec).
func Decode36(num string, base ...string) (string, error) {
	to := DigitsDec
	if len(base) > 0 {
		to = base[0]
	}

	return Convert(num, Digits36, to)
}

// Encode62 encodes a string into Digits62 with optional specified base (default: DigitsDec).
func Encode62(num string, base ...string) (string, error) {
	from := DigitsDec
	if len(base) > 0 {
		from = base[0]
	}

	return Convert(num, from, Digits62)
}

// Decode62 decodes a string from Digits62 with optional specified base (default: DigitsDec).
func Decode62(num string, base ...string) (string, error) {
	to := DigitsDec
	if len(base) > 0 {
		to = base[0]
	}

	return Convert(num, Digits62, to)
}

// Encode64 encodes a string into Digits64 with optional specified base (default: DigitsDec).
func Encode64(num string, base ...string) (string, error) {
	from := DigitsDec
	if len(base) > 0 {
		from = base[0]
	}

	return Convert(num, from, Digits64)
}

// Decode64 decodes a string from Digits64 with optional specified base (default: DigitsDec).
func Decode64(num string, base ...string) (string, error) {
	to := DigitsDec
	if len(base) > 0 {
		to = base[0]
	}

	return Convert(num, Digits64, to)
}
