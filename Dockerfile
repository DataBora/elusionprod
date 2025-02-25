# Use the official Rust image
FROM rust:latest

# Create a new empty shell project
WORKDIR /usr/src/app

# Copy your manifests
COPY Cargo.toml ./

# Copy your source tree
COPY src ./src

# Build the application
RUN cargo build --release

# Run the binary
CMD ["/usr/src/app/target/release/elusionlearning"]