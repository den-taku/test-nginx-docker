# FROM rust:1.56.0 AS cacher
FROM ekidd/rust-musl-builder:stable AS builder

WORKDIR /user/src/server
COPY Cargo.toml Cargo.toml

# for caching
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo 'fn main(){println!("cached!")}' > src/main.rs

RUN cargo build --release --target x86_64-unknown-linux-musl

RUN rm src/main.rs

# copy code
COPY src/ src/

# for recompile
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/server*

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/libserver*

RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip /user/src/server/target/x86_64-unknown-linux-musl/release/server

# FROM ekidd/rust-musl-builder:stable
FROM rust:1.67

COPY --from=builder \
    /user/src/server/target/x86_64-unknown-linux-musl/release/server /
RUN mkdir target

EXPOSE 7890

ENTRYPOINT [ "/server" ]

# FROM rust:1.67

# WORKDIR /usr/src/server

# COPY ./ ./

# RUN cargo build --release

# EXPOSE 7890

# CMD ["./target/release/server"]
# CMD ["bash"]