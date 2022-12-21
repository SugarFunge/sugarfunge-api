FROM rustlang/rust:nightly AS builder

WORKDIR /sugarfunge-api

COPY . /sugarfunge-api

RUN cargo build --locked --release

FROM gcr.io/distroless/cc-debian11

COPY --from=builder /sugarfunge-api/target/release/sugarfunge-api /

ENTRYPOINT ["/sugarfunge-api"]
