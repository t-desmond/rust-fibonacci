FROM rust:alpine AS build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=build /app/target/release/fabinnoci /app/fabinnoci

CMD ["/app/fabinnoci"]
