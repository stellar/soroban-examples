#!/bin/bash
set -e

clear

echo "========================================="
echo " ✅ Getting Started with the Stellar CLI"
echo "========================================="
echo ""
echo "  ⚙️ Setup testnet:"
echo "  stellar network use testnet"
echo ""
echo "  🆔 Configure an Identity:"
echo "  stellar keys generate --global alice --network testnet --fund"
echo "  stellar keys address alice"
echo ""
echo "  🛠️ Build a contract:"
echo "  stellar contract build"
echo ""
echo "  🧪 Run tests | 🔨Build all Projects"
echo "     cargo test     ⭐️make all       "
echo ""
echo "  📖 Stellar CLI Manual "
echo "  🔗 https://developers.stellar.org/docs/tools/cli/stellar-cli"
echo ""
echo "  👩‍🔬 Stellar Lab "
echo "  🔗 https://lab.stellar.org/"
echo "========================================="

