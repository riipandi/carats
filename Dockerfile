# syntax=docker/dockerfile:1

# -----------------------------------------------------------------------------
# Sync version information and build the frontend
# -----------------------------------------------------------------------------
FROM node:20-alpine as base
WORKDIR /app
COPY --chown=node:node . .
RUN apk update && apk add --no-cache --update-cache jq
RUN export PKG_WEB_VERSION=$(cat package.json | jq -r .version) &&\
    export APP_VERSION=$(sed -nE 's/^\s*version = "(.*?)"/\1/p' Cargo.toml) &&\
    sed -i "s/\"version\": \"$PKG_WEB_VERSION\"/\"version\": \"$APP_VERSION\"/" package.json
RUN npm config set fund false && npm install --no-audit && npm run build

# -----------------------------------------------------------------------------
# Builder main application: https://endler.dev/2020/rust-compile-times
# -----------------------------------------------------------------------------
FROM cgr.dev/chainguard/rust:1.69 AS builder
WORKDIR /app
COPY --chown=$(whoami): --from=base /app /app
RUN cargo build --release --locked --bin funstack

# -----------------------------------------------------------------------------
# Final image: https://kerkour.com/rust-small-docker-image
# -----------------------------------------------------------------------------
LABEL org.opencontainers.image.source "https://github.com/riipandi/funstack"
LABEL org.opencontainers.image.description "This is a starter Rust and React project."
FROM cgr.dev/chainguard/glibc-dynamic:latest as runner

ARG BIND_PORT 9090
ARG BIND_ADDR 0.0.0.0
ARG DATABASE_URL
ARG DATABASE_AUTO_MIGRATE
ARG JWT_SECRET_KEY
ARG DISABLE_UI

ENV BIND_PORT $BIND_PORT
ENV BIND_ADDR $BIND_ADDR
ENV DATABASE_URL $DATABASE_URL
ENV DATABASE_AUTO_MIGRATE $DATABASE_AUTO_MIGRATE
ENV JWT_SECRET_KEY $JWT_SECRET_KEY
ENV DISABLE_UI $DISABLE_UI

# Import compiled binaries from builder
COPY --from=builder /app/target/release/funstack /sbin/funstack

EXPOSE $BIND_PORT

ENTRYPOINT ["/sbin/funstack", "--port", "${BIND_PORT}"]
