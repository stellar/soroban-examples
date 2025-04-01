#!/bin/bash
set -e

# Make .bashrc and .zshrc writable
chmod +w "${remoteEnv:HOME}/.bashrc" &&
  echo -e "\n# Rust sccache settings" >>"${remoteEnv:HOME}/.bashrc" &&
  chmod +w "${remoteEnv:HOME}/.zshrc" &&
  echo -e "\n# Rust sccache settings" >>"${remoteEnv:HOME}/.zshrc"

# Loop and export each env var for current session
# Append env vars to .zshrc and  for persistent usage
echo "🎯 Exporting environment variables:"
for env_var in "${ENV_VARS[@]}"; do
  eval "$env_var" && echo "🔹 $env_var" &&
    echo "$env_var" >>"${remoteEnv:HOME}/.bashrc" &&
    echo "$env_var" >>"${remoteEnv:HOME}/.zshrc"
done
