#!/bin/bash
set -e

# Stellar CLI from source
#cargo install --locked stellar-cli --features opt

echo "Installed Stellar CLI"

# Check the exit status and provide informative output
if [ $? -eq 0 ]; then
  echo "✅ updateContentStellarCli() executed successfully"
else
  echo "❌ Error executing updateContentStellarCli() "
fi
