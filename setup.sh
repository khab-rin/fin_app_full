#!/bin/bash

set -e

echo "🚀 Welcome to the Setup!"
echo "----------------------------------------------------------------"
echo "⚠️  IMPORTANT: Docker must be installed manually before running this script!"
echo "----------------------------------------------------------------"
echo "This script will automatically configure your environment and install:"
echo "  📦 Basic Linux GUI dependencies (GTK, WebKit, Build Essentials)"
echo "  🦀 Rust Toolchain & Rust Analyzer (if missing)"
echo "  ⚡ Bun Package Manager & Frontend dependencies (if missing)"
echo "  🚀 Tauri CLI & SQLx CLI (if missing)"
echo "  🐳 Infrastructure containers, local databases, and migrations"
echo "----------------------------------------------------------------"
read -r -p "🤔 Do you have Docker installed and want to proceed? (y/n): " main_choice


if [[ ! "$main_choice" =~ ^[Yy]$ ]]; then
    echo "❌ Setup aborted by user."
    exit 0
fi

echo "🚀 Setup statrting..."


if ! command -v docker &> /dev/null; then
    echo "❌ Docker CLI is missing!"
    echo "💡 Install Docker Desktop and enable WSL Integration."
    exit 1
fi

echo "🐳 Checking Docker daemon..."

for i in {1..15}; do
    if docker info &>/dev/null; then
        break
    fi

    echo "⏳ Waiting for Docker Desktop... ($i/15)"
    sleep 2
done

if ! docker info &>/dev/null; then
    echo "❌ Docker daemon is not running or WSL Integration is disabled!"
    echo "💡 Start Docker Desktop and enable WSL Integration for this distro."
    exit 1
fi

if ! docker compose version &>/dev/null; then
    echo "❌ Docker Compose plugin is missing!"
    exit 1
fi

echo "✅ Docker is running and available."


sudo apt-get update && sudo apt-get install -y \
    gnome-keyring \
    libsecret-tools \
    dbus-user-session \
    libdbus-1-dev \
	unzip || {
    echo "ERROR: Failed to install dependencies."
    exit 1
}


echo "--------------------------------------------------"
echo "🔐 INITIALIZING SECURE STORAGE..."
echo "--------------------------------------------------"
echo "⚠️  NOTE FOR PASSWORD PROMPTS:"
echo "1. If Keyring is NOT configured yet: You will be asked"
echo "   to create a password TWICE to initialize the storage."
echo "2. If Keyring IS already configured: You will be asked"
echo "   to enter your password ONCE to unlock the test entry."
echo "--------------------------------------------------"

printf '%s' 'test-secret-123' | secret-tool store \
    --label="Keyring test" \
    app test-app key test-token


secret-tool lookup app test-app key test-token

secret-tool clear app test-app key test-token

echo "--------------------------------------------------"
echo "✅ Secure storage verification step passed!"
echo "--------------------------------------------------"


echo "📦 Installing basic Linux system dependencies (requires sudo)..."
sudo apt-get install -y \
	build-essential \
    curl \
    wget \
    pkg-config \
    libssl-dev \
    sqlite3 \
    libsqlite3-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libpango1.0-dev \
    libatk1.0-dev \
    libgdk-pixbuf-2.0-dev


source "$HOME/.cargo/env" 2>/dev/null || true


if ! command -v cargo &> /dev/null; then
    echo "⏳ Rust/Cargo is missing. Installing Rust toolchain automatically..."
    curl -sSfL https://sh.rustup.rs -o rustup-init.sh
    chmod +x rustup-init.sh
    ./rustup-init.sh -y
    rm rustup-init.sh
    source "$HOME/.cargo/env"
    rustup component add rust-analyzer
    echo "✅ Rust installed successfully."
else
    echo "✅ Rust and Cargo are already installed."
fi


export NVM_DIR="$HOME/.nvm"

if [[ ! -s "$NVM_DIR/nvm.sh" ]]; then
    sudo apt-get install -y curl
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
fi

source "$NVM_DIR/nvm.sh"

if [[ "$(nvm version 22)" == "N/A" ]]; then
    nvm install 22
else
    echo "Node.js 22 is already installed."
fi

nvm use 22
nvm alias default 22

# Install Bun if missing.
export PATH="$HOME/.bun/bin:$PATH"

if ! command -v bun &>/dev/null; then
    curl -fsSL https://bun.sh/install | bash
fi

echo "✅ Node.js: $(node -v), Bun: $(bun --version)"


export PATH="$HOME/.cargo/bin:$PATH"

if ! cargo tauri --version &>/dev/null; then
    echo "⏳ Installing Tauri CLI..."
    cargo install tauri-cli
    echo "✅ Tauri CLI installed successfully."
else
    echo "✅ Tauri CLI is already installed."
fi


if ! command -v sqlx &> /dev/null; then
    echo "⏳ Installing sqlx-cli (Postgres + SQLite)..."
    cargo install sqlx-cli --no-default-features --features "postgres,sqlite,native-tls"
    echo "✅ sqlx-cli installed successfully."
else
    echo "✅ sqlx-cli  is already installed."
fi


# PostgreSQL
cd crates/back_api

if [[ ! -f .env ]]; then
    cp .env.example .env
fi

set -a
source .env
set +a

docker compose up -d

echo "⏳ Waiting for Postgres..."

postgres_ready=false

for i in {1..10}; do
    if sqlx migrate run; then
        postgres_ready=true
        break
    fi

    sleep 3
done

if [[ "$postgres_ready" != true ]]; then
    echo "❌ PostgreSQL migrations failed."
    exit 1
fi

echo "✅ Server database created"

# SQLite
cd ../..

if [[ ! -f .env ]]; then
    cp .env.example .env
fi

set -a
source .env
set +a

if [[ "${DATABASE_URL:-}" != sqlite:* ]]; then
    echo "❌ Expected SQLite DATABASE_URL!"
    exit 1
fi

sqlx database create
sqlx migrate run --source "crates/shared_lib/migrations"

echo "✅ Local SQLite database created"

cargo test -p shared_lib --features "lite_pc,lite_mobile"

echo "✅ Rust model export TypeScript - DONE"

echo "----------------------------------------------------------------"
echo "⚠️  RECOMMENDATION:"
echo "   It is highly recommended to restart your system"
echo "   before you start development to apply all environment changes properly."
echo "----------------------------------------------------------------"