# Build
FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev sqlite-dev sqlite-static nodejs npm
WORKDIR /app
COPY . .
RUN cargo install diesel_cli --no-default-features --features sqlite
RUN rm -f /app/default.db
ENV DATABASE_URL=/app/default.db
RUN diesel migration run
RUN cargo build --release

# Web client build
WORKDIR /app/web-client
RUN npm run build
WORKDIR /app

# Runtime
FROM alpine:latest

EXPOSE 80
VOLUME /app/data
VOLUME /app/web

RUN apk add --no-cache libgcc sqlite-libs
WORKDIR /app
COPY --from=builder /app/web-client/dist/web-client/browser ./web
COPY --from=builder /app/target/release/webprefs .
COPY --from=builder /app/default.db .
COPY --from=builder /app/entrypoint.sh .
ENV DATABASE_URL=/app/data/webprefs.db
ENV HOST=0.0.0.0
ENV PORT=80
ENV APP_SERVE_PATH=/app/web
CMD ["./entrypoint.sh"]
