# Rust Backend Template

Fast, minimal Axum-based backend template with a clean, layered architecture and sensible defaults.

## Overview

- Axum + Tokio HTTP server
- Layered modules: `domain` (core), `application` (state/services), `adapters` (HTTP, repositories)
- In-memory repository example (`InMemoryUserRepo`) for quick starts and tests
- Structured logging via `tracing` with `RUST_LOG` control
- Environment-driven config (`APP_HOST`, `APP_PORT`)

## Quick Start

- Prerequisites: Rust (stable), `cargo`.
- Optional: [`just`](https://github.com/casey/just) for handy recipes.

Steps:

1) Copy environment template

```
cp .env.example .env
```

2) Run the server (debug)

```
cargo run
```

Or with `just`:

```
just run
```

By default it binds to `127.0.0.1:3000`.

## Configuration

Configure via env vars (see `.env.example`):

- `APP_HOST`: Interface to bind (default `127.0.0.1`).
- `APP_PORT`: Port to bind (default `3000`).
- `RUST_LOG`: Tracing filter (default `info,tower_http=info`).

## Project Structure

- `src/main.rs`: Binary entry; wires config, tracing, and HTTP router.
- `src/lib.rs`: Library facade exporting the crate modules.
- `src/config`: AppConfig loader and helpers.
- `src/telemetry.rs`: Tracing subscriber initialization.
- `src/error.rs`: App-wide error mapping to HTTP responses.
- `src/domain`: Core domain types, errors, and ports (traits).
- `src/application`: App state and (future) services orchestrating domain logic.
- `src/adapters/http`: HTTP DTOs and route handlers (`axum`).
- `src/adapters/repository`: Outbound adapters (e.g., in-memory repo).

## HTTP Endpoints

Routes are composed in `src/adapters/http/handlers.rs` via `router(...)` and served in `main.rs`.

Note: This template currently wires a minimal example. You can extend the router with versioned paths (e.g., `/api/v1/...`), health checks, and real resource handlers as you build out your app.

## Development Tips

- Logs: control verbosity with `RUST_LOG` (e.g., `RUST_LOG=debug`).
- Error handling: map domain errors to HTTP in `src/error.rs`.
- Replace `InMemoryUserRepo` with a persistent adapter (Postgres, etc.).
- Consider adding a `/health` route and updating the `just health` task.

## Justfile Commands (optional)

If you use `just`:

- `just run`: Start in debug with env from `.env` if present.
- `just run-release`: Start optimized binary.
- `just health`: Quick curl to `/health` (add the route in your app).

## License

Add your preferred license here.

