#!/bin/bash
set -e
# Add Brew and Stellar CLI to $PATH and terminals
homebin=/home/linuxbrew/.linuxbrew/bin && \
homebrew=/home/linuxbrew/.linuxbrew/bin/brew && \
chmod +w ~/.bashrc && chmod +w ~/.zshrc && \
test -d ~/.linuxbrew && eval "$(~/.linuxbrew/bin/brew shellenv)" && \
test -d /home/linuxbrew/.linuxbrew && eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)" && \
echo "eval $($($homebrew --prefix)/bin/brew shellenv)" >> ~/.bashrc && \
echo "eval $($homebrew shellenv)" >> ~/.bashrc && \
echo "eval $($($homebrew --prefix)/bin/brew shellenv)" >> ~/.zshrc && \
echo "eval $($homebrew shellenv)" >> ~/.zshrc && \
export PATH="$($homebrew --prefix)/bin:$PATH" && \
export PATH="$homebin:$PATH" >> ~/.zshrc && \
export PATH="$($homebrew --prefix)/bin:$PATH" && \
export PATH="$homebin:$PATH" >> ~/.bashrc && \
echo 'eval "$(brew shellenv)"' >> ~/.bashrc && \
echo 'eval "$(brew shellenv)"' >> ~/.zshrc && \
source ~/.bashrc && source ~/.zshrc

# Stellar CLI Auto-Complete
echo "source <(stellar completion --shell bash)" >>~/.bashrc && \
echo "source <(stellar completion --shell zsh)" >>~/.zshrc && \
echo "Enabled Stellar CLI auto-completion"

# Check the exit status and provide informative output
if [ $? -eq 0 ]; then
  echo " ✅ postStartCliAutocomplete.sh executed successfully"
else
  echo " ❌ Error executing postStartCliAutocomplete.sh"
fi
