#!/bin/bash
set -e

clear

echo "========================================="
echo " ✅ Getting Started with the Stellar CLI"
echo "========================================="
echo ""
echo "  ⚙️ Environment variables:"
echo "  STELLAR_RPC_URL: ${STELLAR_RPC_URL}"
echo "  STELLAR_NETWORK_PASSPHRASE: ${STELLAR_NETWORK_PASSPHRASE}"
echo ""
echo "  🆔 Configure an Identity:"
echo "  stellar keys generate --global alice --network testnet --fund"
echo "  stellar keys address alice"
echo ""
echo "  🧪 Run tests"
echo "  cargo test"
echo ""
echo "  📖 Stellar CLI Manual "
echo "  🔗 https://developers.stellar.org/docs/tools/cli/stellar-cli"
echo ""
echo "  👩‍🔬 Stellar Lab "
echo "  🔗 https://lab.stellar.org/"
echo "========================================="

