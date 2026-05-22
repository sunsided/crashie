#!/usr/bin/env bash
set -euo pipefail

DOCKER_IMAGE=sunside/crashie
DOCKER_URL="https://hub.docker.com/r/$DOCKER_IMAGE"
PLATFORMS="${PLATFORMS:-linux/amd64,linux/arm64}"
PUSH="${PUSH:-0}"

BUILD_DATE=$(date --rfc-3339=seconds | sed 's/ /T/')
GIT_COMMIT_HASH=$(git rev-parse HEAD)
APP_VERSION=$(sed -n 's/^version = "\(\S*\)\"$/\1/p' Cargo.toml)

# Multi-arch builds require a buildx builder with QEMU emulation. Create one if
# the default doesn't already exist.
if ! docker buildx inspect crashie-builder >/dev/null 2>&1; then
  echo "Creating buildx builder 'crashie-builder' for platforms: $PLATFORMS"
  docker buildx create --name crashie-builder --driver docker-container --use >/dev/null
  docker buildx inspect --bootstrap >/dev/null
else
  docker buildx use crashie-builder >/dev/null
fi

# Multi-arch images can only be pushed (or saved to a tar); --load works for a
# single platform only. When PUSH=0 we fall back to the host platform so the
# resulting image is usable locally.
if [[ "$PUSH" == "1" ]]; then
  OUTPUT_FLAGS=(--push)
else
  OUTPUT_FLAGS=(--load)
  if [[ "$PLATFORMS" == *","* ]]; then
    echo "PUSH=0: --load only supports one platform; building for the host platform only."
    PLATFORMS=""
  fi
fi

BUILD_ARGS=(
  --progress=plain
  --label "org.opencontainers.image.base.name=$DOCKER_IMAGE"
  --label "org.opencontainers.image.url=$DOCKER_URL"
  --label "org.opencontainers.artifact.created=$BUILD_DATE"
  --label "org.opencontainers.image.created=$BUILD_DATE"
  --label "org.opencontainers.image.authors=Markus Mayer"
  --label "org.opencontainers.image.revision=$GIT_COMMIT_HASH"
  --label "org.opencontainers.image.version=$APP_VERSION"
  --tag "$DOCKER_IMAGE:latest"
  --tag "$DOCKER_IMAGE:$APP_VERSION"
  "${OUTPUT_FLAGS[@]}"
)

if [[ -n "$PLATFORMS" ]]; then
  BUILD_ARGS+=(--platform "$PLATFORMS")
fi

docker buildx build "${BUILD_ARGS[@]}" -f Dockerfile .

echo "Built $DOCKER_IMAGE:$APP_VERSION (also tagged as :latest)"
if [[ "$PUSH" == "1" ]]; then
  echo "Pushed to $DOCKER_URL"
else
  echo "Inspect labels using: docker inspect $DOCKER_IMAGE:$APP_VERSION"
  echo "Run 'PUSH=1 $0' to publish multi-arch images to Docker Hub."
fi
