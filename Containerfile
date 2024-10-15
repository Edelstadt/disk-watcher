FROM rust:1.81.0 AS builder
WORKDIR /usr/src/

RUN apt-get update && apt-get install -y musl-tools pkg-config libsqlite3-dev && rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new disk-watcher
WORKDIR /usr/src/disk-watcher
COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM debian:bookworm-slim
ARG USERNAME=disk-usage-watcher
ARG USER_UID=2000
ARG USER_GID=$USER_UID

COPY --from=builder /usr/local/cargo/bin/disk-watcher /usr/local/bin/disk-watcher

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME
RUN mkdir -p /data \
  && chown -R $USERNAME:$USERNAME /data

  USER $USERNAME

CMD ["disk-watcher"]
