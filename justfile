# List all available commands
default:
  @just --list -u

# Set up the local development environment
bootstrap:
    @echo "Bootstrapping project..."
    rustup show
    rustup target add wasm32-unknown-unknown
    rustup component add clippy
    cargo install cargo-binstall --locked
    cargo binstall cargo-nextest -y
    cargo binstall trunk -y
    cargo binstall wasm-bindgen-cli -y
    just clean
    cargo build
    pnpm install -g vercel
    @if ! command -v tailwindcss 2>&1 >/dev/null; then \
        echo "⚠️ tailwindcss not found, install from https://tailwindcss.com/blog/standalone-cli ⚠️"; \
    fi
    @echo "✅ Bootstrap complete ✅"

# Cleans the project
clean:
    @echo "Cleaning project..."
    trunk clean --config app/Trunk.toml
    cargo clean

# Run the development server using vercel-cli
dev:
    @echo "Starting development server..."
    vercel env pull .env
    vercel dev

# Runs a local instance of the database
db:
    @echo "Starting database..."
    @if ! command -v mongod 2>&1 >/dev/null; then \
        echo "⚠️ mongod not found, install from https://www.mongodb.com/docs/manual/installation ⚠️"; \
    else \
        mkdir -p .database; \
        mongod --dbpath .database; \
    fi

# Build the project using vercel-cli
build:
    @echo "Building project..."
    vercel env pull .env
    trunk build --config app/Trunk.toml
    vercel build
