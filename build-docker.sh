#!/usr/bin/env bash
set -euo pipefail

DOCKER_IMAGE=sunside/crashie
DOCKER_URL="https://hub.docker.com/r/$DOCKER_IMAGE"

BUILD_DATE=$(date --rfc-3339=seconds | sed 's/ /T/')
GIT_COMMIT_HASH=$(git rev-parse HEAD)
APP_VERSION=$(sed -n 's/^version = "\(\S*\)\"$/\1/p' Cargo.toml)

docker build --progress=plain --tag "$DOCKER_IMAGE" \
  --label org.opencontainers.image.base.name="$DOCKER_IMAGE" \
  --label org.opencontainers.image.url="$DOCKER_URL" \
  --label org.opencontainers.artifact.created="$BUILD_DATE" \
  --label org.opencontainers.image.created="$BUILD_DATE" \
  --label org.opencontainers.image.authors="Markus Mayer" \
  --label org.opencontainers.image.revision="$GIT_COMMIT_HASH" \
  --label org.opencontainers.image.version="$APP_VERSION" \
  -f Dockerfile .

echo "Tagging image $DOCKER_IMAGE:latest as $DOCKER_IMAGE:$APP_VERSION"
docker tag "$DOCKER_IMAGE:latest" "$DOCKER_IMAGE:$APP_VERSION"

echo "Inspect labels using docker inspect $DOCKER_IMAGE:$APP_VERSION"
