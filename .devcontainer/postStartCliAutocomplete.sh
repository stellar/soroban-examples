#!/bin/bash
set -e

# Add Homebrew to path
test -d ~/.linuxbrew && eval "$(~/.linuxbrew/bin/brew shellenv)" && \
test -d /home/linuxbrew/.linuxbrew && eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)" && \
echo "eval \"\$($(brew --prefix)/bin/brew shellenv)\"" >> ~/.bashrc && \
echo "eval \"\$($(brew --prefix)/bin/brew shellenv)\"" >> ~/.zshrc


# Stellar CLI Auto-Complete
chmod +w ~/.bashrc && \
  echo "source <(stellar completion --shell bash)" >>~/.bashrc && \
  chmod +w ~/.zshrc && \
  echo "source <(stellar completion --shell zsh)" >>~/.zshrc && \
  echo "Enabled Stellar CLI auto-completion"

# Check the exit status and provide informative output
if [ $? -eq 0 ]; then
  echo " ✅ postStartCliAutocomplete.sh executed successfully"
else
  echo " ❌ Error executing postStartCliAutocomplete.sh"
fi
