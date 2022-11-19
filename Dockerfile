FROM rust:1.65 as rust-builder
WORKDIR /usr/src/gt_bot

# Copy cargo
COPY ./Cargo.toml .
COPY ./Cargo.lock .

# Create fake main.rs file in src and build for dependencies
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN cargo build --release

# Copy source files over
RUN rm -rf ./src
COPY ./src ./src

# The last modified attribute of main.rs needs to be updated manually,
# otherwise cargo won't rebuild it.
RUN touch -a -m ./src/main.rs
RUN cargo build --release

FROM debian:buster-slim
COPY --from=rust-builder /usr/src/gt_bot/target/release/gt_bot /usr/local/bin/
WORKDIR /usr/local/bin
CMD ["gt_bot"]