# syntax=docker/dockerfile:1
FROM rust:1.66-alpine as builder
WORKDIR /build
COPY . .
RUN apk add --no-cache build-base binutils mariadb-dev musl-dev bash cmake curl && \
    bash mariadb-static-build.sh && \ 
    bash zlib-static-build.sh && \
    ar x lib/libmysqlclient.a && \
    ar x /lib/libz.a && \
    ar x /usr/lib/libc.a && \
    ar rcs /build/lib/libmysqlclient.a *.o *.lo && \
    rm -rf *.o *.lo && \
    cargo build --target=x86_64-unknown-linux-musl --release --offline -F mysqlclient-static && \
    ldd /build/target/x86_64-unknown-linux-musl/release/tabby-light-settings-sync

FROM scratch

WORKDIR /config

COPY --from=builder /usr/lib/libmariadb.so.3 /usr/lib/libmariadb.so.3
COPY --from=builder /lib/libssl.so.3 /lib/libssl.so.3
COPY --from=builder /lib/libcrypto.so.3 /lib/libcrypto.so.3
COPY --from=builder /lib/libz.so.1 /lib/libz.so.1
COPY --from=builder /lib/ld-musl-x86_64.so.1 /lib/ld-musl-x86_64.so.1

ENV LD_LIBRARY_PATH=/usr/lib/:/lib/

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/tabby-light-settings-sync /
COPY --from=builder /build/users.exemple.yml .

CMD ["/tabby-light-settings-sync"]
