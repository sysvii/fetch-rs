FROM golang:1.10 AS build

ARG version

RUN mkdir -p /go/src/gitlab.com/zyphrus/fetcherd-go
WORKDIR /go/src/gitlab.com/zyphrus/fetcherd-go
COPY . .

RUN go get -v -d ./ && \
    CGO_ENABLED=0 GOOS=linux go build -ldflags "-X main.version=${version:-$(date -u +%Y%m%d.%H%M%S)}" -a -installsuffix cgo -v -o /usr/local/bin/fetcherd cmd/fetcherd/main.go

FROM alpine:3.7

COPY --from=build /usr/local/bin/fetcherd /usr/local/bin/fetcherd

RUN apk --update upgrade && \
    apk add curl ca-certificates && \
    update-ca-certificates && \
    rm -rf /varcache/apk/*

HEALTHCHECK --interval=5m CMD wget http://localhost:8181/healthcheck -q -O /dev/null
ENTRYPOINT ["/usr/local/bin/fetcherd", "-config", "/etc/fetcherd.toml"]
