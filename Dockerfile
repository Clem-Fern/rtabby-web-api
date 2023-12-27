# syntax=docker/dockerfile:1
FROM rust:1.73-alpine AS builder
ARG FEATURE_FLAGS="-F mysqlclient-static -F github-login"
WORKDIR /build
COPY . .
RUN if [[ "$FEATURE_FLAGS" == *"mysqlclient-static"* ]]; then \
        apk add --no-cache build-base binutils mariadb-dev musl-dev bash cmake curl && \
        bash scripts/mariadb-static-build.sh && \ 
        bash scripts/zlib-static-build.sh && \
        ar x lib/libmysqlclient.a && \
        ar x /lib/libz.a && \
        ar x /usr/lib/libc.a && \
        ar rcs /build/lib/libmysqlclient.a *.o *.lo && \
        rm -rf *.o *.lo; \
    fi
RUN if [[ "$FEATURE_FLAGS" == *"sqlite-bundle"* ]]; then \
        apk add --no-cache build-base; \
    fi 

RUN if [[ "$FEATURE_FLAGS" == *"login"* ]]; then \
        echo "login enabled"; \
    else \
        rm -rf /build/web/*; \
    fi 


RUN cargo build --target=x86_64-unknown-linux-musl --release $(echo "$FEATURE_FLAGS" | sed 's/|/ /g')

FROM scratch

WORKDIR /config

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/rtabby-web-api /
COPY --from=builder /build/users.exemple.yml .
COPY --from=builder /build/web/ /www/web/
ENV STATIC_FILES_BASE_DIR=/www/web/

CMD ["/rtabby-web-api"]
