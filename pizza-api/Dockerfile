FROM rust AS build
WORKDIR /tmp/app

ADD Cargo.toml Cargo.lock ./
ADD src/ ./src

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build /tmp/app/target/release/pizza-api ./pizza-api
ENV PORT=8080
CMD ["./pizza-api"]