# Bash Script Testing Framework

This directory contains a testing framework for shell scripts used in the devcontainer setup. The tests help ensure that
shell scripts perform as expected and catch regressions before they affect users.

## Running Tests

To run all tests:

```bash
# Make the run script executable
chmod +x .devcontainer/run_tests.sh

# Run all tests
.devcontainer/run_tests.sh
```

To run a specific test:

```bash
# Make the test script executable
chmod +x .devcontainer/post_start_cli_autocomplete_test.sh

# Run the specific test
.devcontainer/post_start_cli_autocomplete_test.sh
```

## Test Structure

Each test script follows this general structure:

1. **Setup Phase**: Create a temporary environment for testing
2. **Test Functions**: Individual test cases for different script features
3. **Assertion Mechanism**: Check if conditions are met and track pass/fail
4. **Cleanup Phase**: Remove temporary files and restore the environment

## Adding New Tests

To create tests for a new script:

1. Create a new test file named `scriptname_test.sh`
2. Copy the basic structure from an existing test script
3. Modify the test functions to validate your script's functionality
4. Add appropriate assertions to check expected outcomes

## Testing Philosophy

Our tests are designed to:

- Run in an isolated environment without modifying the real system
- Test each feature and command independently
- Provide clear feedback when tests fail
- Be idempotent (can be run multiple times safely)
- Clean up after themselves

### Testing File Operations

```bash
# Before operation
touch "${HOME}/.testfile"
chmod 600 "${HOME}/.testfile"

# Run script
run_script_with_mock

# Assert expected changes
assert "[ $(stat -c %a ${HOME}/.testfile) = '644' ]" "File permission change worked"
```

### Testing Environment Variables

```bash
env_test_output=$(
  # Clear variable to ensure clean test
  my_var=""
  source "${TEMP_DIR}/test_script.sh"
  echo "MY_VAR=$my_var"
)

assert "echo '$env_test_output' | grep -q 'expected-value'" "Environment variable is correctly set"
```

### Testing Command Execution

```bash
# Mock the command
mkdir -p "${TEMP_DIR}/bin"
cat > "${TEMP_DIR}/bin/my-command" << 'EOF'
#!/bin/bash
echo "Mock executed with: $@"
EOF
chmod +x "${TEMP_DIR}/bin/my-command"

# Add to PATH for the test
PATH="${TEMP_DIR}/bin:$PATH"

# Run script and capture output
output=$(run_script_with_mock)

# Assert command was executed with expected arguments
```
