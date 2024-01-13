# Based on:
# https://github.com/linux-china/axum-demo/blob/master/Dockerfile

# Use Rust image with specific version
ARG RUST_VERSION=1.74
FROM rust:${RUST_VERSION} as base

# Build the project with necessary files
FROM base AS builder
ARG PROJECT
WORKDIR /app

# Copy required files from monorepo
COPY ./apps/${PROJECT} ./apps/${PROJECT}
COPY ./crates ./crates

# Build the project
WORKDIR /app/apps/${PROJECT}
RUN cargo build --bin server --release

# Create a lean production image
# https://github.com/rust-lang/docker-rust/issues/105#issuecomment-1149081766
FROM debian:bookworm AS runner
ARG PROJECT
ENV APP_USER=appuser
ENV APP_BINARY=dyn_${PROJECT}

# Add non-root user for security
RUN groupadd ${APP_USER}
RUN useradd -g ${APP_USER} ${APP_USER}
USER appuser

# Copy built application and set working directory
WORKDIR /app
COPY --from=builder --chown=${APP_USER}:${APP_USER} /app/apps/${PROJECT}/target/release/${APP_BINARY} ./app_binary

ARG PORT=8080
ENV APP_PORT=${PORT}
EXPOSE ${PORT}

# Command to run the application
CMD ["./app_binary"]