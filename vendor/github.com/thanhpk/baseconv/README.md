# About baseconv [![Build Status](https://travis-ci.org/knq/baseconv.svg)](https://travis-ci.org/knq/baseconv) [![Coverage Status](https://coveralls.io/repos/knq/baseconv/badge.svg?branch=master&service=github)](https://coveralls.io/github/knq/baseconv?branch=master) #

A simple [Go](http://www.golang.org/project/) package for converting between
strings in arbitrary bases.

This package is useful when working with extremely large numbers (larger than
int64), and need to convert them to different base (ie, decimal, hex, octal,
etc) representations, and thus cannot use the standard go libraries.

This was written for a specific use case where there was a need to
encode/decode large numbers stored as strings in a database.

This is similar in concept to PHP's [```base_convert```](http://php.net/manual/en/function.base-convert.php)
function.

## Installation ##

Install the package via the following:

    go get -u github.com/thanhpk/baseconv

## Usage ##

Please see [the GoDoc API page](http://godoc.org/github.com/knq/baseconv) for a
full API listing.

The baseconv package can be used similarly to the following:
```go
// example.go
package main

import (
	"fmt"
	"github.com/thanhpk/baseconv"
)

func main() {
	valHex := "70B1D707EAC2EDF4C6389F440C7294B51FFF57BB"
	valDec, _ := baseconv.DecodeHex(valHex)
	val62, _ := baseconv.Convert(valHex, baseconv.DigitsHex, baseconv.Digits62)
	val36, _ := baseconv.Encode36(val62, baseconv.Digits62)

	fmt.Println("dec string: " + valDec)
	fmt.Printf("62 string: " + val62)
	fmt.Println("36 string: " + val36)

	conVal36, _ := baseconv.Decode36(val36, baseconv.DigitsDec)
	fmt.Printf("dec and 36 values same: %t\n", valDec == conVal36)

	val62, _ = baseconv.Convert("7(n42-&DG$MT", baseconv.ASCII, baseconv.Digits62)
	fmt.Println("62 string: " + val62)
}
```

Example output:
```sh
dec string: 643372930067913326838082478477533553256088688571
62 string: G4wUogcmwGCpA70D91bEZvVVVAx36 string: D5WJFAEW7FYPQN2KA6XPOFDLWNS9HA3
dec and 36 values same: true
62 string: eiEKIgmLml7S8
```
