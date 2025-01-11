FROM rust:1.84.0-alpine3.21 AS builder

RUN apk add --no-cache musl-dev
COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir ./src \
  && echo 'fn main() {}' > ./src/main.rs \
  && cargo build --release --locked \
  && rm -rf ./src

COPY ./src ./src
COPY ./templates ./templates
RUN touch ./src/main.rs && cargo build --release --frozen

FROM alpine:3.21 AS runner

WORKDIR /opt/app
COPY --from=builder /target/release/proci /opt/app/
EXPOSE 3000
ENTRYPOINT ["/opt/app/proci"]
