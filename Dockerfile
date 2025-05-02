# --- Stage 1: Rust build (wasm-pack) ---
FROM rust:slim AS builder-rust

# Install wasm-pack
RUN cargo install wasm-pack

# Set working directory and copy Rust source
WORKDIR /app/lib
COPY ./lib .

# Compile to WebAssembly
ENV RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
RUN wasm-pack build --target web

# --- Stage 2: Node.js build ---
FROM node:lts-slim AS builder-node

# Set working directory and copy frontend source
WORKDIR /app/www
COPY ./www .

# Copy compiled wasm package from Rust builder
COPY --from=builder-rust /app/lib/pkg /app/lib/pkg

# Install dependencies and build frontend
ENV PUBLIC_PATH=/
RUN npm ci && npm run build

# --- Stage 3: Minimal runtime image ---
FROM busybox:stable

# Use non-root user
USER www-data

# Copy built frontend to serving directory
COPY --from=builder-node --chown=www-data:www-data /app/www/dist/spa /var/www/html/

# Start HTTP server
ENTRYPOINT ["/bin/httpd"]
CMD ["-f", "-p", "8080", "-h", "/var/www/html"]
