#!/bin/bash
set -e


# Add Brew and Stellar CLI to $PATH and terminals
chmod 644 ~/.bashrc && chmod 644 ~/.zshrc 
chmod 644 ~/.profile && chmod 644 ~/.zprofile
test -d /home/linuxbrew/.linuxbrew  && echo "/home/linuxbrew/.linuxbrew/bin/brew" >> ~/.zprofile
test -d /home/linuxbrew/.linuxbrew && echo "/home/linuxbrew/.linuxbrew/bin/brew" >>~/.zshrc

test -d /home/linuxbrew/.linuxbrew  && echo "/home/linuxbrew/.linuxbrew/bin/brew" >> ~/.profile
test -d /home/linuxbrew/.linuxbrew && echo "/home/linuxbrew/.linuxbrew/bin/brew" >>~/.bashrc

export PATH="/home/linuxbrew/.linuxbrew/bin/brew":$PATH
echo "export PATH='/home/linuxbrew/.linuxbrew/bin:$PATH'" >> ~/.zshrc
export PATH="/home/linuxbrew/.linuxbrew/bin:$PATH"
echo "export PATH='/home/linuxbrew/.linuxbrew/bin:$PATH'" >> ~/.bashrc


# Stellar CLI Auto-Complete
chmod 644 ~/.bashrc && \
  echo "source <(stellar completion --shell bash)" >>~/.bashrc && \
  chmod 644 ~/.zshrc && \
  echo "source <(stellar completion --shell zsh)" >>~/.zshrc && \
  echo "Enabled Stellar CLI auto-completion"

# Store exit status of the previous command
RESULT=$?

# Check the exit status and provide informative output
if [ $RESULT -eq 0 ]; then
  echo " ✅ postStartCliAutocomplete.sh executed successfully"
else
  echo " ❌ Error executing postStartCliAutocomplete.sh"
fi