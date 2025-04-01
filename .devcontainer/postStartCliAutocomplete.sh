#!/bin/bash
set -e

# Stellar CLI Auto-Complete
chmod +w "${remoteEnv:HOME}/.bashrc" && \
  echo "source <(stellar completion --shell bash)" >>"${remoteEnv:HOME}/.bashrc" && \
  chmod +w "${remoteEnv:HOME}/.zshrc" && \
  echo "source <(stellar completion --shell bash)" >>"${remoteEnv:HOME}/.zshrc" && \
  echo "Enabled Stellar CLI auto-completion"

# Check the exit status and provide informative output
if [ $? -eq 0 ]; then
  echo " ✅ postStartCliAutocomplete.sh executed successfully"
else
  echo " ❌ Error executing postStartCliAutocomplete.sh"
fi
