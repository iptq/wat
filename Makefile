.PHONY: all bindata clean is-go-installed is-yarn-installed

all: wat

web/dist/index.html: is-yarn-installed
	cd web && yarn run build

bindata: web/dist/index.html
	fileb0x b0x.toml

wat: is-go-installed bindata
	go build -o $@

is-go-installed:
	go version
	go get -u -v github.com/UnnoTed/fileb0x

is-yarn-installed:
	cd web && yarn

