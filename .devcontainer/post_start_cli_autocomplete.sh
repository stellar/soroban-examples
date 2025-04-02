#!/bin/bash
set -e
# Add Brew and Stellar CLI to $PATH and terminals
# Add Brew and Stellar CLI to $PATH and terminals
chmod 644 ~/.bashrc && chmod 644 ~/.zshrc 
chmod 644 ~/.profile && chmod 644 ~/.zprofile
test -d ~/.linuxbrew && echo ~/.linuxbrew/bin/brew >> ~/.zprofile
test -d /home/linuxbrew/.linuxbrew && echo "$(~/.linuxbrew/bin/brew)" >>.zshrc

test -d ~/.linuxbrew && echo ~/.linuxbrew/bin/brew >> ~/.profile
test -d /home/linuxbrew/.linuxbrew && echo "$(~/.linuxbrew/bin/brew)" >>.bashrc

eval "eval $(~/.linuxbrew/bin/brew)" >> ~/.zshrc
echo "eval $(~/.linuxbrew/bin/brew)" >> ~/.bashrc

export PATH="/home/linuxbrew/.linuxbrew/bin/brew/bin:$PATH"
export PATH="/home/linuxbrew/.linuxbrew/bin:$PATH" >> ~/.zshrc
export PATH="/home/linuxbrew/.linuxbrew/bin/brew/bin:$PATH"
export PATH="/home/linuxbrew/.linuxbrew/bin:$PATH" >> ~/.bashrc

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
