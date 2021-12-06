# Based on https://github.com/paritytech/polkadot/blob/master/scripts/dockerfiles/polkadot/polkadot_builder.Dockerfile

FROM paritytech/ci-linux:production AS builder
WORKDIR /sugarfunge-api
COPY . /sugarfunge-api
RUN cargo +nightly build --locked --release

FROM ubuntu:20.04

RUN apt-get update && \
  apt-get install --no-install-recommends -y libssl-dev

COPY --from=builder /sugarfunge-api/target/release/sugarfunge-api /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /sugarfunge-api sugarfunge-api && \
	mkdir -p /data /sugarfunge-api/.local/share && \
	chown -R sugarfunge-api:sugarfunge-api /data && \
	ln -s /data /sugarfunge-api/.local/share/sugarfunge-api

USER sugarfunge-api

EXPOSE 4000

ENTRYPOINT ["/usr/local/bin/sugarfunge-api"]
