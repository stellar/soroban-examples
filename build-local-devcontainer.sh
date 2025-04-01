#!/bin/bash

# Define devcontainer configuration directory
DEVCONTAINER_DIR=".devcontainer"

# Define configuration file path
CONFIG_FILE="devcontainer.json"

# Prebuild image on dockerhub
# https://hub.docker.com/repository/docker/chrisstellar/vsc-soroban-examples-95cce9-prebuild/general
PRE_BUILD_IMAGE="chrisstellar/vsc-soroban-examples-95cce9-prebuild"

BUILD_DETAILS_DIR="z-dc-build-info/"
BUILD_DETAILS_FILE="build-details.json"

if [ ! -e "${BUILD_DETAILS_DIR}" ]; then
  mkdir -p "${BUILD_DETAILS_DIR}"
fi

if [ ! -e "${BUILD_DETAILS_DIR}${BUILD_DETAILS_FILE}" ]; then
  touch "${BUILD_DETAILS_DIR}${BUILD_DETAILS_FILE}"
fi

# Build the devcontainer
output=$(devcontainer build \
  --workspace-folder . \
  --config $DEVCONTAINER_DIR/$CONFIG_FILE \
  --cache-from $PRE_BUILD_IMAGE:latest)

#--dotfiles-repository

# Check the exit status and push pre-build
if [ $? -eq 0 ]; then
  echo " ✅ Devcontainer built successfully"

  # Extract imageName from JSON output using jq
  image_name=$(echo "$output" | jq -r '.imageName[0]')
  echo " 🔹 Image name: ${image_name}"
  docker inspect "${image_name}" >>"${BUILD_DETAILS_DIR}${BUILD_DETAILS_FILE}"

  # Push new pre-build
  docker tag "${image_name}":latest "${PRE_BUILD_IMAGE}":latest
  docker push "${PRE_BUILD_IMAGE}":latest
  echo " 🛠️ New prebuild pushed ${PRE_BUILD_IMAGE}:latest"
  echo " ⚙️ Build info available at ${BUILD_DETAILS_DIR}${BUILD_DETAILS_FILE}"

  echo 'Y' | docker image prune

else
  echo " ❌ Error building devcontainer. Please check logs above."
  exit 1
fi
