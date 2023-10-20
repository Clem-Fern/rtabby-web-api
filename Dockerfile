# syntax=docker/dockerfile:1
FROM rust:1.66-alpine as builder
WORKDIR /build
COPY . .
RUN apk add --no-cache build-base binutils mariadb-dev musl-dev bash cmake curl && \
    bash scripts/mariadb-static-build.sh && \ 
    bash scripts/zlib-static-build.sh && \
    ar x lib/libmysqlclient.a && \
    ar x /lib/libz.a && \
    ar x /usr/lib/libc.a && \
    ar rcs /build/lib/libmysqlclient.a *.o *.lo && \
    rm -rf *.o *.lo && \
    cargo build --target=x86_64-unknown-linux-musl --release -F mysqlclient-static

FROM scratch

WORKDIR /config

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/tabby-light-settings-sync /
COPY --from=builder /build/users.exemple.yml .

CMD ["/tabby-light-settings-sync"]
