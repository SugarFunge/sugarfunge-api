# Based on https://github.com/paritytech/polkadot/blob/master/scripts/dockerfiles/polkadot/polkadot_builder.Dockerfile

FROM paritytech/ci-linux:production AS builder
WORKDIR /sugarfunge-api
COPY . /sugarfunge-api
RUN cargo +nightly build --locked --release

FROM gcr.io/distroless/cc-debian11

COPY --from=builder /sugarfunge-api/target/release/sugarfunge-api /

ENTRYPOINT ["/sugarfunge-api"]
