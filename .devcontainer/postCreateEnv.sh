#!/bin/bash
set -e

mkdir -p "${containerWorkspaceFolder}/.sccache" && chmod +w "${containerWorkspaceFolder}/.sccache"

# Define environment variables
ENV_VARS=(
  'export RUSTC_WRAPPER="sccache"'
  'export SCCACHE_CACHE_SIZE="5G"'
  'export SCCACHE_DIR="/workspace/.sccache"'
)

# Loop and export each env var for current session
echo "🎯 Exporting environment variables:"
for env_var in "${ENV_VARS[@]}"; do
  eval "$env_var"
  echo "🔹 $env_var"
done

# Append env vars to .zshrc and  for persistent usage
chmod +w "${remoteEnv:HOME}/.bashrc" && \
echo -e "\n# Rust sccache settings" >> "${remoteEnv:HOME}/.bashrc" && \
chmod +w "${remoteEnv:HOME}/.zshrc" && \
echo -e "\n# Rust sccache settings" >> "${remoteEnv:HOME}/.zshrc" && \
for env_var in "${ENV_VARS[@]}"; do
  echo "$env_var" >> "${remoteEnv:HOME}/.bashrc" && \
  echo "$env_var" >> "${remoteEnv:HOME}/.zshrc"
done
