# Podman Dev Container Setup for Dioxus Fullstack

This dev container is optimized for **Podman** (both rootless and root modes). Below is Podman-specific setup and troubleshooting guidance.

## Prerequisites

- **Podman** v3.4+ (this setup has been tested with v3.4.4)
- **VS Code** with the "Dev Containers" extension (formerly "Remote - Containers")
- **Dev Containers extension configured to use Podman** (not Docker)

### Verify Podman is installed and working
```bash
podman --version
podman info | head -20
```

Expected output should show Podman version and host info (including `rootless: true` or `rootless: false`).

## Setting up VS Code to use Podman

1. Open VS Code **Settings** (Ctrl+, or Cmd+,)
2. Search for "Dev Containers" settings
3. Find the setting **Dev > Containers: Docker Path** or similar
4. Change it to point to `podman` instead of `docker`:
   ```
   podman
   ```
5. Restart VS Code

Alternatively, set the environment variable:
```bash
export DEVCONTAINERS_DOCKER_CLI=podman
```

## Opening the Dev Container

1. In VS Code, open the Command Palette (Ctrl+Shift+P)
2. Run **"Dev Containers: Reopen in Container"** or **"Dev Containers: Rebuild and Reopen in Container"**
3. Wait for the container to build and start (this may take a few minutes on first run)

The container will:
- Pull the `rust:1.90-bookworm` image
- Create a named volume `dioxus-cargo-cache` for Rust build artifacts
- Mount your workspace into `/workspaces/Dioxus-fullstack-Auth`
- Run `.devcontainer/on_create.sh` to install Dioxus CLI, Tailwind CSS, and other dependencies

## Rootless Podman Notes

This setup works with **rootless Podman** (no root privileges required on the host):

- Container runs as `root` inside the container (remoteUser: "root")
- Host user is mapped to container root via UID mapping
- Cargo cache is persisted across container restarts via a named volume

### User/Group Considerations

If you get permission errors:
- Ensure your host user can run Podman: `podman run --rm hello-world`
- If not, add your user to the podman group: `sudo usermod -aG podman $USER` (then log out/in)

## Troubleshooting

### Container fails to open with mount or path errors

**Error: "Can't resolve mount" or "workspaceMount invalid"**
- Solution: The `devcontainer.json` has been fixed to use standard Podman mount syntax. Rebuild the container.

### Tailwind CSS generation fails

**Error: "Can't resolve 'tailwindcss'" or "npx: command not found"**
- The setup script installs Tailwind locally in the workspace (`npm install` without `-g`)
- If this persists, check that `package.json` exists and `npm install` completed in the container
- Run in container terminal: `npm list tailwindcss`

### Cargo builds are slow on first run

- First build downloads ~500MB of Rust dependencies
- These are cached in the `dioxus-cargo-cache` Podman volume for subsequent runs
- Restarting the container will reuse the cache

### VS Code extensions not installing inside container

- Extensions listed in `devcontainer.json` under `customizations.vscode.extensions` are installed by the Dev Containers extension automatically
- If missing after container opens, manually install from the VS Code Extensions panel inside the container

## Building and Running

Once inside the container, use:

```bash
# Start dev server with live reload
dx serve --platform web

# Build for production
dx build --platform web

# Run tests
cargo test

# Generate Tailwind CSS (if styles changed)
npm run build
```

## Cleanup

To remove the Podman container and volume:

```bash
# Stop and remove the container
podman ps -a --filter ancestor=rust:1.90-bookworm
podman rm <container-id>

# Remove the Cargo cache volume (optional)
podman volume rm dioxus-cargo-cache
```

Then, in VS Code: **"Dev Containers: Reopen Locally"** to exit the container and return to local mode.

## Additional Resources

- [Podman Documentation](https://podman.io/docs)
- [VS Code Dev Containers Guide](https://code.visualstudio.com/docs/remote/containers)
- [Dioxus Documentation](https://dioxuslabs.com/)
