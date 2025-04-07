#!/bin/bash
set -e

clear

echo "========================================="
echo " ✅ Getting Started with the Stellar CLI"
echo "========================================="
echo ""
echo "  ⚙️ Setup testnet:"
echo "     stellar network use testnet"
echo ""
echo "  🆔 Configure an Identity:"
echo "     stellar keys generate --global alice --network testnet --fund"
echo "     stellar keys address alice"
echo ""
echo "  🛠️ Build a contract(For specific example.  Replace 'alloc' with desired project):"
echo "     stellar contract build --manifest-path $CODESPACE_VSCODE_FOLDER/alloc/Cargo.toml"
echo ""
echo "  🧪 Run tests | 🔨Build Projects"
echo "  ⚠️ Running make all in the root directory will fill up a 32GB storage instance"
echo "     cd account"
echo "     cargo test     ⭐️make all       "
echo ""
echo "  📖 Stellar CLI Manual(cmd+click) 🔗 https://developers.stellar.org/docs/tools/cli/stellar-cli"
echo "  👩‍🔬 Stellar Lab(cmd+click) 🔗 https://lab.stellar.org/"
echo "========================================="

