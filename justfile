# Use bash with strict flags
set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

# Auto-load env vars from a dotenv file
set dotenv-load := true

# Export common vars with sane defaults (overridden by dotenv/env)
set export := true
export APP_HOST := env_var_or_default("APP_HOST", "127.0.0.1")
export APP_PORT := env_var_or_default("APP_PORT", "3000")
export RUST_LOG := env_var_or_default("RUST_LOG", "info,tower_http=info")
export BASE_URL := "http://${APP_HOST}:${APP_PORT}"

# Default task
default: run

# Run the server (debug)
run:
    @echo "Running on ${BASE_URL} (dotenv: ${DOTENV_PATH:-.env})"
    cargo run

# Run the server (release)
run-release:
    @echo "Running (release) on ${BASE_URL} (dotenv: ${DOTENV_PATH:-.env})"
    cargo run --release

# Quick health check
health:
    curl -i "${BASE_URL}/health"

