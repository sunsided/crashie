FROM rust:1.75-alpine3.19 AS builder
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY ./ /app
RUN cargo build --release
RUN strip target/release/crashie

FROM alpine:3.19
RUN apk add --no-cache libgcc
COPY --from=builder /app/target/release/crashie .
ENV PATH=$PATH:/
ENTRYPOINT ["/crashie"]
CMD ["--help"]
