# Stage 1: Build the Rust application
FROM rust:1.80 as builder

# Install dependencies for building, including libclang
RUN apt-get update && apt-get install -y \
    clang \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs file to allow the dependencies to be cached
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

# Build the dependencies and cache them
RUN cargo build --release

# Remove the dummy main.rs
RUN rm src/main.rs

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Debugging step: Show directory contents
RUN ls -la /
RUN ls -la /app/target/release

# Stage 2: Create the final image with the built application
FROM debian:bookworm-slim

# Set the working directory inside the container
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/random_words_api_rust /app

# Expose the port the app will run on
EXPOSE 3000

# Ensure the binary is executable
RUN chmod +x /app/random_words_api_rust

# Set the command to run the application
CMD ["./random_words_api_rust"]

