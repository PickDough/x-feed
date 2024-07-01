# Use the official Rust image as a builder
FROM rust:bullseye as builder

# Create a new empty shell project
WORKDIR /x-feed


# Copy Cargo files
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./migration ./migration
COPY ./entity ./entity
COPY ./messaging ./messaging
COPY ./model ./model
COPY ./persistance ./persistance

COPY ./src ./src

RUN cargo build --release

# Our final base
FROM debian:bullseye

# Copy the build artifact from the build stage
COPY --from=builder /x-feed/target/release/ .

RUN apt-get update && apt-get upgrade -y
RUN apt-get install libssl-dev

RUN apt-get install -y -q build-essential curl