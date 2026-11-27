
#!/bin/bash

set -euo pipefail

# --------------------------------------------------
# PROJECT ROOT
# --------------------------------------------------

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

PROJECT_ROOT="$SCRIPT_DIR"

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

echo "🐳 Checking Docker..."

if ! command -v docker &>/dev/null; then
    echo "❌ Docker CLI is missing!"
    echo "💡 Install Docker and make sure its daemon is accessible."
    exit 1
fi

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

    curl -4 -fsSL --retry 3 --retry-all-errors \
        https://sh.rustup.rs \
        -o rustup-init.sh

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

    curl -4 -fsSL --retry 3 --retry-all-errors \
        https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh \
        | bash
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

    curl -4 -fsSL --retry 3 --retry-all-errors \
        https://bun.sh/install | bash

    export PATH="$HOME/.bun/bin:$PATH"
fi

if ! command -v bun &>/dev/null; then
    echo "❌ Bun installation failed!"
    exit 1
fi

echo "✅ Node.js: $(node --version)"
echo "✅ Bun: $(bun --version)"


# --------------------------------------------------
# PERSIST ENVIRONMENT PATHS
# --------------------------------------------------

echo "⚙️ Configuring environment..."

BASHRC="$HOME/.bashrc"
touch "$BASHRC"

if ! grep -qF 'export PATH="$HOME/.cargo/bin:$PATH"' "$BASHRC"; then
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$BASHRC"
fi

if ! grep -qF 'export PATH="$HOME/.bun/bin:$PATH"' "$BASHRC"; then
    echo 'export PATH="$HOME/.bun/bin:$PATH"' >> "$BASHRC"
fi

# Current setup script environment
export PATH="$HOME/.cargo/bin:$HOME/.bun/bin:$PATH"

echo "✅ Environment paths configured."


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

    cargo install sqlx-cli \
        --no-default-features \
        --features "postgres,sqlite,native-tls"
else
    echo "✅ SQLx CLI is already installed."
fi


# --------------------------------------------------
# FRONTEND DEPENDENCIES
# --------------------------------------------------

echo "📦 Installing frontend dependencies..."

PC_UI="$PROJECT_ROOT/crates/lite_pc/ui"
MOBILE_UI="$PROJECT_ROOT/crates/lite_mobile/ui"

if [[ ! -f "$PC_UI/package.json" ]]; then
    echo "❌ package.json not found: $PC_UI"
    exit 1
fi

if [[ ! -f "$MOBILE_UI/package.json" ]]; then
    echo "❌ package.json not found: $MOBILE_UI"
    exit 1
fi

echo "📦 Installing PC frontend dependencies..."
cd "$PC_UI"
bun install

echo "📦 Installing Mobile frontend dependencies..."
cd "$MOBILE_UI"
bun install

cd "$PROJECT_ROOT"

echo "✅ Frontend dependencies installed."


# --------------------------------------------------
# POSTGRESQL
# --------------------------------------------------

echo "🐳 Setting up PostgreSQL..."

cd "$PROJECT_ROOT/crates/back_api"

if [[ ! -f .env ]]; then
    if [[ ! -f .env.example ]]; then
        echo "❌ crates/back_api/.env.example not found!"
        exit 1
    fi

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

cd "$PROJECT_ROOT"

if [[ ! -f .env ]]; then
    if [[ ! -f .env.example ]]; then
        echo "❌ Project .env.example not found!"
        exit 1
    fi

    cp .env.example .env
fi

set -a
source .env
set +a

if [[ "${DATABASE_URL:-}" != sqlite:* ]]; then
    echo "❌ Expected SQLite DATABASE_URL!"
    exit 1
fi

# Determine the SQLite database file path
sqlite_db_path="${DATABASE_URL#sqlite:}"
sqlite_db_path="${sqlite_db_path%%\?*}"
sqlite_db_path="${sqlite_db_path#//}"

if [[ "$sqlite_db_path" == ":memory:" || -z "$sqlite_db_path" ]]; then
    echo "❌ A persistent SQLite database path is required!"
    exit 1
fi

if [[ "$sqlite_db_path" != /* ]]; then
    sqlite_db_path="$PROJECT_ROOT/$sqlite_db_path"
fi

if [[ -f "$sqlite_db_path" ]]; then
    echo "✅ SQLite database already exists."
else
    echo "⏳ Creating SQLite database..."
    sqlx database create
fi

echo "⏳ Running SQLite migrations..."

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
echo "   Restart WSL or open a new terminal"
echo "   to apply all environment changes properly."
echo "----------------------------------------------------------------"
