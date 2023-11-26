# syntax=docker/dockerfile:1.4
FROM rust:buster AS base

ENV USER=root
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV=development

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch && \
    cargo install cargo-watch
COPY . /code

FROM base AS development

EXPOSE 8000

CMD ["cargo", "watch", "-x", "run"]
