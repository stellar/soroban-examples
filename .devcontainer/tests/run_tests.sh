#!/bin/bash
# Run script to execute all unit tests for bash scripts

# Set up text coloring for output
red='\033[0;31m'
green='\033[0;32m'
yellow='\033[1;33m'
nc='\033[0m' # No Color

# Find and execute all test scripts
find "$ContentRoot"$"/.devcontainer" -name "*_test.sh" -type f | while read -r test_script; do
  echo -e "${yellow}Running test: ${test_script}${nc}"
  chmod +x "$test_script"
  
  if "$test_script"; then
    echo -e "${green}✓ Test passed: ${test_script}${nc}"
  else
    echo -e "${red}✗ Test failed: ${test_script}${nc}"
    exit 1
  fi
  echo -e "${yellow}------------------------${nc}"
done

# Output overall result
if [ "${failed}" = "1" ]; then
  echo -e "${red}Some tests failed!${nc}"
  exit 1
else
  echo -e "${green}All tests passed successfully!${nc}"
  exit 0
fi
