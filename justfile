# configuration for https://github.com/casey/just

# Set shell for non-Windows OSs:
set shell := ["bash", "-c"]
# Set shell for Windows OSs:
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

log := "warn"
export JUST_LOG := log

default: lint-format lint-typos lint-docs check clippy test-code test-docs build changelog

# Lint code style (formatting, typos, docs)
[group: 'lint']
lint-format:
  cargo fmt --all
# Run typo checks
[group: 'lint']
lint-typos:
  typos
# Check documentation for errors and warnings
[group: 'lint']
lint-docs:
  cargo rustdoc --lib -- -Zunstable-options --check -Dwarnings

# Check code for errors and warnings
[group: 'check']
check:
  cargo check

# Run Clippy for linting
[group: 'clippy']
clippy:
  cargo clippy --all-targets --tests --benches -- -D warnings

# Run tests for code
[group: 'test']
test-code:
  cargo nextest run --all-targets --all-features

# Run tests for documentation
[group: 'test']
test-docs:
  cargo test --doc

# Compile the project
[group: 'build']
build:
  cargo build

# Generate changelog
[group: 'misc']
changelog:
  git cliff -o CHANGELOG.md