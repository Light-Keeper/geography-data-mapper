FROM rust:1.68.1-alpine as builder
RUN apk update
RUN apk add libc-dev
RUN cargo init app
ADD app/Cargo.toml /app
ADD app/Cargo.lock /app
WORKDIR /app
RUN cargo build --release
RUN rm -rf src/*
COPY app/ ./
RUN touch /app/src/main.rs
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/geography-data-mapper /app
COPY frontend/out /app/frontend
ENV STATIC_FILES_DIR=/app/frontend
ENV SQLITE_DB_PATH=/data/geomap.db
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["/app/geography-data-mapper"]
