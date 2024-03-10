# syntax=docker/dockerfile:1
FROM rust:1.76-alpine AS builder
ARG FEATURE_FLAGS="-F|mysql-bundle|-F|all-login"
WORKDIR /build
COPY . .

RUN apk add --no-cache build-base

RUN if [[ "$FEATURE_FLAGS" == *"mysql-bundle"* ]]; then \
        apk add --no-cache binutils mariadb-dev musl-dev bash cmake curl && \
        bash scripts/mariadb-static-build.sh && \ 
        bash scripts/zlib-static-build.sh && \
        ar x lib/libmysqlclient.a && \
        ar x /lib/libz.a && \
        ar x /usr/lib/libc.a && \
        ar rcs /build/lib/libmysqlclient.a *.o *.lo && \
        rm -rf *.o *.lo; \
    fi

RUN if [[ "$FEATURE_FLAGS" == *"login"* ]]; then \
        echo "login enabled"; \
    else \
        rm -rf /build/web/*; \
    fi 


RUN cargo build --release --no-default-features --target-dir /build/target/docker $(echo "$FEATURE_FLAGS" | sed 's/|/ /g')

FROM scratch
ARG GIT_COMMIT

WORKDIR /config

COPY --from=builder /build/target/docker/release/rtabby-web-api /
COPY --from=builder /build/users.exemple.yml .
COPY --from=builder /build/web/ /www/web/
ENV STATIC_FILES_BASE_DIR=/www/web/
ENV GIT_COMMIT=$GIT_COMMIT

CMD ["/rtabby-web-api"]
