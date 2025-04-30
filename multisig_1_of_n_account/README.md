
## How to use this contract

### Deploy admin

```
cd contract
stellar contract build --out-dir out/
stellar contract deploy \
  --alias admin \
  --wasm out/soroban_multisig_1_of_n_account_contract.wasm
  -- \
  --signers '[
    "",
    ""
  ]'
cd ..
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

```
stellar contract invoke --id asset -- \
  mint \
  --to CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4 \
  --amount 123
```



