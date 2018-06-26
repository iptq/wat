FROM golang:onbuild

RUN mkdir /build
WORKDIR /build
COPY . /build/
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o wat .

FROM alpine:latest
RUN apk --no-cache add ca-certificates
RUN touch /wat.conf
WORKDIR /root
COPY --from=0 /build/wat .
CMD ["/root/wat", "-conf", "/wat.conf"]
