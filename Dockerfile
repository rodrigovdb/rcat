# Use a Rust base image with Cargo installed
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
# COPY Cargo.toml Cargo.lock ./

# Create an empty src directory to trick Cargo into thinking it's a valid Rust project
# RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies without the actual source code to cache dependencies separately
# RUN cargo build --release

# Now copy the source code
COPY . .

# Build your application
# RUN cargo build --release

RUN rustc hello_world.rs

CMD ["./hello_world"]
