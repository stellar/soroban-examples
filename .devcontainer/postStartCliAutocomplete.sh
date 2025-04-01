#!/bin/bash
set -e

# Stellar CLI Auto-Complete
echo "source <(stellar completion --shell bash)" >>"${remoteEnv:HOME}"/.bashrc
echo "source <(stellar completion --shell bash)" >>"${remoteEnv:HOME}"/.zshrc
echo "Enabled Stellar CLI auto-completion"

echo "Installed Stellar CLI"

# Check the exit status and provide informative output
if [ $? -eq 0 ]; then
  echo " ✅ postStartCliAutocomplete.sh executed successfully"
else
  echo " ❌ Error executing postStartCliAutocomplete.sh"
fi
