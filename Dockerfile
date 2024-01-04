FROM rust:1.75-alpine3.19 AS builder
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache musl-dev
RUN cargo install cargo-auditable
WORKDIR /app
COPY ./ /app
RUN cargo auditable build --release
RUN strip target/release/crashie

FROM alpine:3.19
RUN apk add --no-cache libgcc
COPY --from=builder /app/target/release/crashie .
ENV PATH=$PATH:/

ENTRYPOINT ["/crashie"]
CMD ["--help"]

ARG DESCRIPTION="A Command-Line Utility that exits with a random exit code after a configurable delay"
LABEL org.opencontainers.image.title="crashie"
LABEL org.opencontainers.image.description="$DESCRIPTION"
LABEL org.opencontainers.artifact.description="$DESCRIPTION"
LABEL org.opencontainers.image.documentation="https://github.com/sunsided/crashie"
LABEL org.opencontainers.image.source="https://github.com/sunsided/crashie"
LABEL org.label-schema.schema-version="1.0"
LABEL org.label-schema.docker.cmd="docker run --rm -it sunside/crashie"
