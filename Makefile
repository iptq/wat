.PHONY: all bindata clean is-go-installed is-yarn-installed

wat: is-go-installed bindata
	go build -o $@ -tags bindata

app/dist/index.html: is-yarn-installed
	cd app && yarn run build

bindata: app/dist/index.html

wat: is-go-installed bindata
	go build -v ./cmd/wat

is-go-installed:
	go version
	go get -v github.com/UnnoTed/fileb0x

is-yarn-installed:
	cd app && yarn
