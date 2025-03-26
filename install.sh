if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  PLATFORM="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
  PLATFORM="macos"
else
  echo "Unsupported platform. Please install manually."
  exit 1
fi

# Download the latest release
VERSION="0.2.91"
DOWNLOAD_URL="https://github.com/TeenCoder159/lion/releases/download/v${VERSION}/lion-${PLATFORM}"
INSTALL_DIR="/usr/local/bin"

# Create temp directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

echo "Downloading Lion CLI..."
curl -L -o lion "$DOWNLOAD_URL"
chmod +x lion

# Install
echo "Installing Lion CLI..."
sudo mkdir -p "$INSTALL_DIR"
sudo mv lion "$INSTALL_DIR/"

# Create symlink for convenience
sudo ln -sf "$INSTALL_DIR/lion" "$INSTALL_DIR/lion-cli"

echo "Lion CLI installed successfully! Run 'lion help' to get started."
