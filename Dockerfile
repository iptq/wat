FROM golang:onbuild

RUN mkdir /build
WORKDIR /build
COPY . /build/
RUN GOOS=linux go build -a -installsuffix cgo -o wat .

FROM debian:jessie-slim
RUN apt-get update -y && apt-get install -y \
    libsqlite3-dev
WORKDIR /root
COPY --from=0 /build/wat .
CMD ["/root/wat", "-conf", "/wat.conf"]
