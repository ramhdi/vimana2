# Stage 1: Build the app using the Rust official image
FROM rust:latest as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo files and source code
COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src ./src
COPY templates ./templates
COPY static ./static

# Install dependencies and build the app in release mode
RUN cargo build --release

# Identify and copy required shared libraries
# RUN mkdir -p /app/libs \
#     && ldd target/release/vimana2 | grep "=> /" | awk '{print $3}' | xargs cp -t /app/libs

# Stage 2: Create a minimal runtime image using distroless
FROM gcr.io/distroless/cc as runtime

# Set working directory in the new image
WORKDIR /app

# Copy the compiled binary and shared libraries from the builder stage
COPY --from=builder /app/target/release/vimana2 /app/vimana2
# COPY --from=builder /app/libs /app/libs
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/static /app/static

# Set the environment variable to include the shared libraries
# ENV LD_LIBRARY_PATH=/app/libs

# Expose the port the application will run on
EXPOSE 8081

# Run the application
CMD ["/app/vimana2"]
