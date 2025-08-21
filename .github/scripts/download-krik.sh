#!/bin/bash

# Download Krik binary for GitHub Actions
# This script tries to download a pre-built binary and falls back to compilation if needed

set -e  # Exit on any error

GITHUB_REPO="mcaserta/krik"
BINARY_NAME="kk-linux-x86_64"
TARGET_PATH="/usr/local/bin/kk"
FALLBACK_NEEDED=false

echo "🚀 Setting up Krik..."

# Function to install from source
install_from_source() {
    echo "🔨 Installing Rust and compiling Krik from source..."
    
    # Install Rust if not present
    if ! command -v cargo &> /dev/null; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
    fi
    
    # Install Krik
    cargo install krik
    echo "✅ Krik compiled and installed from source"
}

# Function to verify binary works
verify_binary() {
    if command -v kk &> /dev/null && kk --version &> /dev/null; then
        echo "✅ Krik binary verification successful"
        kk --version
        return 0
    else
        echo "❌ Binary verification failed"
        return 1
    fi
}

# Try to download pre-built binary
echo "📦 Attempting to download pre-built Krik binary..."
if wget -q "https://github.com/${GITHUB_REPO}/releases/latest/download/${BINARY_NAME}"; then
    echo "✅ Downloaded pre-built Krik binary"
    
    # Make executable and move to target location
    chmod +x "${BINARY_NAME}"
    sudo mv "${BINARY_NAME}" "${TARGET_PATH}"
    
    # Verify the binary works
    if verify_binary; then
        echo "🎉 Pre-built binary setup completed successfully!"
        exit 0
    else
        echo "⚠️ Pre-built binary verification failed, removing and falling back to compilation"
        sudo rm -f "${TARGET_PATH}"
        FALLBACK_NEEDED=true
    fi
else
    echo "📦 Pre-built binary not available (likely first release), will compile from source"
    FALLBACK_NEEDED=true
fi

# Fallback to source compilation
if [ "${FALLBACK_NEEDED}" = true ]; then
    install_from_source
    
    # Final verification
    if verify_binary; then
        echo "🎉 Source compilation setup completed successfully!"
    else
        echo "💥 Failed to install Krik - both binary download and source compilation failed"
        exit 1
    fi
fi