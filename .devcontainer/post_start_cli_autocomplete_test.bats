#!/usr/bin/env bats

setup() {
    load 'test_helper.bats'
    _tests_helper
    _create_dir_file
}

# executed after each test
teardown() {
    _delete_dir_file
}

@test "script executes successfully and configures PATH and autocompletion" {
    # Copy the script to the temporary directory
    cp "$(dirname "$BATS_TEST_FILENAME")/post_start_cli_autocomplete.sh" "${BATS_TMPDIR}/"
    chmod +x "${BATS_TMPDIR}/post_start_cli_autocomplete.sh"

    # Set HOME to BATS_TMPDIR for the test
    export HOME="${BATS_TMPDIR}"

    # Run the script and capture its output
    run "${BATS_TMPDIR}/post_start_cli_autocomplete.sh"

    # Verify /usr/local/bin/ is appended to .bashrc and .zshrc files
    assert_file_contains "${BATS_TMPDIR}/.bashrc" "/usr/local/bin/"
    assert_file_contains "${BATS_TMPDIR}/.zshrc" "/usr/local/bin/"

    # Verify env PATH export in .bashrc and .zshrc
    assert_file_contains "${BATS_TMPDIR}/.bashrc" $'export PATH=\'/usr/local/bin/'
    assert_file_contains "${BATS_TMPDIR}/.zshrc" $'export PATH=\'/usr/local/bin/'

    # Verify stellar CLI completion is included in .bashrc and .zshrc
    assert_file_contains "${BATS_TMPDIR}/.bashrc" "source <(stellar completion --shell bash)"
    assert_file_contains "${BATS_TMPDIR}/.zshrc" "source <(stellar completion --shell zsh)"

    # Check script execution status is successful
    assert_success

    # Check for success message on script execution
    assert_output --partial "✅ postStartCliAutocomplete.sh executed successfully"
}
