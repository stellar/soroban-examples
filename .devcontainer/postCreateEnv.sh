#!/bin/bash
set -e

# Make .bashrc and .zshrc writable
chmod +w "${remoteEnv:VSCODE_HOME}/.bashrc" &&
  echo -e "\n# Rust sccache settings" >>"${remoteEnv:VSCODE_HOME}/.bashrc" &&
  chmod +w "${remoteEnv:VSCODE_HOME}/.zshrc" &&
  echo -e "\n# Rust sccache settings" >>"${remoteEnv:VSCODE_HOME}/.zshrc"

# Loop and export each env var for current session
# Append env vars to .zshrc and  for persistent usage
echo "🎯 Exporting environment variables:"
for env_var in "${ENV_VARS[@]}"; do
  eval "$env_var" && echo "🔹 $env_var" &&
    echo "$env_var" >>"${remoteEnv:VSCODE_HOME}/.bashrc" &&
    echo "$env_var" >>"${remoteEnv:VSCODE_HOME}/.zshrc"
done
