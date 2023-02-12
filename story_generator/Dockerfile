FROM rust:slim-bullseye AS build
ADD . /work
RUN apt-get update && apt-get install -y libssl-dev pkg-config && \
    cd work && cargo build --release


FROM debian:bullseye-slim
COPY --from=build /work/target/release/story_generator /story_generator
ADD generator_conf.yml /generator_conf.yml

# needs updated certificates
# gh needs tons of shit... should probably have distributed the plain exec instead...
RUN apt-get update && type -p curl >/dev/null || apt-get install curl -y && \
    curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg \
    && chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg \
    && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | tee /etc/apt/sources.list.d/github-cli.list > /dev/null && \
    apt-get update && apt-get install -y ca-certificates wget git gh && apt-get clean

ENTRYPOINT /bin/sh
