VERSION 0.6
FROM alpine:3.14
WORKDIR /src

build:
    FROM rust:alpine3.14
    RUN apk update \
        && apk add \
            git \
            gcc \
            g++ \
            pkgconfig

    COPY . /src
    WORKDIR /src
    RUN cargo build --release
    SAVE ARTIFACT /src/target/release/tokei AS LOCAL ./tokei

docker:
    BUILD +build
    COPY +build/tokei /usr/local/bin/
    WORKDIR /src
    ENTRYPOINT [ "tokei" ]
    CMD [ "--help" ]
    SAVE IMAGE tokei
