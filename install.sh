#!/bin/bash
#
# This script downloads and installs the latest release of fernctl.

set -e

# Determine the target triple based on OS and architecture.
OS="$(uname -s)"
ARCH="$(uname -m)"

if [[ "$OS" == "Darwin" ]]; then
  if [[ "$ARCH" == "arm64" ]]; then
    TARGET="aarch64-apple-darwin"
  else
    TARGET="x86_64-apple-darwin"
  fi
elif [[ "$OS" == "Linux" ]]; then
  TARGET="x86_64-unknown-linux-gnu"
else
  echo "Unsupported operating system: $OS" >&2
  exit 1
fi

# Get the download URL for the correct binary.
DOWNLOAD_URL=$(curl -sL https://api.github.com/repos/BenSimmers/fernctl/releases/latest \
  | grep "browser_download_url.*fernctl-$TARGET" \
  | cut -d : -f 2,3 \
  | tr -d '"[:space:]')

if [ -z "$DOWNLOAD_URL" ]; then
  echo "Could not find a release for $TARGET. Please check https://github.com/BenSimmers/fernctl/releases" >&2
  exit 1
fi

# Download the binary and make it executable.
echo "Downloading fernctl for $TARGET..."
curl -sL -o fernctl "$DOWNLOAD_URL"
chmod +x fernctl

echo "fernctl has been installed successfully. You can now run it with ./fernctl"
