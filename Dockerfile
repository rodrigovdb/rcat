# Use a Rust base image with Cargo installed
FROM rust:latest AS builder

ARG APP_HOME=/usr/src/app
# Set the working directory inside the container
WORKDIR $APP_HOME

# Now copy the source code
COPY . .

# Build the dependencies without the actual source code to cache dependencies separately
RUN cargo build

# Make the executble available in the final image, if it does not exist.
RUN [ ! -e /usr/bin/rcat ] && ln -s ${APP_HOME}/target/debug/rcat /usr/bin/rcat || true

CMD ["rcat", "-h"]
