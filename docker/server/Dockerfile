FROM rust:1.67-buster

RUN useradd -ms /bin/bash rust

RUN mkdir /home/rust/cargo

RUN chown -R rust /home/rust/cargo

RUN sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d -b /usr/local/bin

RUN cargo install cargo-watch && \
    rustup component add rustfmt && \
    rustup component add clippy

ENV CARGO_HOME=/home/rust/cargo

WORKDIR /home/rust/app

USER rust

CMD task start:watch
