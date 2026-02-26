FROM alpine:3.23 AS build
ENV RUSTFLAGS='-C target-feature=+crt-static'
ENV PATH=$PATH:/root/.cargo/bin
RUN apk add -U \
    build-base \
    curl \
    musl-dev && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
        sh -s -- --no-modify-path -y
COPY . /src
WORKDIR /src
RUN cargo build --release --target "$(uname -m)-unknown-linux-musl"

FROM scratch
COPY --from=build /src/target/*/release/chandler /
ENTRYPOINT ["/chandler"]
