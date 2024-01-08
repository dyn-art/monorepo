# Based on:
# https://fintlabs.medium.com/optimized-multi-stage-docker-builds-with-turborepo-and-pnpm-for-nodejs-microservices-in-a-monorepo-c686fdcf051f
# https://github.com/vercel/turbo/blob/main/examples/with-docker/apps/api/Dockerfile

# Use Alpine image with specific Node.js version
ARG NODE_VERSION=18.18.0
FROM node:${NODE_VERSION}-alpine AS alpine

# Update packages and install libc6-compat for compatibility issues
# https://github.com/nodejs/docker-node/tree/b4117f9333da4138b03a546ec926ef50a31506c3#nodealpine
RUN apk update
RUN apk add --no-cache libc6-compat

# Setup pnpm and turbo on the alpine base
FROM alpine as base
RUN npm install pnpm turbo -g
RUN pnpm config set store-dir ~/.pnpm-store

# Prune projects to include only necessary dependencies
FROM base AS pruner
ARG PROJECT
WORKDIR /app

COPY . .
RUN turbo prune --scope=@dyn/${PROJECT} --docker

# Build the project with necessary files
FROM base AS builder
ARG PROJECT
WORKDIR /app

# Copy pruned lockfile, workspace, and scripts
COPY --from=pruner /app/out/pnpm-lock.yaml ./pnpm-lock.yaml
COPY --from=pruner /app/out/pnpm-workspace.yaml ./pnpm-workspace.yaml
COPY --from=pruner /app/out/json/ ./
COPY --from=pruner /app/scripts ./scripts

# Install dependencies with cache mount for efficiency
RUN --mount=type=cache,id=pnpm,target=~/.pnpm-store pnpm install --frozen-lockfile

# Copy source code of isolated subworkspace and build
COPY --from=pruner /app/out/full/ ./
RUN turbo build --filter=@dyn/cli
RUN turbo build --filter=@dyn/${PROJECT}
RUN --mount=type=cache,id=pnpm,target=~/.pnpm-store pnpm prune --prod --no-optional

# Remove unnecessary files and directories, keeping only dist and node_modules
RUN rm -rf ./packages/cli
RUN rm -rf ./packages/config
RUN rm -rf ./packages/types
RUN rm -rf ./scripts
RUN for dir in apps/*/ packages/*/; do \
    (cd "$dir" && find . -mindepth 1 -maxdepth 1 ! -name 'dist' ! -name 'node_modules' ! -name 'package.json' -exec rm -rf {} +); \
    done

# Create a lean production image on the alpine base
FROM alpine AS runner
ARG PROJECT

# Add non-root user for security
RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nodejs
USER nodejs

# Copy built application and set working directory
WORKDIR /app
COPY --from=builder --chown=nodejs:nodejs /app ./
WORKDIR /app/apps/${PROJECT}

ARG PORT=8080
ENV PORT=${PORT}
ENV NODE_ENV=production
EXPOSE ${PORT}

CMD ["node", "dist/index.js"]
