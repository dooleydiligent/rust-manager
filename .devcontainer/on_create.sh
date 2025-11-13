#!/bin/bash 
set -euo pipefail
export DEBIAN_FRONTEND=noninteractive

apt update -y && apt upgrade -y 

apt install -y \
  libvirt-daemon-system \
  libvirt-clients \
  qemu-kvm \
  qemu-system \
  libvirt-dev \
  virt-manager \
  ca-certificates \
  curl \
  procps

# Ensure libvirt group exists and add the current user (root or dev user)
if getent group libvirt >/dev/null; then
  echo "libvirt group exists"
else
  groupadd -r libvirt || true
fi

if id -u "${USER:-root}" >/dev/null 2>&1; then
  usermod -aG libvirt "${USER:-root}" || true
fi

# Ensure /var/run/libvirt exists
mkdir -p /var/run/libvirt
chown root:libvirt /var/run/libvirt || true
chmod 0775 /var/run/libvirt || true


# Try to start libvirtd. If systemd is present use systemctl, otherwise run libvirtd directly.
if command -v systemctl >/dev/null 2>&1 && pidof systemd >/dev/null 2>&1; then
  echo "Starting libvirtd via systemctl"
  systemctl enable --now libvirtd || true
else
  echo "No systemd detected — starting libvirtd in background"
  if [ -x /usr/sbin/libvirtd ]; then
    # redirect logs so process keeps running in background
    nohup /usr/sbin/libvirtd >/var/log/libvirtd.log 2>&1 &
    sleep 1
  else
    echo "Warning: /usr/sbin/libvirtd not found"
  fi
fi

# Adjust /dev/kvm permissions (host must have exposed it via runArgs)
if [ -e /dev/kvm ]; then
  chgrp kvm /dev/kvm 2>/dev/null || true
  chmod g+rw /dev/kvm 2>/dev/null || true
fi


# && npx tailwindcss -i ./input.css -o ./assets/tailwind.css
cargo install cargo-binstall
cargo binstall dioxus-cli --version 0.6.3 -y
cargo fetch

curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
nvm install node
npm i -g @tailwindcss/cli && npm i tailwindcss 

echo "Initializing tailwindcss"
npx tailwindcss -i ./input.css -o ./assets/tailwind.css

echo "Watch with:"
echo "npx tailwindcss -i ./input.css -o ./assets/tailwind.css -w"

echo "Building the project"
dx bundle --platform web
echo "Watch with:"
echo "dx serve --platform web"
