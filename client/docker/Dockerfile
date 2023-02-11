# FROM rust:1.56.0 AS cacher
FROM ekidd/rust-musl-builder:stable AS builder

WORKDIR /user/src/client
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
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/client*

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/libclient*

RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip /user/src/client/target/x86_64-unknown-linux-musl/release/client

# FROM ekidd/rust-musl-builder:stable
FROM rust:1.67

COPY --from=builder \
    /user/src/client/target/x86_64-unknown-linux-musl/release/client /
RUN mkdir target

ENTRYPOINT [ "/client" ]