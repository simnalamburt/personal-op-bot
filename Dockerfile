FROM --platform=linux/amd64 messense/rust-musl-cross:x86_64-musl AS amd64
COPY . .
RUN cargo install --path . --root /x

FROM --platform=linux/amd64 messense/rust-musl-cross:aarch64-musl AS arm64
COPY . .
RUN cargo install --path . --root /x

FROM ${TARGETARCH} AS build

FROM alpine
COPY --from=build /x/bin/personal-op-bot /usr/local/bin/personal-op-bot
CMD ["personal-op-bot", "--config", "/a/config.toml"]
