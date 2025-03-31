#!/bin/bash

# Define devcontainer configuration directory
DEVCONTAINER_DIR=".devcontainer"

# Define configuration file path
CONFIG_FILE="devcontainer.json"

# Prebuild image on dockerhub https://hub.docker.com/repository/docker/chrisstellar/vsc-soroban-quest-b12b9-prebuild/general
PRE_BUILD_IMAGE="chrisstellar/vsc-soroban-examples-95cce9-prebuild"

BUILD_DETAILS="z-dc-build-info/build-details.json"

# Build the devcontainer
output=$(devcontainer build \
  --workspace-folder . \
  --config $DEVCONTAINER_DIR/$CONFIG_FILE \
  --log-level "debug" \
  --cache-from $PRE_BUILD_IMAGE:latest)

#--dotfiles-repository

# Check the exit status and provide informative output
if [ $? -eq 0 ]; then
  echo " ✅ Devcontainer built successfully"

  # Extract imageName from JSON output using jq
  image_name=$(echo "$output" | jq -r '.imageName[0]')
  echo "🔹 Image name: $image_name"
  docker inspect "$image_name" >>$BUILD_DETAILS

  # Push new pre-build
  docker tag "$image_name":latest $PRE_BUILD_IMAGE:latest
  docker push $PRE_BUILD_IMAGE:latest
  echo " 🛠️ New prebuild pushed ${PRE_BUILD_IMAGE}:latest"

  echo 'Y' | docker image prune

else
  echo " ❌ Error building devcontainer. Please check logs above."
  exit 1
fi
