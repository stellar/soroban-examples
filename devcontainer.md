## Running Devcontainers

Building the Devcontainer locally.

### Build image locally

Requires:
- [Devcontainer CLI](https://github.com/devcontainers/cli)

**Local build script:**
```
./build-local-devcontainer.sh
```

### Build and Run Remote Devcontainer on Github Codespaces

Run in browser.

**Launch via Link:**
- <a href="https://github.com/codespaces/new?repo=stellar/soroban-examples">Run on Codespaces</a>
- Or click "Open in Codespaces" button in [ ** README ️** ](./README.md)

**Launch via Github CLI:**

Requires:
- [Github CLI](https://cli.github.com/)

Install Github CLI:
```
npm install -g @devcontainers/cli
```

Create new Codespace:
``` 
# Create a new Codespace
gh codespace create --repo stellar/soroban-examples -b main --status --web -l WestUs2
```

Delete existing Codespaces:
```
# Delete all Codespaces (confirmation auto-approved)
echo 'Y' | gh codespace delete --all --force
```

View Remote Logs or SSH into Codespaces Devcontainer from local:
```
gh codespace ssh
gh codespace logs
```

### Use Remote Devcontainer on Github Codespaces as Local VS Code IDE backend

Run in your local VS Code IDE with Codespaces backend with all of your local settings in place.

- Install [Devcontainers VSCode ext](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
- Install [Codespaces VSCode ext](https://marketplace.visualstudio.com/items?itemName=GitHub.codespaces)
- Open `Remote Explorer` View
- Select `Github Codespaces` from dropdown
- Click `Create Codespace` button
- Enter name of repository `stellar/soroban-examples`

![img.png](assets/devcontainer-vscode-local.png)

## Devcontainer Build Details

**Base Image:**
- [mcr.microsoft.com/devcontainers/rust:bullseye](https://hub.docker.com/r/microsoft/devcontainers-rust)
  - Architecture:  `arm64/aarch64`
  - Container OS:  Debian

**Features:**
- [ghcr.io/devcontainers/features/common-utils:2](https://github.com/devcontainers/features/tree/main/src/common-utils)
  - Installs CLI utils for zsh
  - Sets non-root user and UID
  - Upgrades packages

- [ghcr.io/devcontainers/features/rust:1](https://github.com/devcontainers/features/tree/main/src/rust)
  - Sets rustup install profile
  - Sets rust targets
  - Adds VSCode extensions for Rust dev
  - Update containerEnv for: `CARGO_HOME`, `RUSTUP_HOME` and adds Cargo to `PATH`
  - Adds `SYS_PTRACE` Capability which enables `LLDB` debugging in Linux
  - Adds `seccomp=unconfined` during the build process to allow full range of system calls

- [ghcr.io/lee-orr/rusty-dev-containers/cargo-binstall:0](https://github.com/lee-orr/rusty-dev-containers/tree/main/src/cargo-binstall)
  - Used to install the Stellar CLI and sccache
  - More lightweight then Linux Homebrew
  - Linux Homebrew is no longer supported in Devcontainers(See below)
  - Why not use Linux Homebrew?
    - Issue opened on [Homebrew Devcontainers Feature](https://github.com/meaningful-ooo/devcontainer-features/issues/68)

```
[2025-03-29T21:31:42.982Z] #27 4100.8 Error: 
[2025-03-29T21:31:43.114Z] Running Homebrew as root is extremely dangerous and no longer supported.
#27 4100.8 As Homebrew does not drop privileges on installation you would be giving all
#27 4100.8 build scripts full access to your system.
#27 4100.9 Done!
```
- [ghcr.io/lee-orr/rusty-dev-containers/sccache:0](https://github.com/lee-orr/rusty-dev-containers/tree/main/src/sccache)
  - Used as a compiler wrapper and avoids compilation when possible, storing cached results
  - More info on [sccache](https://github.com/mozilla/sccache)

**ENV Vars:**
- `ENV CARGO_INCREMENTAL=0`
  - Force disabling of incremental compilation
  - Rationale:
    - Dependencies unchanged: Layer cache hit (No need for incremental compilation)
    - Dependencies changed: Layer cache miss and complete rebuild (No use for incremental compilation)
    - Incremental compilation in Devcontainers generates extra artifacts that make caching layers bigger 
      with no benefit
- `ENV RUSTFLAGS='-C target-cpu=native'`
  - Rust will generate and optimize code for the CPU running the compiler
  - Rationale:
    - Best practice when building programs which you plan to ONLY use locally
    - Should NOT be used when packaging for distribution or cross-compiling
- `ENV DEBIAN_FRONTEND=noninteractive`
  - Accepts the default answer for all questions for apt-get installs

## Learn about Devcontainers

- [Devcontainers Briefing](https://github.com/anataliocs/comprehensive-devcontainer/blob/main/briefing.md)
- [Devcontainers Properties Reference](https://github.com/anataliocs/comprehensive-devcontainer/blob/main/README.md)
- [Devcontainers devcontainer.json Reference](https://containers.dev/implementors/json_reference/)
- [Available Features](https://containers.dev/features)