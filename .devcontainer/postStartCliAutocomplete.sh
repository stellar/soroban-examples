#!/bin/bash
set -e


# Add Brew and Stellar CLI to $PATH and terminals
chmod +w ~/.bashrc && chmod +w ~/.zshrc && \
test -d ~/.linuxbrew && eval "$(~/.linuxbrew/bin/brew shellenv)" && \
test -d /home/linuxbrew/.linuxbrew && eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)" && \
echo "eval $($(brew --prefix)/bin/brew shellenv)" >> ~/.bashrc && \
echo "eval $($(brew --prefix)/bin/brew shellenv)" >> ~/.zshrc && \
export PATH="$(brew --prefix)/bin:$PATH" && \
export PATH="$(brew --prefix)/bin:$PATH" && \
echo 'eval "$(brew shellenv)"' >> ~/.bashrc && \
echo 'eval "$(brew shellenv)"' >> ~/.zshrc && \
source ~/.bashrc && source ~/.zshrc

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
