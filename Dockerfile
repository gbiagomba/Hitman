# Use the official Rust image as the base image
FROM rust:1.78 as builder

# Set the working directory
WORKDIR /app

# Copy the project files to the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Use a minimal base image for running the application
FROM debian:buster-slim

# Copy the compiled binary from the builder image
COPY --from=builder /app/target/release/hitman /usr/local/bin/hitman

# Set the default command to run the application
CMD ["hitman"]
