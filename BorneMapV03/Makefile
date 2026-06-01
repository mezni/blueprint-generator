.PHONY: build-all test-all lint-all format-all

# --- Format ---
format-all:
	@echo "=== Formatting Rust ==="
	cargo fmt --all
	@echo "=== Formatting TypeScript ==="
	npx prettier --write "apps/*/{src,public}/**/*.{ts,tsx,css,json}" "packages/*/src/**/*.ts" "*.json" 2>/dev/null || true

# --- Lint ---
lint-all:
	@echo "=== Linting Rust ==="
	cargo clippy --workspace -- -D warnings
	@echo "=== Linting TypeScript ==="
	npx eslint "apps/*/{src,public}/**/*.{ts,tsx}" "packages/*/src/**/*.ts" 2>/dev/null || true

# --- Build ---
build-all:
	@echo "=== Building Rust ==="
	cargo build --workspace
	@echo "=== Building Web Apps ==="
	cd apps/driver-web && npm run build
	cd apps/partner-dashboard && npm run build
	cd apps/admin-dashboard && npm run build

# --- Test ---
test-all:
	@echo "=== Testing Rust ==="
	cargo test --workspace
