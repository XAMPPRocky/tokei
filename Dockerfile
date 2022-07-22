# docker run --rm -it -v $(pwd):/src tokei
FROM rust:alpine3.14 as base
RUN apk update \
    && apk add \
        git \
        gcc \
        g++ \
        pkgconfig

COPY . /src

WORKDIR /src
RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --release

FROM alpine:3.14 as tool

RUN apk update \
    && apk add \
        libgcc

COPY --from=base /src/target/release/tokei /usr/local/bin/
WORKDIR /src

ENTRYPOINT [ "tokei" ]
CMD [ "--help" ]
