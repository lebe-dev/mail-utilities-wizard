FROM node:20.12.2-alpine3.19 as frontend-build

WORKDIR /build

COPY frontend/ /build

RUN yarn && \
    npm run build

FROM rust:1.78.0-alpine3.20 as app-build

WORKDIR /build

RUN mkdir -p /build/static && \
    apk add nodejs npm musl-dev elfutils xz wget pkgconfig libressl-dev perl make && \
    wget https://github.com/upx/upx/releases/download/v4.0.2/upx-4.0.2-amd64_linux.tar.xz && \
    unxz upx-4.0.2-amd64_linux.tar.xz && tar xvf upx-4.0.2-amd64_linux.tar && \
    cp upx-4.0.2-amd64_linux/upx /usr/bin/upx && chmod +x /usr/bin/upx

COPY . /build
COPY --from=frontend-build /build/build/ /build/static/

RUN cargo test && \
    cargo build --release && \
    eu-elfcompress target/release/mail-utilities-wizard && \
    strip target/release/mail-utilities-wizard && \
    upx -9 --lzma target/release/mail-utilities-wizard && \
    chmod +x target/release/mail-utilities-wizard

FROM alpine:3.18.3

WORKDIR /app

RUN apk add libressl-dev && \
    adduser -h /app -D mail-utilities-wizard && \
    chmod 700 /app && \
    chown -R mail-utilities-wizard: /app

COPY --from=app-build /build/config.yml-dist /app/config.yml
COPY --from=app-build /build/target/release/mail-utilities-wizard /app/mail-utilities-wizard

RUN chown -R mail-utilities-wizard: /app && chmod +x /app/mail-utilities-wizard

USER mail-utilities-wizard

CMD ["/app/mail-utilities-wizard"]