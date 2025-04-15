#!/bin/bash
set -e
# Define devcontainer configuration directory
devcontainer_dir=".devcontainer"

# Define configuration file path
config_file="devcontainer.json"

# Build the devcontainer
devcontainer build \
  --workspace-folder . \
  --config $devcontainer_dir/$config_file



