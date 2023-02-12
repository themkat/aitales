FROM rust:slim-bullseye AS build
ADD . /work
RUN apt-get update && apt-get install -y libssl-dev pkg-config && \
    cd work && cargo build --release


FROM debian:bullseye-slim
COPY --from=build /work/target/release/story_generator /story_generator
ADD generator_conf.yml /generator_conf.yml

# needs updated certificates
RUN apt-get update && apt-get install -y ca-certificates wget git gh && apt-get clean

ENTRYPOINT /bin/sh
