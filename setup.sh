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
    echo "❌ Error: Docker is missing! It is strictly required for this setup."
    echo "💡 Please install Docker first to proceed."
    exit 1
else
    echo "✅ Docker detected and available."
fi


echo "📦 Installing basic Linux system dependencies (requires sudo)..."
sudo apt-get update && sudo apt-get install -y \
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



if ! command -v bun &> /dev/null; then
    echo "⏳ Bun is missing. Installing Node.js and NPM via system package manager..."
    sudo apt-get install -y nodejs npm
    CURRENT_USER=$(whoami)
    sudo npm install -g bun --user=$CURRENT_USER --unsafe-perm
    echo "✅ Bun installed successfully."
else
    echo "✅ Bun is already installed."
fi


echo "📦 Installing and syncing frontend dependencies for lite_mobile..."
cd crates/lite_mobile/ui
bun add @tauri-apps/api@latest @tauri-apps/plugin-log@latest @tauri-apps/plugin-dialog@latest @tauri-apps/plugin-fs@latest
cd ../../../

echo "📦 Installing and syncing frontend dependencies for lite_pc..."
cd crates/lite_pc/ui
bun add @tauri-apps/api@latest @tauri-apps/plugin-log@latest @tauri-apps/plugin-dialog@latest @tauri-apps/plugin-fs@latest
cd ../../../

echo "✅ Frontend dependencies installed and synced with Rust toolchain successfully."



if ! command -v cargo-tauri &> /dev/null && ! cargo --list | grep -q "tauri"; then
    echo "⏳ Tauri CLI is missing. Installing tauri-cli via Cargo (this may take a few minutes)..."
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

set -x


cp .env.example .env
export $(grep -v '^#' .env | xargs)
sqlx database create
sleep 1
sqlx migrate run --source "crates//shared_lib/migrations"
echo "local database created"
sleep 1



cd crates/back_api
cp .env.example .env
export $(grep -v '^#' .env | xargs)
docker compose up -d
sleep 3
sqlx migrate run
echo "server database created"
cd -

cargo test -p shared_lib
echo "✅ Rust data export to TypeScript: DONE"




# ==============================================================================
# 🎉 FINISH & REBOOT RECOMMENDATION
# ==============================================================================
echo "----------------------------------------------------------------"
echo "🎉 [SUCCESS] All databases and tools initialized successfully!"
echo "----------------------------------------------------------------"
echo "⚠️  RECOMMENDATION:"
echo "   It is highly recommended to restart your system (or WSL session)"
echo "   before you start development to apply all environment changes properly."
echo "----------------------------------------------------------------"