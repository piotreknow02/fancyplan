FROM alpine:latest AS base
WORKDIR /app/

FROM rust:1-alpine AS build
WORKDIR /src/
RUN apk add musl-dev libstdc++ libressl-dev
# Caching cargo workarround 
# Tak a look at https://stackoverflow.com/questions/58473606/cache-rust-dependencies-with-docker-build
RUN echo "fn main() {}" >> dummy.rs
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
RUN rm dummy.rs
COPY . .
RUN cargo build --release

FROM base AS final
WORKDIR /app/
COPY --from=build /src/target/release/fancyplan /app/fancyplan
CMD [ "/app/fancyplan" ]