# Use the official Rust image as a builder
FROM rust:bullseye as builder

# Create a new empty shell project
WORKDIR /x-feed


# Copy Cargo files
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./migration ./migration
COPY ./entity ./entity

# Create fake main.rs file in src and build
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN cargo build --release

# Copy source files over
RUN rm -rf ./src
COPY ./src ./src

# The last modified attribute of main.rs needs to be updated manually,
# otherwise cargo won't rebuild it.
RUN touch -a -m ./src/main.rs
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