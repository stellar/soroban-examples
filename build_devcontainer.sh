#!/bin/bash

# Define devcontainer configuration directory
devcontainer_dir=".devcontainer"

# Define configuration file path
config_file="devcontainer.json"

# Pass in values as parameters
# Example:
# ./build_devcontainer.sh stellar/vsc-soroban-examples-prebuild \
# stellar/vsc-soroban-examples-oci-prebuild ~/cache

pre_build_image=$1
oci_pre_build_image=$2
local_build_cache=$3

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
  --cache-from type=registry,ref="${pre_build_image}" \
  --cache-from type=local,src="${local_build_cache}",mode=max \
  --cache-to type=local,dest="${local_build_cache}",mode=max,oci-mediatypes=true,image-manifest=true \
  --output type=image,name="${pre_build_image}")

echo " ✅ Devcontainer built"
# Extract imageName from JSON output using jq
image_name=$(echo "$output" | jq -r '.imageName[0]')
echo " 🔹 Image name: ${image_name}"
docker inspect "${image_name}" >> "${build_details_dir}${build_details_file}"

# Push new pre-build
docker tag "${image_name}":latest "${pre_build_image}":latest
docker push "${pre_build_image}":latest
echo " 🛠️ New prebuild pushed ${pre_build_image}:latest"
echo " ⚙️ Build info available at ${build_details_dir}${build_details_file}"
echo 'Y' | docker image prune

# Build the devcontainer again with OCI Output
oci_output=$(devcontainer build \
  --workspace-folder . \
  --config $devcontainer_dir/$config_file \
  --cache-from type=registry,ref="${pre_build_image}" \
  --cache-from type=registry,ref="${oci_pre_build_image}" \
  --cache-to type=registry,ref="${oci_pre_build_image}",mode=max,oci-artifact=true \
  --output type=image,name="${oci_pre_build_image}",mode=max,oci-mediatypes=true,compression=zstd)

# Extract ociImageName from JSON output using jq
oci_image_name=$(echo "$oci_output" | jq -r '.imageName[0]')

# Push new OCI pre-build
docker tag "${oci_image_name}":latest "${oci_pre_build_image}":latest
docker push "${oci_pre_build_image}":latest

echo 'Y' | docker image prune
