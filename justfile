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
    brew install tailwindcss
    @echo "✅ Bootstrap complete ✅"

# Cleans the project
clean:
    @echo "Cleaning project..."
    cargo clean
    trunk clean

# Run the development server using vercel-cli
dev:
    @echo "Starting development server..."
    vercel dev

# Build the project using vercel-cli
build:
    @echo "Building project..."
    vercel build

# Deploys the project to production using vercel-cli
deploy:
    @echo "Deploying to production..."
    vercel build --prod
    vercel deploy --prebuilt --archive=tgz --prod
