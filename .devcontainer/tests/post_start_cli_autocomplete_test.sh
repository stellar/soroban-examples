#!/bin/bash
# Unit tests for post_start_cli_autocomplete.sh

# Function to assert a condition
function assert() {
  local condition=$1
  local message=$2

  if eval "$condition"; then
    echo -e "${green}✓ PASS: ${message}${nc}"
    ((pass_count++))
    return 0
  else
    echo -e "${red}✗ FAIL: ${message}${nc}"
    echo -e "${red}     Condition failed: ${condition}${nc}"
    ((fail_count++))
    return 1
  fi
}

# Function to set up the linuxbrew directory for testing
function setup_linuxbrew_mock() {
  mkdir -p "${temp_dir}/home/linuxbrew/.linuxbrew/bin"
  touch "${temp_dir}/home/linuxbrew/.linuxbrew/bin/brew"
  chmod +x "${temp_dir}/home/linuxbrew/.linuxbrew/bin/brew"

  # Create a mock stellar completion command
  cat > "${temp_dir}/home/linuxbrew/.linuxbrew/bin/stellar" << 'EOF'
#!/bin/bash
if [[ "$1" == "completion" && "$2" == "--shell" ]]; then
  echo "# Mock completion for $3"
fi
EOF
  chmod +x "${temp_dir}/home/linuxbrew/.linuxbrew/bin/stellar"
}

# Function to run our script with mocked environment
function run_script_with_mock() {
  # Define variables used in the main script
  local original_path=$PATH

  # Source the script in a subshell to isolate environment changes
  ( 
    # Mock the existence of directories and commands
    PATH="${temp_dir}/home/linuxbrew/.linuxbrew/bin:$PATH"

    # Run the actual script we want to test
    # We'll create a temporary copy with modifications to ensure it works in our test environment
    sed "s|/home/linuxbrew|${temp_dir}/home/linuxbrew|g" .devcontainer/post_start_cli_autocomplete.sh > "${temp_dir}/test_script.sh"
    chmod +x "${temp_dir}/test_script.sh"

    # Run the script
    "${temp_dir}/test_script.sh"
  )

  # Return the script's exit code
  return $?
}

# Test: Check if chmod commands work correctly
function test_chmod_permissions() {
  echo -e "\n${yellow}Testing file permission changes${nc}"

  # Set initial permissions to something different
  chmod 600 "${HOME}/.bashrc"
  chmod 600 "${HOME}/.zshrc"
  chmod 600 "${HOME}/.profile"
  chmod 600 "${HOME}/.zprofile"

  # Run our script
  run_script_with_mock

  # Check if permissions are set correctly to 644
  assert "[ $(stat -c %a "${HOME}"/.bashrc) = '644' ]" "chmod 644 ~/.bashrc works"
  assert "[ $(stat -c %a "${HOME}"/.zshrc) = '644' ]" "chmod 644 ~/.zshrc works"
  assert "[ $(stat -c %a "${HOME}"/.profile) = '644' ]" "chmod 644 ~/.profile works"
  assert "[ $(stat -c %a "${HOME}"/.zprofile) = '644' ]" "chmod 644 ~/.zprofile works"
}

# Test: Check if linuxbrew path test and echo commands work
function test_linuxbrew_path_addition() {
  echo -e "\n${yellow}Testing linuxbrew path addition${nc}"

  # Clear the files first
  > "${HOME}/.bashrc"
  > "${HOME}/.zshrc"
  > "${HOME}/.profile"
  > "${HOME}/.zprofile"

  # Run our script
  run_script_with_mock

  # Check if the correct paths were added to each file
  assert "grep -q '/home/linuxbrew/.linuxbrew/bin/brew' ${HOME}/.zprofile" "Brew path added to .zprofile"
  assert "grep -q '/home/linuxbrew/.linuxbrew/bin/brew' ${HOME}/.zshrc" "Brew path added to .zshrc"
  assert "grep -q '/home/linuxbrew/.linuxbrew/bin/brew' ${HOME}/.profile" "Brew path added to .profile"
  assert "grep -q '/home/linuxbrew/.linuxbrew/bin/brew' ${HOME}/.bashrc" "Brew path added to .bashrc"
}

# Test: Check if PATH is correctly exported
function test_path_export() {
  echo -e "\n${yellow}Testing PATH export functionality${nc}"

  # Run script in a subshell to capture environment changes
  local path_test_output=$(
    # Clear PATH to ensure clean test
    PATH=""
    source "${temp_dir}/test_script.sh"
    echo "PATH=$PATH"
  )

  # Verify PATH contains our expected values
  assert "echo '$path_test_output' | grep -q '/home/linuxbrew/.linuxbrew/bin'" "PATH environment is correctly updated"
}

# Test: Check if stellar CLI auto-completion is added
function test_stellar_completion() {
  echo -e "\n${yellow}Testing Stellar CLI auto-completion${nc}"

  # Clear the files first
  > "${HOME}/.bashrc"
  > "${HOME}/.zshrc"

  # Run our script
  run_script_with_mock

  # Check if the correct stellar completion commands were added
  assert "grep -q 'source <(stellar completion --shell bash)' ${HOME}/.bashrc" "Stellar bash completion added"
  assert "grep -q 'source <(stellar completion --shell zsh)' ${HOME}/.zshrc" "Stellar zsh completion added"
}

# Test: Ensure script returns proper exit code
function test_exit_status() {
  echo -e "\n${yellow}Testing exit status handling${nc}"

  # Run our script and capture its exit status
  run_script_with_mock
  local script_exit_status=$?

  # Check if the script completed successfully
  assert "[ $script_exit_status -eq 0 ]" "Script exits with status 0 on success"

  # Test failure case by making a directory required by the script unavailable
  rm -rf "${temp_dir}/home/linuxbrew"

  # This should fail because linuxbrew directory is missing
  run_script_with_mock
  local script_fail_status=$?

  # Re-create for other tests
  setup_linuxbrew_mock

  # Check if script properly detects and reports failure
  # Note: Because of set -e, it should exit immediately when a command fails
  assert "[ $script_fail_status -ne 0 ]" "Script exits with non-zero status on failure"
}

# Main test runner
function run_tests() {
  echo -e "${yellow}=== Starting Unit Tests for post_start_cli_autocomplete.sh ===${nc}"

  # Set up the test environment
  setup_linuxbrew_mock

  # Run all test functions
  test_chmod_permissions
  test_linuxbrew_path_addition
  test_path_export
  test_stellar_completion
  test_exit_status
  test_idempotent_execution

  # Display test summary
  echo -e "\n${yellow}=== Test Summary ===${nc}"
  echo -e "${green}Passed: ${pass_count}${nc}"
  echo -e "${red}Failed: ${fail_count}${nc}"

  # Clean up
  echo -e "\n${yellow}Cleaning up test environment${nc}"
  rm -rf "${temp_dir}"

  # Return non-zero exit code if any tests failed
  [ ${fail_count} -eq 0 ]
}

#######################################
# description
# Globals:
#   fail_count
#   green
#   HOME
#   nc
#   pass_count
#   red
#   temp_dir
#   yellow
# Arguments:
#  None
#######################################
function main() {
  # Set up text coloring for output
  red='\033[0;31m'

  green='\033[0;32m'

  yellow='\033[1;33m'

  nc='\033[0m' # No Color

  echo -e "${yellow}Running test: ${test_script}${nc}"

  # Counter for tracking test results
  pass_count=0

  fail_count=0

  # Create a temporary directory for testing
  temp_dir=$(mktemp -d)

  echo -e "${yellow}Creating test environment in ${temp_dir}${nc}"

  # Set up mock home directory
  export HOME="${temp_dir}"

  mkdir -p "${HOME}"

  # Initialize empty config files
  touch "${HOME}/.bashrc"

  touch "${HOME}/.zshrc"

  touch "${HOME}/.profile"

  touch "${HOME}/.zprofile"

  # Execute tests
  run_tests

  exit $?

}

main "$@"
