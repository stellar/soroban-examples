#!/bin/bash
set -e

pre_build_image=$1
oci_pre_build_image=$2

# Define devcontainer configuration directory
devcontainer_dir=".devcontainer"

# Define configuration file path
config_file="devcontainer.json"

# Build the devcontainer
output=$(devcontainer build \
  --workspace-folder . \
  --config $devcontainer_dir/$config_file \
  --cache-from "$pre_build_image":latest \
  --output type=image,name="${pre_build_image}:latest")

 if [ "$output" ]; then
    image_name=$(echo "$output" | jq -r '.imageName[0]')
    docker tag "${image_name}":latest "${pre_build_image}":latest
    docker push "${pre_build_image}":latest
 else
   echo " ❌ Error building devcontainer. Please check logs above."
   exit 1
 fi

oci_output=$(devcontainer build \
  --workspace-folder . \
  --config $devcontainer_dir/$config_file \
  --cache-from "$pre_build_image":latest \
  --cache-from "${oci_pre_build_image}:latest" \
  --cache-to type=registry,ref="${oci_pre_build_image}:latest",mode=max,oci-artifact=true \
  --output type=image,name="${oci_pre_build_image}:latest",mode=max,oci-mediatypes=true,compression=zstd)

oci_image_name=$(echo "$oci_output" | jq -r '.imageName[0]')

docker tag "${oci_image_name}":latest "${oci_pre_build_image}":latest
docker push "${oci_pre_build_image}":latest
