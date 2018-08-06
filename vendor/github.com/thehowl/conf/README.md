# conf [![Build Status](https://travis-ci.org/thehowl/conf.svg?branch=master)](https://travis-ci.org/thehowl/conf) [![GoDoc](https://godoc.org/github.com/thehowl/conf?status.svg)](https://godoc.org/github.com/thehowl/conf)

(yes I am that creative with names)

I have been using [ini](http://gopkg.in/ini.v1) for managing configuration files in go for quite some time. One of the things that had bothered me though, was that it really was a pain to set up for small projects, as it's just boilerplate code over and over. So I decided to write my own configuration file system, and now I'm here.

## Quick start

```go
package main

import (
	"github.com/thehowl/conf"
)

type myConf struct {
	Port     string `description:"The port from which the application will take HTTP requests"`
	Password string
	MaxUsers int
}

func main() {
	c := myConf{}
	err := conf.Load(&c, "myapp.conf")
	if err == conf.ErrNoFile {
		// You can export your conf to a file, so you can write default values.
		conf.Export(myConf{
			Port:     ":8080",
			Password: "hunter2",
			MaxUsers: 9001,
		}, "myapp.conf")
		fmt.Println("Please compile the configuration file (myapp.conf.)")
		return
	}
	if err != nil {
		panic(err)
	}
	// You can now use the values in `c` without thinking about the actual configuration ever again!
	fmt.Printf("%#v\n", c)
}
```

## Configuration file format

```
; This is an example configuration file generated with `conf`. Comments are done using semicolons.
;
; This is a simple string value in the configuration:
String=Hello world!

; Note that there are no spaces between the field (key) name and its value. Conf does not trim strings.
; int, float, uint values are done just as easily. You just need to write that they're of that type in
; the struct, and conf will do all the magic!
Int=481

; There are also bools.
Bool=1
; Bools are retrieved through [ParseBool](https://golang.org/pkg/strconv/#ParseBool), as such they
; need to be one of 1, t, T, TRUE, true, True, 0, f, F, FALSE, false, False.

; But, what about strings with newlines?
AreTheyPossible=Yes\
They\
Are!
; If you need to export a flag with a multiline string, conf will automatically escape it.
;
; By the way, conf automatically ignores lines without a valid field=value combination, including
; empty lines, so you can use them as comments, although discouraged.
So yes, this line will be silently ignored!
=This one, too!
And this one, too!=

; Escaping can not only be done with newlines. Here's what you can possibly escape!
Fields\=With\=Equal\=Signs=can be escaped!
Comments=Can \; be escaped!
Oh yeah, fields=can also have spaces and what not in them.; You can also write comments straight after a value!

; And that's all you need to know for using `conf`!
```

## License

MIT
