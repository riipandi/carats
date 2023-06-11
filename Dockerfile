# syntax=docker/dockerfile:1

# -----------------------------------------------------------------------------
# Sync version information and build the frontend
# -----------------------------------------------------------------------------
FROM node:20-alpine as base
WORKDIR /app
COPY --chown=node:node . .
RUN apk update && apk add --no-cache jq
RUN corepack enable && corepack prepare pnpm@latest --activate
RUN export PKG_WEB_VERSION=$(cat package.json | jq -r .version) &&\
    export APP_VERSION=$(sed -nE 's/^\s*version = "(.*?)"/\1/p' Cargo.toml) &&\
    sed -i "s/\"version\": \"$PKG_WEB_VERSION\"/\"version\": \"$APP_VERSION\"/" package.json
RUN pnpm install && pnpm build

# -----------------------------------------------------------------------------
# Build main application: https://endler.dev/2020/rust-compile-times
# -----------------------------------------------------------------------------
FROM rust:1.70-alpine AS builder
RUN apk update && apk add --no-cache musl-dev libc-dev && update-ca-certificates
WORKDIR /app
COPY --from=base /app /app
RUN cargo check --all --bins --release --locked
RUN cargo build --release --locked --frozen
RUN strip -s /app/target/release/carats

# -----------------------------------------------------------------------------
# Final image: https://kerkour.com/rust-small-docker-image
# -----------------------------------------------------------------------------
LABEL org.opencontainers.image.source "https://github.com/riipandi/carats"
LABEL org.opencontainers.image.description "This is a starter Rust and React project."
FROM alpine:3.18 as runner

ARG DATABASE_URL
ARG DATABASE_AUTO_MIGRATE
ARG JWT_SECRET_KEY
ARG DISABLE_UI

ENV DATABASE_URL $DATABASE_URL
ENV DATABASE_AUTO_MIGRATE $DATABASE_AUTO_MIGRATE
ENV JWT_SECRET_KEY $JWT_SECRET_KEY
ENV DISABLE_UI $DISABLE_UI

# Prepare environment and create non-root user
RUN adduser --disabled-password --gecos "" \
  --home "/nonexistent" --shell "/sbin/nologin" \
  --no-create-home --uid 10001 nonroot

# Import compiled binaries from builder
COPY --from=builder --chown=nonroot:nonroot /app/target/release/carats /usr/local/bin/carats

USER nonroot:nonroot
EXPOSE 9090

ENTRYPOINT ["/usr/local/bin/carats", "--port=9090"]
