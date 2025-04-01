#!/bin/bash
set -e

# Stellar CLI Auto-Complete
chmod +w "${remoteEnv:VSCODE_HOME}/.bashrc" && \
  echo "source <(stellar completion --shell bash)" >>"${remoteEnv:VSCODE_HOME}/.bashrc" && \
  chmod +w "${remoteEnv:VSCODE_HOME}/.zshrc" && \
  echo "source <(stellar completion --shell bash)" >>"${remoteEnv:VSCODE_HOME}/.zshrc" && \
  echo "Enabled Stellar CLI auto-completion"

# Check the exit status and provide informative output
if [ $? -eq 0 ]; then
  echo " ✅ postStartCliAutocomplete.sh executed successfully"
else
  echo " ❌ Error executing postStartCliAutocomplete.sh"
fi
