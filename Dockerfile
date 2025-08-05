# Step 1: Build backend (Rust)
FROM rust:1.74 as backend-builder
WORKDIR /app
COPY backend ./backend
WORKDIR /app/backend
RUN cargo build --release

# Step 2: Build frontend (React)
FROM node:18 as frontend-builder
WORKDIR /app
COPY frontend ./frontend
WORKDIR /app/frontend
RUN npm install && npm run build

# Step 3: Final image
FROM debian:bookworm-slim

# Copy backend binary
COPY --from=backend-builder /app/backend/target/release/backend /usr/local/bin/backend

# Copy frontend build to serve as static files (optional)
COPY --from=frontend-builder /app/frontend/dist /var/www/frontend

# Install a simple static file server (e.g. using Python or nginx if needed)
RUN apt-get update && apt-get install -y python3 && apt-get clean

EXPOSE 3001

CMD ["/usr/local/bin/backend"]
