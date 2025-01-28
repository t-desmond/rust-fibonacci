FROM rust:alpine AS build

WORKDIR /app

COPY . .

RUN cargo build

FROM debian:bookworm-slim

COPY --from=build /app/target/debug/fabinnoci /app/fabinnoci

CMD ["/app/fabinnoci"]
