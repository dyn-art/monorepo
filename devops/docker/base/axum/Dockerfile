# Based on:
# https://github.com/linux-china/axum-demo/blob/master/Dockerfile

# Use Rust image with specific version
ARG RUST_VERSION=1.72
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
RUN cargo build --release

# Create a lean production image
FROM debian:buster-slim AS runner
ARG PROJECT
ENV APP_USER=appuser
ENV APP_BINARY=dyn_${PROJECT}

# Add non-root user for security
RUN groupadd ${APP_USER}
RUN useradd -g ${APP_USER} ${APP_USER}
USER appuser

# Copy built application and set working directory
WORKDIR /app
COPY --from=builder --chown=${APP_USER}:${APP_USER} /app/apps/${PROJECT}/target/release/${APP_BINARY} ./

ARG PORT=8080
ENV PORT=${PORT}
EXPOSE ${PORT}

# Command to run the application
CMD ["./${APP_BINARY}"]