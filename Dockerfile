# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.76.0-nightly
FROM rustlang/rust:nightly AS build

COPY . /app/

WORKDIR /app

RUN cargo install cargo-leptos && \
  rustup target add wasm32-unknown-unknown && \
  cargo leptos build --release && \
  cp ./target/release/rr /bin/server && \
  cp -r ./target/site /app/site

FROM debian:bullseye-slim AS final

RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid 10001 \
  appuser
USER appuser

COPY --from=build /bin/server /bin/
COPY --from=build /app/site /app/site

ENV LEPTOS_OUTPUT_NAME="rr"
ENV LEPTOS_SITE_ROOT="/app/site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_RELOAD_PORT="3001"

EXPOSE 3000 3001

CMD ["/bin/server"]

