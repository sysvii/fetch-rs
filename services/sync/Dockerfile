FROM golang:1.12 AS build

ARG version

RUN mkdir -p /code
WORKDIR /code
COPY . .

RUN make docker

FROM alpine:3.9

COPY --from=build /code/bin/sitesync /usr/local/bin/sitesync

RUN apk --update upgrade && \
    apk add curl ca-certificates && \
    update-ca-certificates && \
    rm -rf /varcache/apk/*

HEALTHCHECK --interval=5m --timeout=30s --retries=3 --start-period=10s \
    CMD wget http://localhost:3000/healthcheck -q -O /dev/null

CMD ["/usr/local/bin/sitesync", "-config", "/etc/sitesync.toml"]
