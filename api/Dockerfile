FROM rust:latest

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

# Cargo needs a main function, https://github.com/rust-lang/cargo/issues/2644
RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY . .

RUN cargo build --release

EXPOSE 3001

ENTRYPOINT ["/app/target/release/api"]
