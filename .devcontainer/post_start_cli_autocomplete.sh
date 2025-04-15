#!/bin/bash
set -euxo pipefail

# Add Stellar CLI to $PATH
chmod 644 ~/.bashrc && chmod 644 ~/.zshrc

test -d /usr/local/bin/ && echo "/usr/local/bin/" >> ~/.zshrc
test -d /usr/local/bin/ && echo "/usr/local/bin/" >> ~/.bashrc

export PATH="/usr/local/bin/":$PATH
echo "export PATH='/usr/local/bin/:$PATH'" >> ~/.zshrc
echo "export PATH='/usr/local/bin/:$PATH'" >> ~/.bashrc

# Stellar CLI Auto-Complete
chmod 644 ~/.bashrc
echo "source <(stellar completion --shell bash)" >>~/.bashrc
chmod 644 ~/.zshrc
echo "source <(stellar completion --shell zsh)" >>~/.zshrc
echo "Enabled Stellar CLI auto-completion"

echo " ✅ postStartCliAutocomplete.sh executed successfully"

