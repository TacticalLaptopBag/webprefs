# Build
FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev sqlite-dev sqlite-static
WORKDIR /app
COPY . .
RUN cargo install diesel_cli --no-default-features --features sqlite
RUN rm -f /app/default.db
ENV DATABASE_URL=/app/default.db
RUN diesel migration run
RUN cargo build --release

# Runtime
FROM alpine:latest

EXPOSE 80
VOLUME /app/data

RUN apk add --no-cache libgcc sqlite-libs
WORKDIR /app
COPY --from=builder /app/target/release/webprefs .
COPY --from=builder /app/default.db .
COPY --from=builder /app/entrypoint.sh .
ENV DATABASE_URL=/app/data/webprefs.db
ENV HOST=0.0.0.0
ENV PORT=80
CMD ["./entrypoint.sh"]
