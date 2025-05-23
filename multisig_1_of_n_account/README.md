# Multisig 1-of-n Contract Account

This example contains a custom contract account that authorizes when one ed25519
signature is provided, where the signature was produced by a ed25519 key
configured in the contract. The contract may hold any number of keys, and any
key may authorize for it.

The example also contains a stellar-cli plugin that signs authorizations using
an ed25519.

## Usage

The example below sets up an asset with the contract account as the admin. The
admin authorizes with ed25519 keys.

The ed25519 keys used in the example below are:

- Secret Key:
  `0000000000000000000000000000000000000000000000000000000000000000`\
  Public Key: `3b6a27bcceb6a42d62a3a8d02a6f0d73653215771de243a63ac048a18b59da29`
- Secret Key:
  `0000000000000000000000000000000000000000000000000000000000000001`\
  Public Key: `4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29`

These keys are publicly viewable and not random. Do not use these keys for any
purpose. Select your own keys and update them in the commands below when
executing.

To generate your own keys use any secure random generator to generate a 32-byte
seed, then any ed25519 library to derive the public key from the seed. For
example:

```
$ deno repl
Deno 2.3.1
exit using ctrl+d, ctrl+c, or close()
> import { Keypair } from "npm:@stellar/stellar-sdk"
> import { encodeHex } from "jsr:@std/encoding"
> const kp = Keypair.random()
> encodeHex(kp.rawPublicKey())
"5745..."
> encodeHex(kp.rawSecretKey())
"acc8..."
```

### Install the `stellar sign-auth-ed25519` plugin

Install one of the stellar sign-auth-ed25519 plugin implementations.

#### Rust

Install the `stellar sign-auth-ed25519` plugin:

```
cd stellar-cli-sign-auth-ed25519
cargo install --locked --path .
```

#### JavaScript ([Deno])

Install [Deno] with:

macOS/Linux:

```
curl -fsSL https://deno.land/install.sh | sh
```

Windows:

```
irm https://deno.land/install.ps1 | iex
```

Install the `stellar sign-auth-ed25519-js` plugin:

```
cd stellar-cli-sign-auth-ed25519-js
deno install \
    --global \
    --name stellar-sign-auth-ed25519-js \
    --config deno.json \
    --allow-read --no-prompt \
    --force \
    main.ts
```

Note: By default Deno scripts when installed have no permissions and cannot read
or write files, read environment variables, access the network and cannot
execute commands. The `--allow-read` flag is specified to give the script
permission to read files so that it can read .wasm dependencies in the
`@stellar/stellar-xdr-json` package which is a Rust-built-to-wasm npm package.

[Deno]: https://deno.com

### Change to the Contract directory

```
cd contract
```

### Configure Network

```
stellar network use testnet
```

### Create a Testnet Account to Deploy with and pay Tx fees

```
stellar keys generate --fund feepayer
stellar keys use feepayer
```

### Build the contract

```
stellar contract build --out-dir out/
```

### Deploy the contract account

Deploy the contract, including a list of signers that will be permitted to
authorize / sign for the contract account. The signers should be the hex encoded
32-byte ed25519 public keys discussed at the top of this document.

```
stellar contract deploy \
    --alias admin \
    --wasm out/soroban_multisig_1_of_n_account_contract.wasm \
    -- \
    --signers '[
      "3b6a27bcceb6a42d62a3a8d02a6f0d73653215771de243a63ac048a18b59da29",
      "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29"
    ]'
```

### Deploy asset

Create an account that'll be the issuer of the asset. Deploy the asset contract.

```
stellar keys generate --fund issuer
stellar contract asset deploy \
    --alias asset \
    --asset ABC:issuer
```

### List of contracts

```
stellar contract alias ls
```

### Set admin

Set the admin of the asset contract to the contract account deployed in a
previous step. The default admin for the asset contract is the issuer, and
setting the admin will change which address can authorize operations like
minting.

```
stellar contract invoke --source issuer --id asset -- \
    set_admin \
    --new_admin admin
```

### Mint

Call mint, sending to an address. The invocation will be signed by the admin
using one of the ed25519 keys set in the constructor. Choose which key signs by
setting the `--secret-key` option to one of the two keys above.

In the command below replace `sign-auth-ed25519` with `sign-auth-ed25519-js` if
using the JS stellar-cli plugin.

```
stellar contract invoke --id asset --build-only -- \
    mint \
    --to CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4 \
    --amount 123 \
  | stellar tx simulate \
  | stellar sign-auth-ed25519 \
      --secret-key 0000000000000000000000000000000000000000000000000000000000000001 \
      --signature-expiration-ledger 2296800 \
  | stellar tx simulate \
  | stellar tx sign --sign-with-key feepayer \
  | stellar tx send
```

Note: The 'feepayer' key is signing the transaction to pay the fee, and the
admin signature produced by the `sign-auth-ed25519` plugin is what is
authorizing the mint. The issuer key is not involved in the mint at all.

### View Balance

Verify that the mint did take place by viewing the balance of the recipient.

```
stellar contract invoke --id asset -- \
    balance \
    --id CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4
```
