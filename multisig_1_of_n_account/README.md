
## How to use this contract

The example below sets up an asset with an admin, where that admin can sign for any authorization with ed25519 keys.

The ed25519 keys used in the example below are:

- Secret Key: `0000000000000000000000000000000000000000000000000000000000000000`  
  Public Key: `3b6a27bcceb6a42d62a3a8d02a6f0d73653215771de243a63ac048a18b59da29`
- Secret Key: `0000000000000000000000000000000000000000000000000000000000000001`  
  Public Key: `4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29`

These keys are publicly viewable and not random. Do not use these keys for any purpose. Select your own keys.

### Install the sign-ed25519 cli plugin

```
cd sign-ed25519
cargo install --locked --path .
```

### Deploy admin

```
cd contract
stellar contract build --out-dir out/
stellar contract deploy \
    --alias admin \
    --wasm out/soroban_multisig_1_of_n_account_contract.wasm
    -- \
    --signers '[
      "3b6a27bcceb6a42d62a3a8d02a6f0d73653215771de243a63ac048a18b59da29",
      "4cb5abf6ad79fbf5abbccafcc269d85cd2651ed4b885b5869f241aedf0a5ba29"
    ]'
```

### Deploy asset

```
stellar keys generate issuer
stellar contract asset deploy \
    --alias asset \
    --asset ABC:issuer
```

### List of contracts

```
stellar contract alias ls
```

### Set admin

```
stellar contract invoke --id asset -- \
    set_admin \
    --new_admin admin
```

### Mint

Call mint, sending to an address. The invocation will be signed by the admin using one of the ed25519 keys set in the constructor. Choose which key signs by setting the `SECRET_KEY` environment variable.

```
stellar contract invoke --id asset -- \
    mint \
    --to CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4 \
    --amount 123 \
  | stellar tx simulate \
  | tr -d '\n' \
  | SECRET_KEY=0000000000000000000000000000000000000000000000000000000000000001 \
      stellar sign-ed25519 \
  | stellar tx simulate \
  | stellar tx send
```

### View Balance

```
stellar contract invoke --id asset -- \
    balance \
    --id CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4
```
