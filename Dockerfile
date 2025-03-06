FROM clux/muslrust:stable AS build

COPY . .

RUN cargo build --bins --release


FROM alpine:3.21

RUN apk add --no-cache tini openssl
ENTRYPOINT ["/sbin/tini", "--"]

COPY --from=build /volume/target/*-unknown-linux-musl/release/tunnelto /volume/target/*-unknown-linux-musl/release/tunnelto_server /usr/local/bin/

CMD ["tunnelto"]
