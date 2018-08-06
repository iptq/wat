# UUID4

[![Build Status](https://travis-ci.org/frankenbeanies/uuid4.svg?branch=master)](https://travis-ci.org/frankenbeanies/uuid4) [![Coverage Status](https://coveralls.io/repos/github/frankenbeanies/uuid4/badge.svg?branch=master)](https://coveralls.io/github/frankenbeanies/uuid4?branch=master)

An RFC 4122 compliant UUID library

## License

[MIT](LICENSE)

## Installation

```
$ go get github.com/frankenbeanies/uuid4
```

## Usage

```go
import "github.com/frankenbeanies/uuid4"
```

## Methods

### New()

Generates a new UUID4

```go
uuid := uuid4.New()
```

### String()

Provides an RFC 4122 compliant string representation of the UUID4

```go
uuidStr := uuid4.New().String()
fmt.Println(uuidStr)
```

### Bytes()

Provides the byte representation of UUID4

```go
uuidBytes := uuid4.New().Bytes()
fmt.Println(uuid)
```

### ParseString()

Parses string into a UUID4

```go
uuid := uuid4.ParseString("cc2161ae-33c1-4cb1-aa53-e81000f20a30")
```