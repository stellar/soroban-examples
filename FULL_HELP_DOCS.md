# Stellar CLI Manual

This document contains the help content for the `stellar` command-line program.

## `stellar`

Work seamlessly with Stellar accounts, contracts, and assets from the command line.

- Generate and manage keys and accounts
- Build, deploy, and interact with contracts
- Deploy asset contracts
- Stream events
- Start local testnets
- Decode, encode XDR
- More!

For additional information see:

- Stellar Docs: https://developers.stellar.org
- Smart Contract Docs: https://developers.stellar.org/docs/build/smart-contracts/overview
- CLI Docs: https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli

To get started generate a new identity:

    stellar keys generate alice

Use keys with the `--source` flag in other commands.

Commands that work with contracts are organized under the `contract` subcommand. List them:

    stellar contract --help

Use contracts like a CLI:

    stellar contract invoke --id CCR6QKTWZQYW6YUJ7UP7XXZRLWQPFRV6SWBLQS4ZQOSAF4BOUD77OTE2 --source alice --network testnet -- --help

Anything after the `--` double dash (the "slop") is parsed as arguments to the contract-specific CLI, generated on-the-fly from the contract schema. For the hello world example, with a function called `hello` that takes one string argument `to`, here's how you invoke it:

    stellar contract invoke --id CCR6QKTWZQYW6YUJ7UP7XXZRLWQPFRV6SWBLQS4ZQOSAF4BOUD77OTE2 --source alice --network testnet -- hello --to world


**Usage:** `stellar [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `contract` — Tools for smart contract developers
* `events` — Watch the network for contract events
* `env` — Prints the environment variables
* `keys` — Create and manage identities including keys and addresses
* `network` — Configure connection to networks
* `container` — Start local networks in containers
* `snapshot` — Download a snapshot of a ledger from an archive
* `tx` — Sign, Simulate, and Send transactions
* `xdr` — Decode and encode XDR
* `completion` — Print shell completion code for the specified shell
* `cache` — Cache for transactions and contract specs
* `version` — Print version information

###### **Options:**

* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `-f`, `--filter-logs <FILTER_LOGS>` — Filter logs output. To turn on `stellar_cli::log::footprint=debug` or off `=off`. Can also use env var `RUST_LOG`
* `-q`, `--quiet` — Do not write logs to stderr including `INFO`
* `-v`, `--verbose` — Log DEBUG events
* `--very-verbose` — Log DEBUG and TRACE events
* `--list` — List installed plugins. E.g. `stellar-hello`
* `--no-cache` — Do not cache your simulations and transactions



## `stellar contract`

Tools for smart contract developers

**Usage:** `stellar contract <COMMAND>`

###### **Subcommands:**

* `asset` — Utilities to deploy a Stellar Asset Contract or get its id
* `alias` — Utilities to manage contract aliases
* `bindings` — Generate code client bindings for a contract
* `build` — Build a contract from source
* `extend` — Extend the time to live ledger of a contract-data ledger entry
* `deploy` — Deploy a wasm contract
* `fetch` — Fetch a contract's Wasm binary
* `id` — Generate the contract id for a given contract or asset
* `info` — Access info about contracts
* `init` — Initialize a Soroban contract project
* `inspect` — (Deprecated in favor of `contract info` subcommand) Inspect a WASM file listing contract functions, meta, etc
* `upload` — Install a WASM file to the ledger without creating a contract instance
* `install` — (Deprecated in favor of `contract upload` subcommand) Install a WASM file to the ledger without creating a contract instance
* `invoke` — Invoke a contract function
* `optimize` — Optimize a WASM file
* `read` — Print the current value of a contract-data ledger entry
* `restore` — Restore an evicted value for a contract-data legder entry



## `stellar contract asset`

Utilities to deploy a Stellar Asset Contract or get its id

**Usage:** `stellar contract asset <COMMAND>`

###### **Subcommands:**

* `id` — Get Id of builtin Soroban Asset Contract. Deprecated, use `stellar contract id asset` instead
* `deploy` — Deploy builtin Soroban Asset Contract



## `stellar contract asset id`

Get Id of builtin Soroban Asset Contract. Deprecated, use `stellar contract id asset` instead

**Usage:** `stellar contract asset id [OPTIONS] --asset <ASSET>`

###### **Options:**

* `--asset <ASSET>` — ID of the Stellar classic asset to wrap, e.g. "USDC:G...5"
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar contract asset deploy`

Deploy builtin Soroban Asset Contract

**Usage:** `stellar contract asset deploy [OPTIONS] --asset <ASSET> --source-account <SOURCE_ACCOUNT>`

###### **Options:**

* `--asset <ASSET>` — ID of the Stellar classic asset to wrap, e.g. "USDC:G...5"
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--alias <ALIAS>` — The alias that will be used to save the assets's id. Whenever used, `--alias` will always overwrite the existing contract id configuration without asking for confirmation



## `stellar contract alias`

Utilities to manage contract aliases

**Usage:** `stellar contract alias <COMMAND>`

###### **Subcommands:**

* `remove` — Remove contract alias
* `add` — Add contract alias
* `show` — Show the contract id associated with a given alias
* `ls` — List all aliases



## `stellar contract alias remove`

Remove contract alias

**Usage:** `stellar contract alias remove [OPTIONS] <ALIAS>`

###### **Arguments:**

* `<ALIAS>` — The contract alias that will be removed

###### **Options:**

* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config



## `stellar contract alias add`

Add contract alias

**Usage:** `stellar contract alias add [OPTIONS] --id <CONTRACT_ID> <ALIAS>`

###### **Arguments:**

* `<ALIAS>` — The contract alias that will be used

###### **Options:**

* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `--overwrite` — Overwrite the contract alias if it already exists
* `--id <CONTRACT_ID>` — The contract id that will be associated with the alias



## `stellar contract alias show`

Show the contract id associated with a given alias

**Usage:** `stellar contract alias show [OPTIONS] <ALIAS>`

###### **Arguments:**

* `<ALIAS>` — The contract alias that will be displayed

###### **Options:**

* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config



## `stellar contract alias ls`

List all aliases

**Usage:** `stellar contract alias ls [OPTIONS]`

###### **Options:**

* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar contract bindings`

Generate code client bindings for a contract

**Usage:** `stellar contract bindings <COMMAND>`

###### **Subcommands:**

* `json` — Generate Json Bindings
* `rust` — Generate Rust bindings
* `typescript` — Generate a TypeScript / JavaScript package
* `python` — Generate Python bindings
* `java` — Generate Java bindings



## `stellar contract bindings json`

Generate Json Bindings

**Usage:** `stellar contract bindings json --wasm <WASM>`

###### **Options:**

* `--wasm <WASM>` — Path to wasm binary



## `stellar contract bindings rust`

Generate Rust bindings

**Usage:** `stellar contract bindings rust --wasm <WASM>`

###### **Options:**

* `--wasm <WASM>` — Path to wasm binary



## `stellar contract bindings typescript`

Generate a TypeScript / JavaScript package

**Usage:** `stellar contract bindings typescript [OPTIONS] --output-dir <OUTPUT_DIR> <--wasm <WASM>|--wasm-hash <WASM_HASH>|--contract-id <CONTRACT_ID>>`

###### **Options:**

* `--wasm <WASM>` — Wasm file path on local filesystem. Provide this OR `--wasm-hash` OR `--contract-id`
* `--wasm-hash <WASM_HASH>` — Hash of Wasm blob on a network. Provide this OR `--wasm` OR `--contract-id`
* `--contract-id <CONTRACT_ID>` — Contract ID/alias on a network. Provide this OR `--wasm-hash` OR `--wasm`
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--output-dir <OUTPUT_DIR>` — Where to place generated project
* `--overwrite` — Whether to overwrite output directory if it already exists



## `stellar contract bindings python`

Generate Python bindings

**Usage:** `stellar contract bindings python`



## `stellar contract bindings java`

Generate Java bindings

**Usage:** `stellar contract bindings java`



## `stellar contract build`

Build a contract from source

Builds all crates that are referenced by the cargo manifest (Cargo.toml) that have cdylib as their crate-type. Crates are built for the wasm32 target. Unless configured otherwise, crates are built with their default features and with their release profile.

In workspaces builds all crates unless a package name is specified, or the command is executed from the sub-directory of a workspace crate.

To view the commands that will be executed, without executing them, use the --print-commands-only option.

**Usage:** `stellar contract build [OPTIONS]`

###### **Options:**

* `--manifest-path <MANIFEST_PATH>` — Path to Cargo.toml
* `--package <PACKAGE>` — Package to build

   If omitted, all packages that build for crate-type cdylib are built.
* `--profile <PROFILE>` — Build with the specified profile

  Default value: `release`
* `--features <FEATURES>` — Build with the list of features activated, space or comma separated
* `--all-features` — Build with the all features activated
* `--no-default-features` — Build with the default feature not activated
* `--out-dir <OUT_DIR>` — Directory to copy wasm files to

   If provided, wasm files can be found in the cargo target directory, and the specified directory.

   If ommitted, wasm files are written only to the cargo target directory.
* `--print-commands-only` — Print commands to build without executing them
* `--meta <META>` — Add key-value to contract meta (adds the meta to the `contractmetav0` custom section)



## `stellar contract extend`

Extend the time to live ledger of a contract-data ledger entry.

If no keys are specified the contract itself is extended.

**Usage:** `stellar contract extend [OPTIONS] --ledgers-to-extend <LEDGERS_TO_EXTEND> --source-account <SOURCE_ACCOUNT>`

###### **Options:**

* `--ledgers-to-extend <LEDGERS_TO_EXTEND>` — Number of ledgers to extend the entries
* `--ttl-ledger-only` — Only print the new Time To Live ledger
* `--id <CONTRACT_ID>` — Contract ID to which owns the data entries. If no keys provided the Contract's instance will be extended
* `--key <KEY>` — Storage key (symbols only)
* `--key-xdr <KEY_XDR>` — Storage key (base64-encoded XDR)
* `--wasm <WASM>` — Path to Wasm file of contract code to extend
* `--wasm-hash <WASM_HASH>` — Path to Wasm file of contract code to extend
* `--durability <DURABILITY>` — Storage entry durability

  Default value: `persistent`

  Possible values:
  - `persistent`:
    Persistent
  - `temporary`:
    Temporary

* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout



## `stellar contract deploy`

Deploy a wasm contract

**Usage:** `stellar contract deploy [OPTIONS] --source-account <SOURCE_ACCOUNT> <--wasm <WASM>|--wasm-hash <WASM_HASH>> [-- <CONTRACT_CONSTRUCTOR_ARGS>...]`

###### **Arguments:**

* `<CONTRACT_CONSTRUCTOR_ARGS>` — If provided, will be passed to the contract's `__constructor` function with provided arguments for that function as `--arg-name value`

###### **Options:**

* `--wasm <WASM>` — WASM file to deploy
* `--wasm-hash <WASM_HASH>` — Hash of the already installed/deployed WASM file
* `--salt <SALT>` — Custom salt 32-byte salt for the token id
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `-i`, `--ignore-checks` — Whether to ignore safety checks when deploying contracts

  Default value: `false`
* `--alias <ALIAS>` — The alias that will be used to save the contract's id. Whenever used, `--alias` will always overwrite the existing contract id configuration without asking for confirmation



## `stellar contract fetch`

Fetch a contract's Wasm binary

**Usage:** `stellar contract fetch [OPTIONS] --id <CONTRACT_ID>`

###### **Options:**

* `--id <CONTRACT_ID>` — Contract ID to fetch
* `-o`, `--out-file <OUT_FILE>` — Where to write output otherwise stdout is used
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config



## `stellar contract id`

Generate the contract id for a given contract or asset

**Usage:** `stellar contract id <COMMAND>`

###### **Subcommands:**

* `asset` — Deploy builtin Soroban Asset Contract
* `wasm` — Deploy normal Wasm Contract



## `stellar contract id asset`

Deploy builtin Soroban Asset Contract

**Usage:** `stellar contract id asset [OPTIONS] --asset <ASSET>`

###### **Options:**

* `--asset <ASSET>` — ID of the Stellar classic asset to wrap, e.g. "USDC:G...5"
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar contract id wasm`

Deploy normal Wasm Contract

**Usage:** `stellar contract id wasm [OPTIONS] --salt <SALT> --source-account <SOURCE_ACCOUNT>`

###### **Options:**

* `--salt <SALT>` — ID of the Soroban contract
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar contract info`

Access info about contracts

**Usage:** `stellar contract info <COMMAND>`

###### **Subcommands:**

* `interface` — Output the interface of a contract
* `meta` — Output the metadata stored in a contract
* `env-meta` — Output the env required metadata stored in a contract
* `build` — Output the contract build information, if available



## `stellar contract info interface`

Output the interface of a contract.

A contract's interface describes the functions, parameters, and types that the contract makes accessible to be called.

The data outputted by this command is a stream of `SCSpecEntry` XDR values. See the type definitions in [stellar-xdr](https://github.com/stellar/stellar-xdr). [See also XDR data format](https://developers.stellar.org/docs/learn/encyclopedia/data-format/xdr).

Outputs no data when no data is present in the contract.

**Usage:** `stellar contract info interface [OPTIONS] <--wasm <WASM>|--wasm-hash <WASM_HASH>|--contract-id <CONTRACT_ID>>`

###### **Options:**

* `--wasm <WASM>` — Wasm file path on local filesystem. Provide this OR `--wasm-hash` OR `--contract-id`
* `--wasm-hash <WASM_HASH>` — Hash of Wasm blob on a network. Provide this OR `--wasm` OR `--contract-id`
* `--contract-id <CONTRACT_ID>` — Contract ID/alias on a network. Provide this OR `--wasm-hash` OR `--wasm`
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--output <OUTPUT>` — Format of the output

  Default value: `rust`

  Possible values:
  - `rust`:
    Rust code output of the contract interface
  - `xdr-base64`:
    XDR output of the info entry
  - `json`:
    JSON output of the info entry (one line, not formatted)
  - `json-formatted`:
    Formatted (multiline) JSON output of the info entry




## `stellar contract info meta`

Output the metadata stored in a contract.

A contract's meta is a series of key-value pairs that the contract developer can set with any values to provided metadata about the contract. The meta also contains some information like the version of Rust SDK, and Rust compiler version.

The data outputted by this command is a stream of `SCMetaEntry` XDR values. See the type definitions in [stellar-xdr](https://github.com/stellar/stellar-xdr). [See also XDR data format](https://developers.stellar.org/docs/learn/encyclopedia/data-format/xdr).

Outputs no data when no data is present in the contract.

**Usage:** `stellar contract info meta [OPTIONS] <--wasm <WASM>|--wasm-hash <WASM_HASH>|--contract-id <CONTRACT_ID>>`

###### **Options:**

* `--wasm <WASM>` — Wasm file path on local filesystem. Provide this OR `--wasm-hash` OR `--contract-id`
* `--wasm-hash <WASM_HASH>` — Hash of Wasm blob on a network. Provide this OR `--wasm` OR `--contract-id`
* `--contract-id <CONTRACT_ID>` — Contract ID/alias on a network. Provide this OR `--wasm-hash` OR `--wasm`
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--output <OUTPUT>` — Format of the output

  Default value: `text`

  Possible values:
  - `text`:
    Text output of the meta info entry
  - `xdr-base64`:
    XDR output of the info entry
  - `json`:
    JSON output of the info entry (one line, not formatted)
  - `json-formatted`:
    Formatted (multiline) JSON output of the info entry




## `stellar contract info env-meta`

Output the env required metadata stored in a contract.

Env-meta is information stored in all contracts, in the `contractenvmetav0` WASM custom section, about the environment that the contract was built for. Env-meta allows the Soroban Env to know whether the contract is compatibility with the network in its current configuration.

The data outputted by this command is a stream of `SCEnvMetaEntry` XDR values. See the type definitions in [stellar-xdr](https://github.com/stellar/stellar-xdr). [See also XDR data format](https://developers.stellar.org/docs/learn/encyclopedia/data-format/xdr).

Outputs no data when no data is present in the contract.

**Usage:** `stellar contract info env-meta [OPTIONS] <--wasm <WASM>|--wasm-hash <WASM_HASH>|--contract-id <CONTRACT_ID>>`

###### **Options:**

* `--wasm <WASM>` — Wasm file path on local filesystem. Provide this OR `--wasm-hash` OR `--contract-id`
* `--wasm-hash <WASM_HASH>` — Hash of Wasm blob on a network. Provide this OR `--wasm` OR `--contract-id`
* `--contract-id <CONTRACT_ID>` — Contract ID/alias on a network. Provide this OR `--wasm-hash` OR `--wasm`
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--output <OUTPUT>` — Format of the output

  Default value: `text`

  Possible values:
  - `text`:
    Text output of the meta info entry
  - `xdr-base64`:
    XDR output of the info entry
  - `json`:
    JSON output of the info entry (one line, not formatted)
  - `json-formatted`:
    Formatted (multiline) JSON output of the info entry




## `stellar contract info build`

Output the contract build information, if available.

If the contract has a meta entry like `source_repo=github:user/repo`, this command will try to fetch the attestation information for the WASM file.

**Usage:** `stellar contract info build [OPTIONS] <--wasm <WASM>|--wasm-hash <WASM_HASH>|--contract-id <CONTRACT_ID>>`

###### **Options:**

* `--wasm <WASM>` — Wasm file path on local filesystem. Provide this OR `--wasm-hash` OR `--contract-id`
* `--wasm-hash <WASM_HASH>` — Hash of Wasm blob on a network. Provide this OR `--wasm` OR `--contract-id`
* `--contract-id <CONTRACT_ID>` — Contract ID/alias on a network. Provide this OR `--wasm-hash` OR `--wasm`
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar contract init`

Initialize a Soroban contract project.

This command will create a Cargo workspace project and add a sample Stellar contract. The name of the contract can be specified by `--name`. It can be run multiple times with different names in order to generate multiple contracts, and files won't be overwritten unless `--overwrite` is passed.

**Usage:** `stellar contract init [OPTIONS] <PROJECT_PATH>`

###### **Arguments:**

* `<PROJECT_PATH>`

###### **Options:**

* `--name <NAME>` — An optional flag to specify a new contract's name.

  Default value: `hello-world`
* `--overwrite` — Overwrite all existing files.



## `stellar contract inspect`

(Deprecated in favor of `contract info` subcommand) Inspect a WASM file listing contract functions, meta, etc

**Usage:** `stellar contract inspect [OPTIONS] --wasm <WASM>`

###### **Options:**

* `--wasm <WASM>` — Path to wasm binary
* `--output <OUTPUT>` — Output just XDR in base64

  Default value: `docs`

  Possible values:
  - `xdr-base64`:
    XDR of array of contract spec entries
  - `xdr-base64-array`:
    Array of xdr of contract spec entries
  - `docs`:
    Pretty print of contract spec entries

* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar contract upload`

Install a WASM file to the ledger without creating a contract instance

**Usage:** `stellar contract upload [OPTIONS] --source-account <SOURCE_ACCOUNT> --wasm <WASM>`

###### **Options:**

* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--wasm <WASM>` — Path to wasm binary
* `-i`, `--ignore-checks` — Whether to ignore safety checks when deploying contracts

  Default value: `false`



## `stellar contract install`

(Deprecated in favor of `contract upload` subcommand) Install a WASM file to the ledger without creating a contract instance

**Usage:** `stellar contract install [OPTIONS] --source-account <SOURCE_ACCOUNT> --wasm <WASM>`

###### **Options:**

* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--wasm <WASM>` — Path to wasm binary
* `-i`, `--ignore-checks` — Whether to ignore safety checks when deploying contracts

  Default value: `false`



## `stellar contract invoke`

Invoke a contract function

Generates an "implicit CLI" for the specified contract on-the-fly using the contract's schema, which gets embedded into every Soroban contract. The "slop" in this command, everything after the `--`, gets passed to this implicit CLI. Get in-depth help for a given contract:

stellar contract invoke ... -- --help

**Usage:** `stellar contract invoke [OPTIONS] --id <CONTRACT_ID> --source-account <SOURCE_ACCOUNT> [-- <CONTRACT_FN_AND_ARGS>...]`

###### **Arguments:**

* `<CONTRACT_FN_AND_ARGS>` — Function name as subcommand, then arguments for that function as `--arg-name value`

###### **Options:**

* `--id <CONTRACT_ID>` — Contract ID to invoke
* `--is-view` — View the result simulating and do not sign and submit transaction. Deprecated use `--send=no`
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--send <SEND>` — Whether or not to send a transaction

  Default value: `default`

  Possible values:
  - `default`:
    Send transaction if simulation indicates there are ledger writes, published events, or auth required, otherwise return simulation result
  - `no`:
    Do not send transaction, return simulation result
  - `yes`:
    Always send transaction




## `stellar contract optimize`

Optimize a WASM file

**Usage:** `stellar contract optimize [OPTIONS] --wasm <WASM>`

###### **Options:**

* `--wasm <WASM>` — Path to wasm binary
* `--wasm-out <WASM_OUT>` — Path to write the optimized WASM file to (defaults to same location as --wasm with .optimized.wasm suffix)



## `stellar contract read`

Print the current value of a contract-data ledger entry

**Usage:** `stellar contract read [OPTIONS]`

###### **Options:**

* `--output <OUTPUT>` — Type of output to generate

  Default value: `string`

  Possible values:
  - `string`:
    String
  - `json`:
    Json
  - `xdr`:
    XDR

* `--id <CONTRACT_ID>` — Contract ID to which owns the data entries. If no keys provided the Contract's instance will be extended
* `--key <KEY>` — Storage key (symbols only)
* `--key-xdr <KEY_XDR>` — Storage key (base64-encoded XDR)
* `--wasm <WASM>` — Path to Wasm file of contract code to extend
* `--wasm-hash <WASM_HASH>` — Path to Wasm file of contract code to extend
* `--durability <DURABILITY>` — Storage entry durability

  Default value: `persistent`

  Possible values:
  - `persistent`:
    Persistent
  - `temporary`:
    Temporary

* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar contract restore`

Restore an evicted value for a contract-data legder entry.

If no keys are specificed the contract itself is restored.

**Usage:** `stellar contract restore [OPTIONS] --source-account <SOURCE_ACCOUNT>`

###### **Options:**

* `--id <CONTRACT_ID>` — Contract ID to which owns the data entries. If no keys provided the Contract's instance will be extended
* `--key <KEY>` — Storage key (symbols only)
* `--key-xdr <KEY_XDR>` — Storage key (base64-encoded XDR)
* `--wasm <WASM>` — Path to Wasm file of contract code to extend
* `--wasm-hash <WASM_HASH>` — Path to Wasm file of contract code to extend
* `--durability <DURABILITY>` — Storage entry durability

  Default value: `persistent`

  Possible values:
  - `persistent`:
    Persistent
  - `temporary`:
    Temporary

* `--ledgers-to-extend <LEDGERS_TO_EXTEND>` — Number of ledgers to extend the entry
* `--ttl-ledger-only` — Only print the new Time To Live ledger
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout



## `stellar events`

Watch the network for contract events

**Usage:** `stellar events [OPTIONS]`

###### **Options:**

* `--start-ledger <START_LEDGER>` — The first ledger sequence number in the range to pull events https://developers.stellar.org/docs/learn/encyclopedia/network-configuration/ledger-headers#ledger-sequence
* `--cursor <CURSOR>` — The cursor corresponding to the start of the event range
* `--output <OUTPUT>` — Output formatting options for event stream

  Default value: `pretty`

  Possible values:
  - `pretty`:
    Colorful, human-oriented console output
  - `plain`:
    Human-oriented console output without colors
  - `json`:
    JSON formatted console output

* `-c`, `--count <COUNT>` — The maximum number of events to display (defer to the server-defined limit)

  Default value: `10`
* `--id <CONTRACT_IDS>` — A set of (up to 5) contract IDs to filter events on. This parameter can be passed multiple times, e.g. `--id C123.. --id C456..`, or passed with multiple parameters, e.g. `--id C123 C456`.

   Though the specification supports multiple filter objects (i.e. combinations of type, IDs, and topics), only one set can be specified on the command-line today, though that set can have multiple IDs/topics.
* `--topic <TOPIC_FILTERS>` — A set of (up to 4) topic filters to filter event topics on. A single topic filter can contain 1-4 different segment filters, separated by commas, with an asterisk (`*` character) indicating a wildcard segment.

   **Example:** topic filter with two segments: `--topic "AAAABQAAAAdDT1VOVEVSAA==,*"`

   **Example:** two topic filters with one and two segments each: `--topic "AAAABQAAAAdDT1VOVEVSAA==" --topic '*,*'`

   Note that all of these topic filters are combined with the contract IDs into a single filter (i.e. combination of type, IDs, and topics).
* `--type <EVENT_TYPE>` — Specifies which type of contract events to display

  Default value: `all`

  Possible values: `all`, `contract`, `system`

* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config



## `stellar env`

Prints the environment variables

Prints to stdout in a format that can be used as .env file. Environment variables have precedence over defaults.

Pass a name to get the value of a single environment variable.

If there are no environment variables in use, prints the defaults.

**Usage:** `stellar env [OPTIONS] [NAME]`

###### **Arguments:**

* `<NAME>` — Env variable name to get the value of.

   E.g.: $ stellar env STELLAR_ACCOUNT

###### **Options:**

* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar keys`

Create and manage identities including keys and addresses

**Usage:** `stellar keys <COMMAND>`

###### **Subcommands:**

* `add` — Add a new identity (keypair, ledger, OS specific secure store)
* `public-key` — Given an identity return its address (public key)
* `fund` — Fund an identity on a test network
* `generate` — Generate a new identity using a 24-word seed phrase The seed phrase can be stored in a config file (default) or in an OS-specific secure store
* `ls` — List identities
* `rm` — Remove an identity
* `secret` — Output an identity's secret key
* `use` — Set the default identity that will be used on all commands. This allows you to skip `--source-account` or setting a environment variable, while reusing this value in all commands that require it



## `stellar keys add`

Add a new identity (keypair, ledger, OS specific secure store)

**Usage:** `stellar keys add [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — Name of identity

###### **Options:**

* `--secret-key` — (deprecated) Enter secret (S) key when prompted
* `--seed-phrase` — (deprecated) Enter key using 12-24 word seed phrase
* `--secure-store` — Save the new key in secure store. This only supports seed phrases for now
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--public-key <PUBLIC_KEY>` — Add a public key, ed25519, or muxed account, e.g. G1.., M2..



## `stellar keys public-key`

Given an identity return its address (public key)

**Usage:** `stellar keys public-key [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — Name of identity to lookup, default test identity used if not provided

###### **Options:**

* `--hd-path <HD_PATH>` — If identity is a seed phrase use this hd path, default is 0
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar keys fund`

Fund an identity on a test network

**Usage:** `stellar keys fund [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — Name of identity to lookup, default test identity used if not provided

###### **Options:**

* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `--hd-path <HD_PATH>` — If identity is a seed phrase use this hd path, default is 0
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar keys generate`

Generate a new identity using a 24-word seed phrase The seed phrase can be stored in a config file (default) or in an OS-specific secure store

**Usage:** `stellar keys generate [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — Name of identity

###### **Options:**

* `--no-fund` — Do not fund address
* `--seed <SEED>` — Optional seed to use when generating seed phrase. Random otherwise
* `-s`, `--as-secret` — Output the generated identity as a secret key
* `--secure-store` — Save in OS-specific secure store
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--hd-path <HD_PATH>` — When generating a secret key, which `hd_path` should be used from the original `seed_phrase`
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `--fund` — Fund generated key pair

  Default value: `false`
* `--overwrite` — Overwrite existing identity if it already exists



## `stellar keys ls`

List identities

**Usage:** `stellar keys ls [OPTIONS]`

###### **Options:**

* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `-l`, `--long`



## `stellar keys rm`

Remove an identity

**Usage:** `stellar keys rm [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — Identity to remove

###### **Options:**

* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar keys secret`

Output an identity's secret key

**Usage:** `stellar keys secret [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — Name of identity to lookup, default is test identity

###### **Options:**

* `--phrase` — Output seed phrase instead of private key
* `--hd-path <HD_PATH>` — If identity is a seed phrase use this hd path, default is 0
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar keys use`

Set the default identity that will be used on all commands. This allows you to skip `--source-account` or setting a environment variable, while reusing this value in all commands that require it

**Usage:** `stellar keys use [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — Set the default network name

###### **Options:**

* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar network`

Configure connection to networks

**Usage:** `stellar network <COMMAND>`

###### **Subcommands:**

* `add` — Add a new network
* `rm` — Remove a network
* `ls` — List networks
* `start` — ⚠️ Deprecated: use `stellar container start` instead
* `stop` — ⚠️ Deprecated: use `stellar container stop` instead
* `use` — Set the default network that will be used on all commands. This allows you to skip `--network` or setting a environment variable, while reusing this value in all commands that require it
* `container` — ⚠️ Deprecated: use `stellar container` instead
* `health` — Checks the health of the configured RPC



## `stellar network add`

Add a new network

**Usage:** `stellar network add [OPTIONS] --rpc-url <RPC_URL> --network-passphrase <NETWORK_PASSPHRASE> <NAME>`

###### **Arguments:**

* `<NAME>` — Name of network

###### **Options:**

* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — Optional header (e.g. API Key) to include in requests to the RPC
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar network rm`

Remove a network

**Usage:** `stellar network rm [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — Network to remove

###### **Options:**

* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar network ls`

List networks

**Usage:** `stellar network ls [OPTIONS]`

###### **Options:**

* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `-l`, `--long` — Get more info about the networks



## `stellar network start`

⚠️ Deprecated: use `stellar container start` instead

Start network

Start a container running a Stellar node, RPC, API, and friendbot (faucet).

`stellar network start NETWORK [OPTIONS]`

By default, when starting a testnet container, without any optional arguments, it will run the equivalent of the following docker command:

`docker run --rm -p 8000:8000 --name stellar stellar/quickstart:testing --testnet --enable rpc,horizon`

**Usage:** `stellar network start [OPTIONS] [NETWORK]`

###### **Arguments:**

* `<NETWORK>` — Network to start. Default is `local`

  Possible values: `local`, `testnet`, `futurenet`, `pubnet`


###### **Options:**

* `-d`, `--docker-host <DOCKER_HOST>` — Optional argument to override the default docker host. This is useful when you are using a non-standard docker host path for your Docker-compatible container runtime, e.g. Docker Desktop defaults to $HOME/.docker/run/docker.sock instead of /var/run/docker.sock
* `--name <NAME>` — Optional argument to specify the container name
* `-l`, `--limits <LIMITS>` — Optional argument to specify the limits for the local network only
* `-p`, `--ports-mapping <PORTS_MAPPING>` — Argument to specify the `HOST_PORT:CONTAINER_PORT` mapping

  Default value: `8000:8000`
* `-t`, `--image-tag-override <IMAGE_TAG_OVERRIDE>` — Optional argument to override the default docker image tag for the given network
* `--protocol-version <PROTOCOL_VERSION>` — Optional argument to specify the protocol version for the local network only



## `stellar network stop`

⚠️ Deprecated: use `stellar container stop` instead

Stop a network started with `network start`. For example, if you ran `stellar network start local`, you can use `stellar network stop local` to stop it.

**Usage:** `stellar network stop [OPTIONS] [NAME]`

###### **Arguments:**

* `<NAME>` — Container to stop

  Default value: `local`

###### **Options:**

* `-d`, `--docker-host <DOCKER_HOST>` — Optional argument to override the default docker host. This is useful when you are using a non-standard docker host path for your Docker-compatible container runtime, e.g. Docker Desktop defaults to $HOME/.docker/run/docker.sock instead of /var/run/docker.sock



## `stellar network use`

Set the default network that will be used on all commands. This allows you to skip `--network` or setting a environment variable, while reusing this value in all commands that require it

**Usage:** `stellar network use [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — Set the default network name

###### **Options:**

* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar network container`

⚠️ Deprecated: use `stellar container` instead

Commands to start, stop and get logs for a quickstart container

**Usage:** `stellar network container <COMMAND>`

###### **Subcommands:**

* `logs` — Get logs from a running network container
* `start` — Start a container running a Stellar node, RPC, API, and friendbot (faucet)
* `stop` — Stop a network container started with `stellar container start`



## `stellar network container logs`

Get logs from a running network container

**Usage:** `stellar network container logs [OPTIONS] [NAME]`

###### **Arguments:**

* `<NAME>` — Container to get logs from

  Default value: `local`

###### **Options:**

* `-d`, `--docker-host <DOCKER_HOST>` — Optional argument to override the default docker host. This is useful when you are using a non-standard docker host path for your Docker-compatible container runtime, e.g. Docker Desktop defaults to $HOME/.docker/run/docker.sock instead of /var/run/docker.sock



## `stellar network container start`

Start a container running a Stellar node, RPC, API, and friendbot (faucet).

`stellar container start NETWORK [OPTIONS]`

By default, when starting a testnet container, without any optional arguments, it will run the equivalent of the following docker command:

`docker run --rm -p 8000:8000 --name stellar stellar/quickstart:testing --testnet --enable rpc,horizon`

**Usage:** `stellar network container start [OPTIONS] [NETWORK]`

###### **Arguments:**

* `<NETWORK>` — Network to start. Default is `local`

  Possible values: `local`, `testnet`, `futurenet`, `pubnet`


###### **Options:**

* `-d`, `--docker-host <DOCKER_HOST>` — Optional argument to override the default docker host. This is useful when you are using a non-standard docker host path for your Docker-compatible container runtime, e.g. Docker Desktop defaults to $HOME/.docker/run/docker.sock instead of /var/run/docker.sock
* `--name <NAME>` — Optional argument to specify the container name
* `-l`, `--limits <LIMITS>` — Optional argument to specify the limits for the local network only
* `-p`, `--ports-mapping <PORTS_MAPPING>` — Argument to specify the `HOST_PORT:CONTAINER_PORT` mapping

  Default value: `8000:8000`
* `-t`, `--image-tag-override <IMAGE_TAG_OVERRIDE>` — Optional argument to override the default docker image tag for the given network
* `--protocol-version <PROTOCOL_VERSION>` — Optional argument to specify the protocol version for the local network only



## `stellar network container stop`

Stop a network container started with `stellar container start`

**Usage:** `stellar network container stop [OPTIONS] [NAME]`

###### **Arguments:**

* `<NAME>` — Container to stop

  Default value: `local`

###### **Options:**

* `-d`, `--docker-host <DOCKER_HOST>` — Optional argument to override the default docker host. This is useful when you are using a non-standard docker host path for your Docker-compatible container runtime, e.g. Docker Desktop defaults to $HOME/.docker/run/docker.sock instead of /var/run/docker.sock



## `stellar network health`

Checks the health of the configured RPC

**Usage:** `stellar network health [OPTIONS]`

###### **Options:**

* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--output <OUTPUT>` — Format of the output

  Default value: `text`

  Possible values:
  - `text`:
    Text output of network health status
  - `json`:
    JSON result of the RPC request
  - `json-formatted`:
    Formatted (multiline) JSON output of the RPC request




## `stellar container`

Start local networks in containers

**Usage:** `stellar container <COMMAND>`

###### **Subcommands:**

* `logs` — Get logs from a running network container
* `start` — Start a container running a Stellar node, RPC, API, and friendbot (faucet)
* `stop` — Stop a network container started with `stellar container start`



## `stellar container logs`

Get logs from a running network container

**Usage:** `stellar container logs [OPTIONS] [NAME]`

###### **Arguments:**

* `<NAME>` — Container to get logs from

  Default value: `local`

###### **Options:**

* `-d`, `--docker-host <DOCKER_HOST>` — Optional argument to override the default docker host. This is useful when you are using a non-standard docker host path for your Docker-compatible container runtime, e.g. Docker Desktop defaults to $HOME/.docker/run/docker.sock instead of /var/run/docker.sock



## `stellar container start`

Start a container running a Stellar node, RPC, API, and friendbot (faucet).

`stellar container start NETWORK [OPTIONS]`

By default, when starting a testnet container, without any optional arguments, it will run the equivalent of the following docker command:

`docker run --rm -p 8000:8000 --name stellar stellar/quickstart:testing --testnet --enable rpc,horizon`

**Usage:** `stellar container start [OPTIONS] [NETWORK]`

###### **Arguments:**

* `<NETWORK>` — Network to start. Default is `local`

  Possible values: `local`, `testnet`, `futurenet`, `pubnet`


###### **Options:**

* `-d`, `--docker-host <DOCKER_HOST>` — Optional argument to override the default docker host. This is useful when you are using a non-standard docker host path for your Docker-compatible container runtime, e.g. Docker Desktop defaults to $HOME/.docker/run/docker.sock instead of /var/run/docker.sock
* `--name <NAME>` — Optional argument to specify the container name
* `-l`, `--limits <LIMITS>` — Optional argument to specify the limits for the local network only
* `-p`, `--ports-mapping <PORTS_MAPPING>` — Argument to specify the `HOST_PORT:CONTAINER_PORT` mapping

  Default value: `8000:8000`
* `-t`, `--image-tag-override <IMAGE_TAG_OVERRIDE>` — Optional argument to override the default docker image tag for the given network
* `--protocol-version <PROTOCOL_VERSION>` — Optional argument to specify the protocol version for the local network only



## `stellar container stop`

Stop a network container started with `stellar container start`

**Usage:** `stellar container stop [OPTIONS] [NAME]`

###### **Arguments:**

* `<NAME>` — Container to stop

  Default value: `local`

###### **Options:**

* `-d`, `--docker-host <DOCKER_HOST>` — Optional argument to override the default docker host. This is useful when you are using a non-standard docker host path for your Docker-compatible container runtime, e.g. Docker Desktop defaults to $HOME/.docker/run/docker.sock instead of /var/run/docker.sock



## `stellar snapshot`

Download a snapshot of a ledger from an archive

**Usage:** `stellar snapshot <COMMAND>`

###### **Subcommands:**

* `create` — Create a ledger snapshot using a history archive



## `stellar snapshot create`

Create a ledger snapshot using a history archive.

Filters (address, wasm-hash) specify what ledger entries to include.

Account addresses include the account, and trustlines.

Contract addresses include the related wasm, contract data.

If a contract is a Stellar asset contract, it includes the asset issuer's account and trust lines, but does not include all the trust lines of other accounts holding the asset. To include them specify the addresses of relevant accounts.

Any invalid contract id passed as `--address` will be ignored.

**Usage:** `stellar snapshot create [OPTIONS] --output <OUTPUT>`

###### **Options:**

* `--ledger <LEDGER>` — The ledger sequence number to snapshot. Defaults to latest history archived ledger
* `--address <ADDRESS>` — Account or contract address/alias to include in the snapshot
* `--wasm-hash <WASM_HASHES>` — WASM hashes to include in the snapshot
* `--output <OUTPUT>` — Format of the out file

  Possible values: `json`

* `--out <OUT>` — Out path that the snapshot is written to

  Default value: `snapshot.json`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `--archive-url <ARCHIVE_URL>` — Archive URL



## `stellar tx`

Sign, Simulate, and Send transactions

**Usage:** `stellar tx <COMMAND>`

###### **Subcommands:**

* `edit` — Edit a transaction envelope from stdin. This command respects the environment variables `STELLAR_EDITOR`, `EDITOR` and `VISUAL`, in that order
* `hash` — Calculate the hash of a transaction envelope
* `new` — Create a new transaction
* `operation` — Manipulate the operations in a transaction, including adding new operations
* `send` — Send a transaction envelope to the network
* `sign` — Sign a transaction envelope appending the signature to the envelope
* `simulate` — Simulate a transaction envelope from stdin



## `stellar tx edit`

Edit a transaction envelope from stdin. This command respects the environment variables `STELLAR_EDITOR`, `EDITOR` and `VISUAL`, in that order.

Example: Start a new edit session

$ stellar tx edit

Example: Pipe an XDR transaction envelope

$ stellar tx new manage-data --data-name hello --build-only | stellar tx edit

**Usage:** `stellar tx edit`



## `stellar tx hash`

Calculate the hash of a transaction envelope

**Usage:** `stellar tx hash [OPTIONS] [TX_XDR]`

###### **Arguments:**

* `<TX_XDR>` — Base-64 transaction envelope XDR or file containing XDR to decode, or stdin if empty

###### **Options:**

* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config



## `stellar tx new`

Create a new transaction

**Usage:** `stellar tx new <COMMAND>`

###### **Subcommands:**

* `account-merge` — Transfers the XLM balance of an account to another account and removes the source account from the ledger
* `bump-sequence` — Bumps forward the sequence number of the source account to the given sequence number, invalidating any transaction with a smaller sequence number
* `change-trust` — Creates, updates, or deletes a trustline
Learn more about trustlines
https://developers.stellar.org/docs/learn/fundamentals/stellar-data-structures/accounts#trustlines
* `create-account` — Creates and funds a new account with the specified starting balance
* `manage-data` — Sets, modifies, or deletes a data entry (name/value pair) that is attached to an account
Learn more about entries and subentries:
https://developers.stellar.org/docs/learn/fundamentals/stellar-data-structures/accounts#subentries
* `payment` — Sends an amount in a specific asset to a destination account
* `set-options` — Set option for an account such as flags, inflation destination, signers, home domain, and master key weight
Learn more about flags:
https://developers.stellar.org/docs/learn/glossary#flags
Learn more about the home domain:
https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0001.md
Learn more about signers operations and key weight:
https://developers.stellar.org/docs/learn/encyclopedia/security/signatures-multisig#multisig
* `set-trustline-flags` — Allows issuing account to configure authorization and trustline flags to an asset
The Asset parameter is of the `TrustLineAsset` type. If you are modifying a trustline to a regular asset (i.e. one in a Code:Issuer format), this is equivalent to the Asset type.
If you are modifying a trustline to a pool share, however, this is composed of the liquidity pool's unique ID.
Learn more about flags:
https://developers.stellar.org/docs/learn/glossary#flags



## `stellar tx new account-merge`

Transfers the XLM balance of an account to another account and removes the source account from the ledger

**Usage:** `stellar tx new account-merge [OPTIONS] --source-account <SOURCE_ACCOUNT> --account <ACCOUNT>`

###### **Options:**

* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--account <ACCOUNT>` — Muxed Account to merge with, e.g. `GBX...`, 'MBX...'



## `stellar tx new bump-sequence`

Bumps forward the sequence number of the source account to the given sequence number, invalidating any transaction with a smaller sequence number

**Usage:** `stellar tx new bump-sequence [OPTIONS] --source-account <SOURCE_ACCOUNT> --bump-to <BUMP_TO>`

###### **Options:**

* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--bump-to <BUMP_TO>` — Sequence number to bump to



## `stellar tx new change-trust`

Creates, updates, or deletes a trustline
Learn more about trustlines
https://developers.stellar.org/docs/learn/fundamentals/stellar-data-structures/accounts#trustlines

**Usage:** `stellar tx new change-trust [OPTIONS] --source-account <SOURCE_ACCOUNT> --line <LINE>`

###### **Options:**

* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--line <LINE>`
* `--limit <LIMIT>` — Limit for the trust line, 0 to remove the trust line

  Default value: `9223372036854775807`



## `stellar tx new create-account`

Creates and funds a new account with the specified starting balance

**Usage:** `stellar tx new create-account [OPTIONS] --source-account <SOURCE_ACCOUNT> --destination <DESTINATION>`

###### **Options:**

* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--destination <DESTINATION>` — Account Id to create, e.g. `GBX...`
* `--starting-balance <STARTING_BALANCE>` — Initial balance in stroops of the account, default 1 XLM

  Default value: `10_000_000`



## `stellar tx new manage-data`

Sets, modifies, or deletes a data entry (name/value pair) that is attached to an account
Learn more about entries and subentries:
https://developers.stellar.org/docs/learn/fundamentals/stellar-data-structures/accounts#subentries

**Usage:** `stellar tx new manage-data [OPTIONS] --source-account <SOURCE_ACCOUNT> --data-name <DATA_NAME>`

###### **Options:**

* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--data-name <DATA_NAME>` — String up to 64 bytes long. If this is a new Name it will add the given name/value pair to the account. If this Name is already present then the associated value will be modified
* `--data-value <DATA_VALUE>` — Up to 64 bytes long hex string If not present then the existing Name will be deleted. If present then this value will be set in the `DataEntry`



## `stellar tx new payment`

Sends an amount in a specific asset to a destination account

**Usage:** `stellar tx new payment [OPTIONS] --source-account <SOURCE_ACCOUNT> --destination <DESTINATION> --amount <AMOUNT>`

###### **Options:**

* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--destination <DESTINATION>` — Account to send to, e.g. `GBX...`
* `--asset <ASSET>` — Asset to send, default native, e.i. XLM

  Default value: `native`
* `--amount <AMOUNT>` — Amount of the aforementioned asset to send. e.g. `10_000_000` (1 XLM)



## `stellar tx new set-options`

Set option for an account such as flags, inflation destination, signers, home domain, and master key weight
Learn more about flags:
https://developers.stellar.org/docs/learn/glossary#flags
Learn more about the home domain:
https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0001.md
Learn more about signers operations and key weight:
https://developers.stellar.org/docs/learn/encyclopedia/security/signatures-multisig#multisig

**Usage:** `stellar tx new set-options [OPTIONS] --source-account <SOURCE_ACCOUNT>`

###### **Options:**

* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--inflation-dest <INFLATION_DEST>` — Account of the inflation destination
* `--master-weight <MASTER_WEIGHT>` — A number from 0-255 (inclusive) representing the weight of the master key. If the weight of the master key is updated to 0, it is effectively disabled
* `--low-threshold <LOW_THRESHOLD>` — A number from 0-255 (inclusive) representing the threshold this account sets on all operations it performs that have a low threshold. https://developers.stellar.org/docs/learn/encyclopedia/security/signatures-multisig#multisig
* `--med-threshold <MED_THRESHOLD>` — A number from 0-255 (inclusive) representing the threshold this account sets on all operations it performs that have a medium threshold. https://developers.stellar.org/docs/learn/encyclopedia/security/signatures-multisig#multisig
* `--high-threshold <HIGH_THRESHOLD>` — A number from 0-255 (inclusive) representing the threshold this account sets on all operations it performs that have a high threshold. https://developers.stellar.org/docs/learn/encyclopedia/security/signatures-multisig#multisig
* `--home-domain <HOME_DOMAIN>` — Sets the home domain of an account. See https://developers.stellar.org/docs/learn/encyclopedia/network-configuration/federation
* `--signer <SIGNER>` — Add, update, or remove a signer from an account
* `--signer-weight <SIGNER_WEIGHT>` — Signer weight is a number from 0-255 (inclusive). The signer is deleted if the weight is 0
* `--set-required` — When enabled, an issuer must approve an account before that account can hold its asset. https://developers.stellar.org/docs/tokens/control-asset-access#authorization-required-0x1
* `--set-revocable` — When enabled, an issuer can revoke an existing trustline's authorization, thereby freezing the asset held by an account. https://developers.stellar.org/docs/tokens/control-asset-access#authorization-revocable-0x2
* `--set-clawback-enabled` — Enables the issuing account to take back (burning) all of the asset. https://developers.stellar.org/docs/tokens/control-asset-access#clawback-enabled-0x8
* `--set-immutable` — With this setting, none of the other authorization flags (`AUTH_REQUIRED_FLAG`, `AUTH_REVOCABLE_FLAG`) can be set, and the issuing account can't be merged. https://developers.stellar.org/docs/tokens/control-asset-access#authorization-immutable-0x4
* `--clear-required`
* `--clear-revocable`
* `--clear-immutable`
* `--clear-clawback-enabled`



## `stellar tx new set-trustline-flags`

Allows issuing account to configure authorization and trustline flags to an asset
The Asset parameter is of the `TrustLineAsset` type. If you are modifying a trustline to a regular asset (i.e. one in a Code:Issuer format), this is equivalent to the Asset type.
If you are modifying a trustline to a pool share, however, this is composed of the liquidity pool's unique ID.
Learn more about flags:
https://developers.stellar.org/docs/learn/glossary#flags

**Usage:** `stellar tx new set-trustline-flags [OPTIONS] --source-account <SOURCE_ACCOUNT> --trustor <TRUSTOR> --asset <ASSET>`

###### **Options:**

* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--trustor <TRUSTOR>` — Account to set trustline flags for, e.g. `GBX...`, or alias, or muxed account, `M123...``
* `--asset <ASSET>` — Asset to set trustline flags for
* `--set-authorize` — Signifies complete authorization allowing an account to transact freely with the asset to make and receive payments and place orders
* `--set-authorize-to-maintain-liabilities` — Denotes limited authorization that allows an account to maintain current orders but not to otherwise transact with the asset
* `--set-trustline-clawback-enabled` — Enables the issuing account to take back (burning) all of the asset. See our section on Clawbacks: https://developers.stellar.org/docs/learn/encyclopedia/transactions-specialized/clawbacks
* `--clear-authorize`
* `--clear-authorize-to-maintain-liabilities`
* `--clear-trustline-clawback-enabled`



## `stellar tx operation`

Manipulate the operations in a transaction, including adding new operations

**Usage:** `stellar tx operation <COMMAND>`

###### **Subcommands:**

* `add` — Add Operation to a transaction



## `stellar tx operation add`

Add Operation to a transaction

**Usage:** `stellar tx operation add <COMMAND>`

###### **Subcommands:**

* `account-merge` — Transfers the XLM balance of an account to another account and removes the source account from the ledger
* `bump-sequence` — Bumps forward the sequence number of the source account to the given sequence number, invalidating any transaction with a smaller sequence number
* `change-trust` — Creates, updates, or deletes a trustline
Learn more about trustlines
https://developers.stellar.org/docs/learn/fundamentals/stellar-data-structures/accounts#trustlines
* `create-account` — Creates and funds a new account with the specified starting balance
* `manage-data` — Sets, modifies, or deletes a data entry (name/value pair) that is attached to an account
Learn more about entries and subentries:
https://developers.stellar.org/docs/learn/fundamentals/stellar-data-structures/accounts#subentries
* `payment` — Sends an amount in a specific asset to a destination account
* `set-options` — Set option for an account such as flags, inflation destination, signers, home domain, and master key weight
Learn more about flags:
https://developers.stellar.org/docs/learn/glossary#flags
Learn more about the home domain:
https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0001.md
Learn more about signers operations and key weight:
https://developers.stellar.org/docs/learn/encyclopedia/security/signatures-multisig#multisig
* `set-trustline-flags` — Allows issuing account to configure authorization and trustline flags to an asset
The Asset parameter is of the `TrustLineAsset` type. If you are modifying a trustline to a regular asset (i.e. one in a Code:Issuer format), this is equivalent to the Asset type.
If you are modifying a trustline to a pool share, however, this is composed of the liquidity pool's unique ID.
Learn more about flags:
https://developers.stellar.org/docs/learn/glossary#flags



## `stellar tx operation add account-merge`

Transfers the XLM balance of an account to another account and removes the source account from the ledger

**Usage:** `stellar tx operation add account-merge [OPTIONS] --source-account <SOURCE_ACCOUNT> --account <ACCOUNT> [TX_XDR]`

###### **Arguments:**

* `<TX_XDR>` — Base-64 transaction envelope XDR or file containing XDR to decode, or stdin if empty

###### **Options:**

* `--operation-source-account <OPERATION_SOURCE_ACCOUNT>` — Source account used for the operation
* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--account <ACCOUNT>` — Muxed Account to merge with, e.g. `GBX...`, 'MBX...'



## `stellar tx operation add bump-sequence`

Bumps forward the sequence number of the source account to the given sequence number, invalidating any transaction with a smaller sequence number

**Usage:** `stellar tx operation add bump-sequence [OPTIONS] --source-account <SOURCE_ACCOUNT> --bump-to <BUMP_TO> [TX_XDR]`

###### **Arguments:**

* `<TX_XDR>` — Base-64 transaction envelope XDR or file containing XDR to decode, or stdin if empty

###### **Options:**

* `--operation-source-account <OPERATION_SOURCE_ACCOUNT>` — Source account used for the operation
* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--bump-to <BUMP_TO>` — Sequence number to bump to



## `stellar tx operation add change-trust`

Creates, updates, or deletes a trustline
Learn more about trustlines
https://developers.stellar.org/docs/learn/fundamentals/stellar-data-structures/accounts#trustlines

**Usage:** `stellar tx operation add change-trust [OPTIONS] --source-account <SOURCE_ACCOUNT> --line <LINE> [TX_XDR]`

###### **Arguments:**

* `<TX_XDR>` — Base-64 transaction envelope XDR or file containing XDR to decode, or stdin if empty

###### **Options:**

* `--operation-source-account <OPERATION_SOURCE_ACCOUNT>` — Source account used for the operation
* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--line <LINE>`
* `--limit <LIMIT>` — Limit for the trust line, 0 to remove the trust line

  Default value: `9223372036854775807`



## `stellar tx operation add create-account`

Creates and funds a new account with the specified starting balance

**Usage:** `stellar tx operation add create-account [OPTIONS] --source-account <SOURCE_ACCOUNT> --destination <DESTINATION> [TX_XDR]`

###### **Arguments:**

* `<TX_XDR>` — Base-64 transaction envelope XDR or file containing XDR to decode, or stdin if empty

###### **Options:**

* `--operation-source-account <OPERATION_SOURCE_ACCOUNT>` — Source account used for the operation
* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--destination <DESTINATION>` — Account Id to create, e.g. `GBX...`
* `--starting-balance <STARTING_BALANCE>` — Initial balance in stroops of the account, default 1 XLM

  Default value: `10_000_000`



## `stellar tx operation add manage-data`

Sets, modifies, or deletes a data entry (name/value pair) that is attached to an account
Learn more about entries and subentries:
https://developers.stellar.org/docs/learn/fundamentals/stellar-data-structures/accounts#subentries

**Usage:** `stellar tx operation add manage-data [OPTIONS] --source-account <SOURCE_ACCOUNT> --data-name <DATA_NAME> [TX_XDR]`

###### **Arguments:**

* `<TX_XDR>` — Base-64 transaction envelope XDR or file containing XDR to decode, or stdin if empty

###### **Options:**

* `--operation-source-account <OPERATION_SOURCE_ACCOUNT>` — Source account used for the operation
* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--data-name <DATA_NAME>` — String up to 64 bytes long. If this is a new Name it will add the given name/value pair to the account. If this Name is already present then the associated value will be modified
* `--data-value <DATA_VALUE>` — Up to 64 bytes long hex string If not present then the existing Name will be deleted. If present then this value will be set in the `DataEntry`



## `stellar tx operation add payment`

Sends an amount in a specific asset to a destination account

**Usage:** `stellar tx operation add payment [OPTIONS] --source-account <SOURCE_ACCOUNT> --destination <DESTINATION> --amount <AMOUNT> [TX_XDR]`

###### **Arguments:**

* `<TX_XDR>` — Base-64 transaction envelope XDR or file containing XDR to decode, or stdin if empty

###### **Options:**

* `--operation-source-account <OPERATION_SOURCE_ACCOUNT>` — Source account used for the operation
* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--destination <DESTINATION>` — Account to send to, e.g. `GBX...`
* `--asset <ASSET>` — Asset to send, default native, e.i. XLM

  Default value: `native`
* `--amount <AMOUNT>` — Amount of the aforementioned asset to send. e.g. `10_000_000` (1 XLM)



## `stellar tx operation add set-options`

Set option for an account such as flags, inflation destination, signers, home domain, and master key weight
Learn more about flags:
https://developers.stellar.org/docs/learn/glossary#flags
Learn more about the home domain:
https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0001.md
Learn more about signers operations and key weight:
https://developers.stellar.org/docs/learn/encyclopedia/security/signatures-multisig#multisig

**Usage:** `stellar tx operation add set-options [OPTIONS] --source-account <SOURCE_ACCOUNT> [TX_XDR]`

###### **Arguments:**

* `<TX_XDR>` — Base-64 transaction envelope XDR or file containing XDR to decode, or stdin if empty

###### **Options:**

* `--operation-source-account <OPERATION_SOURCE_ACCOUNT>` — Source account used for the operation
* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--inflation-dest <INFLATION_DEST>` — Account of the inflation destination
* `--master-weight <MASTER_WEIGHT>` — A number from 0-255 (inclusive) representing the weight of the master key. If the weight of the master key is updated to 0, it is effectively disabled
* `--low-threshold <LOW_THRESHOLD>` — A number from 0-255 (inclusive) representing the threshold this account sets on all operations it performs that have a low threshold. https://developers.stellar.org/docs/learn/encyclopedia/security/signatures-multisig#multisig
* `--med-threshold <MED_THRESHOLD>` — A number from 0-255 (inclusive) representing the threshold this account sets on all operations it performs that have a medium threshold. https://developers.stellar.org/docs/learn/encyclopedia/security/signatures-multisig#multisig
* `--high-threshold <HIGH_THRESHOLD>` — A number from 0-255 (inclusive) representing the threshold this account sets on all operations it performs that have a high threshold. https://developers.stellar.org/docs/learn/encyclopedia/security/signatures-multisig#multisig
* `--home-domain <HOME_DOMAIN>` — Sets the home domain of an account. See https://developers.stellar.org/docs/learn/encyclopedia/network-configuration/federation
* `--signer <SIGNER>` — Add, update, or remove a signer from an account
* `--signer-weight <SIGNER_WEIGHT>` — Signer weight is a number from 0-255 (inclusive). The signer is deleted if the weight is 0
* `--set-required` — When enabled, an issuer must approve an account before that account can hold its asset. https://developers.stellar.org/docs/tokens/control-asset-access#authorization-required-0x1
* `--set-revocable` — When enabled, an issuer can revoke an existing trustline's authorization, thereby freezing the asset held by an account. https://developers.stellar.org/docs/tokens/control-asset-access#authorization-revocable-0x2
* `--set-clawback-enabled` — Enables the issuing account to take back (burning) all of the asset. https://developers.stellar.org/docs/tokens/control-asset-access#clawback-enabled-0x8
* `--set-immutable` — With this setting, none of the other authorization flags (`AUTH_REQUIRED_FLAG`, `AUTH_REVOCABLE_FLAG`) can be set, and the issuing account can't be merged. https://developers.stellar.org/docs/tokens/control-asset-access#authorization-immutable-0x4
* `--clear-required`
* `--clear-revocable`
* `--clear-immutable`
* `--clear-clawback-enabled`



## `stellar tx operation add set-trustline-flags`

Allows issuing account to configure authorization and trustline flags to an asset
The Asset parameter is of the `TrustLineAsset` type. If you are modifying a trustline to a regular asset (i.e. one in a Code:Issuer format), this is equivalent to the Asset type.
If you are modifying a trustline to a pool share, however, this is composed of the liquidity pool's unique ID.
Learn more about flags:
https://developers.stellar.org/docs/learn/glossary#flags

**Usage:** `stellar tx operation add set-trustline-flags [OPTIONS] --source-account <SOURCE_ACCOUNT> --trustor <TRUSTOR> --asset <ASSET> [TX_XDR]`

###### **Arguments:**

* `<TX_XDR>` — Base-64 transaction envelope XDR or file containing XDR to decode, or stdin if empty

###### **Options:**

* `--operation-source-account <OPERATION_SOURCE_ACCOUNT>` — Source account used for the operation
* `--fee <FEE>` — fee amount for transaction, in stroops. 1 stroop = 0.0000001 xlm

  Default value: `100`
* `--cost` — Output the cost execution to stderr
* `--instructions <INSTRUCTIONS>` — Number of instructions to simulate
* `--build-only` — Build the transaction and only write the base64 xdr to stdout
* `--sim-only` — (Deprecated) simulate the transaction and only write the base64 xdr to stdout
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `--trustor <TRUSTOR>` — Account to set trustline flags for, e.g. `GBX...`, or alias, or muxed account, `M123...``
* `--asset <ASSET>` — Asset to set trustline flags for
* `--set-authorize` — Signifies complete authorization allowing an account to transact freely with the asset to make and receive payments and place orders
* `--set-authorize-to-maintain-liabilities` — Denotes limited authorization that allows an account to maintain current orders but not to otherwise transact with the asset
* `--set-trustline-clawback-enabled` — Enables the issuing account to take back (burning) all of the asset. See our section on Clawbacks: https://developers.stellar.org/docs/learn/encyclopedia/transactions-specialized/clawbacks
* `--clear-authorize`
* `--clear-authorize-to-maintain-liabilities`
* `--clear-trustline-clawback-enabled`



## `stellar tx send`

Send a transaction envelope to the network

**Usage:** `stellar tx send [OPTIONS] [TX_XDR]`

###### **Arguments:**

* `<TX_XDR>` — Base-64 transaction envelope XDR or file containing XDR to decode, or stdin if empty

###### **Options:**

* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar tx sign`

Sign a transaction envelope appending the signature to the envelope

**Usage:** `stellar tx sign [OPTIONS] [TX_XDR]`

###### **Arguments:**

* `<TX_XDR>` — Base-64 transaction envelope XDR, or file containing XDR to decode, or stdin if empty

###### **Options:**

* `--sign-with-key <SIGN_WITH_KEY>` — Sign with a local key or key saved in OS secure storage. Can be an identity (--sign-with-key alice), a secret key (--sign-with-key SC36…), or a seed phrase (--sign-with-key "kite urban…"). If using seed phrase, `--hd-path` defaults to the `0` path
* `--hd-path <HD_PATH>` — If using a seed phrase to sign, sets which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--sign-with-lab` — Sign with https://lab.stellar.org
* `--sign-with-ledger` — Sign with a ledger wallet
* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar tx simulate`

Simulate a transaction envelope from stdin

**Usage:** `stellar tx simulate [OPTIONS] --source-account <SOURCE_ACCOUNT> [TX_XDR]`

###### **Arguments:**

* `<TX_XDR>` — Base-64 transaction envelope XDR or file containing XDR to decode, or stdin if empty

###### **Options:**

* `--rpc-url <RPC_URL>` — RPC server endpoint
* `--rpc-header <RPC_HEADERS>` — RPC Header(s) to include in requests to the RPC provider
* `--network-passphrase <NETWORK_PASSPHRASE>` — Network passphrase to sign the transaction sent to the rpc server
* `-n`, `--network <NETWORK>` — Name of network to use from config
* `-s`, `--source-account <SOURCE_ACCOUNT>` — Account that where transaction originates from. Alias `source`. Can be an identity (--source alice), a public key (--source GDKW...), a muxed account (--source MDA…), a secret key (--source SC36…), or a seed phrase (--source "kite urban…"). If `--build-only` or `--sim-only` flags were NOT provided, this key will also be used to sign the final transaction. In that case, trying to sign with public key will fail
* `--hd-path <HD_PATH>` — If using a seed phrase, which hierarchical deterministic path to use, e.g. `m/44'/148'/{hd_path}`. Example: `--hd-path 1`. Default: `0`
* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."



## `stellar xdr`

Decode and encode XDR

**Usage:** `stellar xdr [CHANNEL] <COMMAND>`

###### **Subcommands:**

* `types` — View information about types
* `guess` — Guess the XDR type
* `decode` — Decode XDR
* `encode` — Encode XDR
* `compare` — Compare two XDR values with each other
* `version` — Print version information

###### **Arguments:**

* `<CHANNEL>` — Channel of XDR to operate on

  Default value: `+curr`

  Possible values: `+curr`, `+next`




## `stellar xdr types`

View information about types

**Usage:** `stellar xdr types <COMMAND>`

###### **Subcommands:**

* `list` — 
* `schema` — 



## `stellar xdr types list`

**Usage:** `stellar xdr types list [OPTIONS]`

###### **Options:**

* `--output <OUTPUT>`

  Default value: `plain`

  Possible values: `plain`, `json`, `json-formatted`




## `stellar xdr types schema`

**Usage:** `stellar xdr types schema [OPTIONS] --type <TYPE>`

###### **Options:**

* `--type <TYPE>` — XDR type to decode
* `--output <OUTPUT>`

  Default value: `json-schema-draft201909`

  Possible values: `json-schema-draft7`, `json-schema-draft201909`




## `stellar xdr guess`

Guess the XDR type

**Usage:** `stellar xdr guess [OPTIONS] [FILE]`

###### **Arguments:**

* `<FILE>` — File to decode, or stdin if omitted

###### **Options:**

* `--input <INPUT>`

  Default value: `single-base64`

  Possible values: `single`, `single-base64`, `stream`, `stream-base64`, `stream-framed`

* `--output <OUTPUT>`

  Default value: `list`

  Possible values: `list`

* `--certainty <CERTAINTY>` — Certainty as an arbitrary value

  Default value: `2`



## `stellar xdr decode`

Decode XDR

**Usage:** `stellar xdr decode [OPTIONS] --type <TYPE> [FILES]...`

###### **Arguments:**

* `<FILES>` — Files to decode, or stdin if omitted

###### **Options:**

* `--type <TYPE>` — XDR type to decode
* `--input <INPUT>`

  Default value: `stream-base64`

  Possible values: `single`, `single-base64`, `stream`, `stream-base64`, `stream-framed`

* `--output <OUTPUT>`

  Default value: `json`

  Possible values: `json`, `json-formatted`, `rust-debug`, `rust-debug-formatted`




## `stellar xdr encode`

Encode XDR

**Usage:** `stellar xdr encode [OPTIONS] --type <TYPE> [FILES]...`

###### **Arguments:**

* `<FILES>` — Files to encode, or stdin if omitted

###### **Options:**

* `--type <TYPE>` — XDR type to encode
* `--input <INPUT>`

  Default value: `json`

  Possible values: `json`

* `--output <OUTPUT>`

  Default value: `single-base64`

  Possible values: `single`, `single-base64`, `stream`




## `stellar xdr compare`

Compare two XDR values with each other

Outputs: `-1` when the left XDR value is less than the right XDR value, `0` when the left XDR value is equal to the right XDR value, `1` when the left XDR value is greater than the right XDR value

**Usage:** `stellar xdr compare [OPTIONS] --type <TYPE> <LEFT> <RIGHT>`

###### **Arguments:**

* `<LEFT>` — XDR file to decode and compare with the right value
* `<RIGHT>` — XDR file to decode and compare with the left value

###### **Options:**

* `--type <TYPE>` — XDR type of both inputs
* `--input <INPUT>`

  Default value: `single-base64`

  Possible values: `single`, `single-base64`




## `stellar xdr version`

Print version information

**Usage:** `stellar xdr version`



## `stellar completion`

Print shell completion code for the specified shell

Ensure the completion package for your shell is installed, e.g. bash-completion for bash.

To enable autocomplete in the current bash shell, run: `source <(stellar completion --shell bash)`

To enable autocomplete permanently, run: `echo "source <(stellar completion --shell bash)" >> ~/.bashrc`


**Usage:** `stellar completion --shell <SHELL>`

###### **Options:**

* `--shell <SHELL>` — The shell type

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`




## `stellar cache`

Cache for transactions and contract specs

**Usage:** `stellar cache <COMMAND>`

###### **Subcommands:**

* `clean` — Delete the cache
* `path` — Show the location of the cache
* `actionlog` — Access details about cached actions like transactions, and simulations. (Experimental. May see breaking changes at any time.)



## `stellar cache clean`

Delete the cache

**Usage:** `stellar cache clean`



## `stellar cache path`

Show the location of the cache

**Usage:** `stellar cache path`



## `stellar cache actionlog`

Access details about cached actions like transactions, and simulations. (Experimental. May see breaking changes at any time.)

**Usage:** `stellar cache actionlog <COMMAND>`

###### **Subcommands:**

* `ls` — List cached actions (transactions, simulations)
* `read` — Read cached action



## `stellar cache actionlog ls`

List cached actions (transactions, simulations)

**Usage:** `stellar cache actionlog ls [OPTIONS]`

###### **Options:**

* `--global` — Use global config
* `--config-dir <CONFIG_DIR>` — Location of config directory, default is "."
* `-l`, `--long`



## `stellar cache actionlog read`

Read cached action

**Usage:** `stellar cache actionlog read --id <ID>`

###### **Options:**

* `--id <ID>` — ID of the cache entry



## `stellar version`

Print version information

**Usage:** `stellar version`



