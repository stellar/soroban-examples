#!/bin/bash

# Define devcontainer configuration directory
devcontainer_dir=".devcontainer"

# Define configuration file path
config_file="devcontainer.json"

# Use env vars or enter in your own values here.
###

# Example
# - buildpack-deps:bookworm
# - buildpack-deps:bookworm2
# - /User/cache

pre_build_image=$1
oci_pre_build_image=$2

build_details_dir="z-dc-build-info/"
build_details_file="build-details.json"

if [ ! -e "${build_details_dir}" ]; then
  mkdir -p "${build_details_dir}"
fi

if [ ! -e "${build_details_dir}${build_details_file}" ]; then
  touch "${build_details_dir}${build_details_file}"
fi

# Build the devcontainer
output=$(devcontainer build \
  --workspace-folder . \
  --config $devcontainer_dir/$config_file \
  --cache-from "$pre_build_image":latest \
  --output type=image,name="${pre_build_image}:later")

#--dotfiles-repository

# Check the exit status and push pre-build
if mycmd; then
  echo " ✅ Devcontainer built successfully"

  # Extract imageName from JSON output using jq
  image_name=$(echo "$output" | jq -r '.imageName[0]')
  echo " 🔹 Image name: ${image_name}"
  docker inspect "${image_name}" >> "${build_details_dir}${build_details_file}"

  # Push new pre-build
  docker tag "${image_name}":latest "${pre_build_image}":latest
  docker push "${pre_build_image}":latest

  echo " 🛠️ New prebuild pushed ${pre_build_image}:latest"
  echo " ⚙️ Build info available at ${build_details_dir}${build_details_file}"

else
  echo " ❌ Error building devcontainer. Please check logs above."
  exit 1
fi

# Build the devcontainer again with OCI Output
oci_output=$(devcontainer build \
  --workspace-folder . \
  --config $devcontainer_dir/$config_file \
  --cache-from "$pre_build_image":latest \
  --cache-from "${oci_pre_build_image}:latest" \
  --cache-to type=registry,ref="${oci_pre_build_image}:latest",mode=max,oci-artifact=true \
  --output type=image,name="${oci_pre_build_image}:latest",mode=max,oci-mediatypes=true,compression=zstd)

# Extract ociImageName from JSON output using jq
oci_image_name=$(echo "$oci_output" | jq -r '.imageName[0]')

# Push new OCI pre-build
docker tag "${oci_image_name}":latest "${oci_pre_build_image}":latest
docker push "${oci_pre_build_image}":latest
