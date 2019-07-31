FROM rustlang/rust:nightly-slim as build
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM rustlang/rust:nightly-slim
WORKDIR /usr/src/app
COPY --from=build /usr/src/app/target/release/stats-api .
CMD ["./stats-api"]
