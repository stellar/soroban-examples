#!/bin/bash

# Define devcontainer configuration directory
DEVCONTAINER_DIR=".devcontainer"

# Define configuration file path
CONFIG_FILE="devcontainer.json"

# Use env vars or enter in your own values here.
###

# Example
# - buildpack-deps:bookworm
# - buildpack-deps:bookworm2
# - /User/cache


PRE_BUILD_IMAGE=$1
OCI_PRE_BUILD_IMAGE=$2
LOCAL_BUILD_CACHE=$3

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
  --cache-from $PRE_BUILD_IMAGE:latest \
  --cache-from type=local,src=${LOCAL_BUILD_CACHE},mode=max \
  --cache-to type=local,dest=${LOCAL_BUILD_CACHE},mode=max,oci-mediatypes=true,image-manifest=true \
  --output type=image,name="${PRE_BUILD_IMAGE}:later");

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

# Build the devcontainer again with OCI Output
oci_output=$(devcontainer build \
  --workspace-folder . \
  --config $DEVCONTAINER_DIR/$CONFIG_FILE \
  --cache-from $PRE_BUILD_IMAGE:latest \
  --cache-from "${OCI_PRE_BUILD_IMAGE}:latest" \
  --cache-to type=registry,ref="${OCI_PRE_BUILD_IMAGE}:latest",mode=max,oci-artifact=true \
  --output type=image,name="${OCI_PRE_BUILD_IMAGE}:latest",mode=max,oci-mediatypes=true,compression=zstd)

# Extract ociImageName from JSON output using jq
oci_image_name=$(echo "$oci_output" | jq -r '.imageName[0]')

# Push new OCI pre-build
docker tag "${oci_image_name}":latest "${OCI_PRE_BUILD_IMAGE}":latest
docker push "${OCI_PRE_BUILD_IMAGE}":latest

echo 'Y' | docker image prune
