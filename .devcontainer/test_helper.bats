_tests_helper() {
    echo "BATS LIB PATH: ${BATS_LIB_PATH}"
    bats_load_library bats-support
    bats_load_library bats-assert
    bats_load_library bats-file
}

_create_dir_file() {
    temp_make
    assert_dir_exist "${BATS_TMPDIR}"
    echo "Temp: ${BATS_TMPDIR}"

    # Create empty bash and zsh config files in the temporary directory
    touch "${BATS_TMPDIR}/.bashrc"
    touch "${BATS_TMPDIR}/.zshrc"

    assert_file_exists "${BATS_TMPDIR}/.bashrc"
    assert_file_exists "${BATS_TMPDIR}/.zshrc"
}

_delete_dir_file() {
    # Remove the files created during the test
    rm -f "${BATS_TMPDIR}/.bashrc" "${BATS_TMPDIR}/.zshrc"
}
