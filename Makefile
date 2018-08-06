.PHONY: all bindata clean is-go-installed is-yarn-installed

wat: is-go-installed bindata
	go build -o $@ -tags bindata

web/dist/index.html: is-yarn-installed
	cd web && yarn run build

bindata: web/dist/index.html
	fileb0x b0x.toml

wat-dev: is-go-installed web/dist/index.html 
	go build -o $@

is-go-installed:
	go version
	go get -v github.com/UnnoTed/fileb0x

is-yarn-installed:
	cd web && yarn

all: wat wat-dev

