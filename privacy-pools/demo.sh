#!/bin/bash
set -e
echo "🚀 Starting Privacy Pool Demo..."

NETWORK=testnet # testnet, local
TOKEN_ADDRESS=CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC # XLM token address on testnet

# Clean up old test files
echo "🧹 Cleaning up old test files..."
rm -f demo_coin.json demo_state.json demo_association.json vk_hex.txt proof_hex.txt public_hex.txt withdrawal_input.json circuits/witness.wtns circuits/proof.json circuits/public.json 2>/dev/null || true

# Check prerequisites
echo "🔍 Checking prerequisites..."
command -v jq >/dev/null 2>&1 || { echo "❌ Error: jq is required but not installed. Please install jq first."; exit 1; }
command -v stellar >/dev/null 2>&1 || { echo "❌ Error: stellar CLI is required but not installed."; exit 1; }
# Fund demo_user account if needed
echo "🏦 Ensuring demo_user account is funded..."
(stellar keys generate demo_user && stellar keys fund demo_user --network $NETWORK) > /dev/null 2>&1 || echo "⚠️  demo_user may already be funded"
# Step 1: Deploy contract
echo "📦 Deploying contract..."
cargo build --target wasm32v1-none --release -p privacy-pools || { echo "❌ Error: Failed to build contract"; exit 1; }
stellar contract optimize --wasm target/wasm32v1-none/release/privacy_pools.wasm --wasm-out target/wasm32v1-none/release/privacy_pools.optimized.wasm || { echo "❌ Error: Failed to optimize WASM"; exit 1; }
# Convert verification key to hex format and extract it
echo "🔑 Converting verification key..."
cargo run --bin circom2soroban vk circuits/output/main_verification_key.json > vk_hex.txt || { echo "❌ Error: Failed to convert verification key"; exit 1; }
VK_HEX=$(cat vk_hex.txt | grep -o '[0-9a-f]*$')
if [ -z "$VK_HEX" ]; then
    echo "❌ Error: Failed to extract verification key hex"
    exit 1
fi

echo "🚀 Deploying contract to $NETWORK..."
CONTRACT_ID=$(stellar contract deploy --wasm target/wasm32v1-none/release/privacy_pools.optimized.wasm --source demo_user --network $NETWORK -- --vk_bytes $VK_HEX --token_address $TOKEN_ADDRESS --admin demo_user 2>&1 | grep -o 'C[A-Z0-9]\{55\}' | tail -1)
if [ -z "$CONTRACT_ID" ]; then
    echo "❌ Error: Failed to extract contract ID from deployment"
    exit 1
fi
echo "✅ Contract deployed with ID: $CONTRACT_ID"

# Check who the admin is
echo "👤 Checking contract admin..."
stellar contract invoke --id $CONTRACT_ID --source demo_user --network $NETWORK -- get_admin || { echo "❌ Error: Failed to get admin"; exit 1; }

# Step 2: Generate coin
echo "🪙 Generating coin..."
cargo run --bin coinutils generate demo_pool -o demo_coin.json || { echo "❌ Error: Failed to generate coin"; exit 1; }
COMMITMENT_HEX=$(cat demo_coin.json | jq -r '.commitment_hex' | sed 's/^0x//')
if [ -z "$COMMITMENT_HEX" ]; then
    echo "❌ Error: Failed to extract commitment hex"
    exit 1
fi
echo "Generated coin with commitment: $COMMITMENT_HEX"
# Step 3: Deposit
echo "💰 Depositing coin..."
stellar contract invoke --id $CONTRACT_ID --source demo_user --network $NETWORK -- deposit --from demo_user --commitment $COMMITMENT_HEX || { echo "❌ Error: Failed to deposit coin"; exit 1; }
echo "Deposit successful!"
# Step 4: Check balance
echo "📊 Checking balance..."
stellar contract invoke --id $CONTRACT_ID --source demo_user --network $NETWORK -- get_balance || { echo "❌ Error: Failed to get balance"; exit 1; }
# Step 5: Create state file and association set
echo "📋 Creating state file..."
COMMITMENT=$(cat demo_coin.json | jq -r '.coin.commitment')
echo "{
  \"commitments\": [
    \"$COMMITMENT\"
  ],
  \"scope\": \"demo_pool\"
}" > demo_state.json

echo "🏷️  Creating association set..."
LABEL=$(cat demo_coin.json | jq -r '.coin.label')
cargo run --bin coinutils update-association demo_association.json "$LABEL" || { echo "❌ Error: Failed to create association set"; exit 1; }

# Extract association root from the association set and set it in the contract
echo "🔗 Setting association root in contract..."
ASSOCIATION_ROOT_DECIMAL=$(cat demo_association.json | jq -r '.root')
if [ -z "$ASSOCIATION_ROOT_DECIMAL" ] || [ "$ASSOCIATION_ROOT_DECIMAL" = "null" ]; then
    echo "❌ Error: Failed to extract association root from association set"
    exit 1
fi

# Convert decimal string to hex using Python (since it handles big integers well)
ASSOCIATION_ROOT_HEX=$(python3 -c "
import sys
decimal_str = sys.argv[1]
# Convert decimal to hex
hex_str = hex(int(decimal_str))[2:]  # Remove '0x' prefix
# Pad to 64 hex characters (32 bytes)
padded_hex = hex_str.zfill(64)
print(padded_hex)
" "$ASSOCIATION_ROOT_DECIMAL")

if [ -z "$ASSOCIATION_ROOT_HEX" ]; then
    echo "❌ Error: Failed to convert association root to hex format"
    exit 1
fi

echo "🔍 Debug: Association root decimal: $ASSOCIATION_ROOT_DECIMAL"
echo "🔍 Debug: Association root hex: $ASSOCIATION_ROOT_HEX"

# Note: Only the admin (contract deployer) can set association root
stellar contract invoke --id $CONTRACT_ID --source demo_user --network $NETWORK -- set_association_root --caller demo_user --association_root $ASSOCIATION_ROOT_HEX || { echo "❌ Error: Failed to set association root"; exit 1; }
echo "✅ Association root set successfully"

echo "🔐 Creating withdrawal proof..."
cargo run --bin coinutils withdraw demo_coin.json demo_state.json demo_association.json -o withdrawal_input.json || { echo "❌ Error: Failed to create withdrawal input"; exit 1; }
echo "📝 Generating witness and proof..."
cd circuits
node build/main_js/generate_witness.js build/main_js/main.wasm ../withdrawal_input.json witness.wtns || { echo "❌ Error: Failed to generate witness"; exit 1; }
snarkjs groth16 prove output/main_final.zkey witness.wtns proof.json public.json || { echo "❌ Error: Failed to generate proof"; exit 1; }
cd ..
echo "🔄 Converting proof for Soroban..."
cargo run --bin circom2soroban proof circuits/proof.json > proof_hex.txt || { echo "❌ Error: Failed to convert proof"; exit 1; }
cargo run --bin circom2soroban public circuits/public.json > public_hex.txt || { echo "❌ Error: Failed to convert public signals"; exit 1; }
PROOF_HEX=$(sed -n '/^Proof Hex encoding:/{n;p;}' proof_hex.txt | tr -d '[:space:]' | sed -E 's/^0x//i')
PUBLIC_HEX=$(sed -n '/^Public signals Hex encoding:/{n;p;}' public_hex.txt | tr -d '[:space:]' | sed -E 's/^0x//i')
if [ -z "$PROOF_HEX" ] || [ -z "$PUBLIC_HEX" ]; then
    echo "❌ Error: Failed to extract proof or public signals"
    exit 1
fi
echo "🔍 Debug: Proof hex length: ${#PROOF_HEX}"
echo "🔍 Debug: Public hex length: ${#PUBLIC_HEX}"

# Step 6: Withdraw
echo "💸 Withdrawing coin..."
stellar contract invoke --id $CONTRACT_ID --source demo_user --network $NETWORK -- withdraw --to demo_user --proof_bytes "$PROOF_HEX" --pub_signals_bytes "$PUBLIC_HEX" || { echo "❌ Error: Failed to withdraw coin"; exit 1; }
echo "Withdrawal successful!"
# Step 7: Verify
echo "✅ Verifying withdrawal..."
stellar contract invoke --id $CONTRACT_ID --source demo_user --network $NETWORK -- get_nullifiers || { echo "❌ Error: Failed to get nullifiers"; exit 1; }
stellar contract invoke --id $CONTRACT_ID --source demo_user --network $NETWORK -- get_balance || { echo "❌ Error: Failed to get final balance"; exit 1; }
echo "🎉 Demo completed successfully!"

