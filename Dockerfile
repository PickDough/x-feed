# Use the official Rust image as a builder
FROM rust:bullseye as builder

# Create a new empty shell project
WORKDIR /x-feed

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

COPY ./src ./src

# Now that the dependencies are built, copy your source code


# Build for release.
RUN cargo build --release --bin x-feed

# Our final base
FROM debian:bullseye

# Copy the build artifact from the build stage
COPY --from=builder /x-feed/target/release/x-feed .

RUN apt-get update && apt-get upgrade -y
RUN apt-get install libssl-dev

RUN apt-get install -y -q build-essential curl

# Set the binary as the entrypoint of the container
ENTRYPOINT ["./x-feed"]