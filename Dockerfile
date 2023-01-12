FROM rust:1.66-alpine as builder
WORKDIR /build
COPY . .
RUN apk add --no-cache build-base sqlite-dev && cargo build --release -F docker

FROM scratch

WORKDIR /storage

COPY --from=builder /build/target/release/tabby-light-settings-sync /
COPY --from=builder /build/users.exemple.yml .

CMD ["/tabby-light-settings-sync"]  
