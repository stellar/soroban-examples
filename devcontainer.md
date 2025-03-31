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

View Logs or SSH into Codespaces Devcontainer:
```
gh codespace ssh
gh codespace logs
```

### Use Remote Devcontainer on Github Codespaces in Local VS Code IDE

Run in your local VS Code IDE with Codespaces backend.

- Install [Devcontainers VSCode ext](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
- Install [Codespaces VSCode ext](https://marketplace.visualstudio.com/items?itemName=GitHub.codespaces)
- Open `Remote Explorer` View
- Select `Github Codespaces` from dropdown
- Click `Create Codespace` button
- Enter name of repository `stellar/soroban-examples`

![img.png](assets/devcontainer-vscode-local.png)