VERSION 0.6
FROM alpine:3.19
WORKDIR /src

build:
    FROM rust:alpine3.19
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
    COPY +build/tokei /usr/local/bin/
    WORKDIR /src
    ENTRYPOINT [ "tokei" ]
    CMD [ "--help" ]
    ARG image_name=tokei:latest
    SAVE IMAGE --push $image_name
