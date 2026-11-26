#!/bin/bash

set -e

echo "🚀 Welcome to the Setup!"
echo "----------------------------------------------------------------"
echo "⚠️  IMPORTANT: Docker must be installed manually before running this script!"
echo "----------------------------------------------------------------"
echo "This script will automatically configure your environment and install:"
echo "  📦 Basic Linux GUI dependencies (GTK, WebKit, Build Essentials)"
echo "  🔐 Secure Storage (GNOME Keyring)"
echo "  🦀 Rust Toolchain & Rust Analyzer"
echo "  ⚡ Node.js, Bun & Frontend dependencies"
echo "  🚀 Tauri CLI & SQLx CLI"
echo "  🐳 Infrastructure containers, local databases, and migrations"
echo "----------------------------------------------------------------"

read -r -p "🤔 Do you have Docker installed and want to proceed? (y/n): " main_choice

if [[ ! "$main_choice" =~ ^[Yy]$ ]]; then
    echo "❌ Setup aborted by user."
    exit 0
fi

echo "🚀 Setup starting..."


# --------------------------------------------------
# DOCKER
# --------------------------------------------------

if ! command -v docker &>/dev/null; then
    echo "❌ Docker CLI is missing!"
    echo "💡 Install Docker and make sure its daemon is accessible."
    exit 1
fi

echo "🐳 Checking Docker daemon..."

docker_ready=false

for i in {1..15}; do
    if docker info &>/dev/null; then
        docker_ready=true
        break
    fi

    echo "⏳ Waiting for Docker... ($i/15)"
    sleep 2
done

if [[ "$docker_ready" != true ]]; then
    echo "❌ Docker daemon is not running or is not accessible!"
    echo "💡 Start Docker Engine/Desktop and check your permissions."
    exit 1
fi

if ! docker compose version &>/dev/null; then
    echo "❌ Docker Compose plugin is missing!"
    exit 1
fi

echo "✅ Docker is running and available."


# --------------------------------------------------
# SYSTEM DEPENDENCIES
# --------------------------------------------------

echo "📦 Installing system dependencies..."

sudo apt-get update

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
    libgdk-pixbuf-2.0-dev \
    gnome-keyring \
    libsecret-tools \
    dbus-user-session \
    libdbus-1-dev \
    unzip


# --------------------------------------------------
# SECURE STORAGE
# --------------------------------------------------

echo "--------------------------------------------------"
echo "🔐 INITIALIZING SECURE STORAGE..."
echo "--------------------------------------------------"
echo "⚠️  NOTE FOR PASSWORD PROMPTS:"
echo "1. If Keyring is NOT configured yet: You will be asked"
echo "   to create a password TWICE to initialize the storage."
echo "2. If Keyring IS already configured: You may be asked"
echo "   to enter your password to unlock the test entry."
echo "--------------------------------------------------"

printf '%s' 'test-secret-123' | secret-tool store \
    --label="Keyring test" \
    app test-app key test-token

stored_secret=$(secret-tool lookup app test-app key test-token)

if [[ "$stored_secret" != "test-secret-123" ]]; then
    echo "❌ Secure storage verification failed!"
    exit 1
fi

secret-tool clear app test-app key test-token

echo "--------------------------------------------------"
echo "✅ Secure storage verification passed!"
echo "--------------------------------------------------"


# --------------------------------------------------
# RUST TOOLCHAIN
# --------------------------------------------------

echo "🦀 Checking Rust toolchain..."

export PATH="$HOME/.cargo/bin:$PATH"

if ! command -v cargo &>/dev/null; then
    echo "⏳ Installing Rust toolchain..."

    curl -sSfL https://sh.rustup.rs -o rustup-init.sh
    chmod +x rustup-init.sh
    ./rustup-init.sh -y
    rm rustup-init.sh

    source "$HOME/.cargo/env"
else
    echo "✅ Rust and Cargo are already installed."
fi

if ! rustup component list --installed | grep -q '^rust-analyzer'; then
    echo "⏳ Installing Rust Analyzer..."
    rustup component add rust-analyzer
fi

echo "✅ Rust: $(rustc --version)"


# --------------------------------------------------
# NODE.JS & BUN
# --------------------------------------------------

echo "⚡ Checking Node.js and Bun..."

export NVM_DIR="$HOME/.nvm"

if [[ ! -s "$NVM_DIR/nvm.sh" ]]; then
    echo "⏳ Installing NVM..."
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
fi

source "$NVM_DIR/nvm.sh"

if [[ "$(nvm version 22)" == "N/A" ]]; then
    echo "⏳ Installing Node.js 22..."
    nvm install 22
else
    echo "✅ Node.js 22 is already installed."
fi

nvm use 22
nvm alias default 22

export PATH="$HOME/.bun/bin:$PATH"

if ! command -v bun &>/dev/null; then
    echo "⏳ Installing Bun..."
    curl -fsSL https://bun.sh/install | bash
    export PATH="$HOME/.bun/bin:$PATH"
fi

if ! command -v bun &>/dev/null; then
    echo "❌ Bun installation failed!"
    exit 1
fi

echo "✅ Node.js: $(node --version)"
echo "✅ Bun: $(bun --version)"


# --------------------------------------------------
# TAURI & SQLX CLI
# --------------------------------------------------

echo "🚀 Checking development tools..."

if ! cargo tauri --version &>/dev/null; then
    echo "⏳ Installing Tauri CLI..."
    cargo install tauri-cli
else
    echo "✅ Tauri CLI is already installed."
fi

if ! command -v sqlx &>/dev/null; then
    echo "⏳ Installing SQLx CLI..."
    cargo install sqlx-cli --no-default-features --features "postgres,sqlite,native-tls"
else
    echo "✅ SQLx CLI is already installed."
fi



# Frontend dependencies:
echo "📦 Installing frontend dependencies..."

cd crates/lite_pc/ui
bun install
cd ../../lite_mobile/ui
cd ../../..

echo "✅ Frontend dependencies installed."


# --------------------------------------------------
# POSTGRESQL
# --------------------------------------------------

echo "🐳 Setting up PostgreSQL..."

cd crates/back_api

if [[ ! -f .env ]]; then
    cp .env.example .env
fi

set -a
source .env
set +a

docker compose up -d

echo "⏳ Waiting for PostgreSQL..."

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

echo "✅ Server database created."


# --------------------------------------------------
# SQLITE
# --------------------------------------------------

echo "🗄️ Setting up SQLite..."

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

echo "✅ Local SQLite database created."


# --------------------------------------------------
# PROJECT TESTS
# --------------------------------------------------

echo "🧪 Running shared_lib tests..."

cargo test -p shared_lib --features "lite_pc,lite_mobile"

echo "✅ Rust model export TypeScript - DONE"


# --------------------------------------------------
# FINISHED
# --------------------------------------------------

echo "----------------------------------------------------------------"
echo "🎉 SETUP COMPLETED SUCCESSFULLY!"
echo "----------------------------------------------------------------"
echo "⚠️  RECOMMENDATION:"
echo "   Restart your system before starting development"
echo "   to apply all environment changes properly."
echo "----------------------------------------------------------------"
