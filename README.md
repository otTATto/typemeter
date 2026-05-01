# TypeMeter

TypeMeter is a local activity meter for your typing.

## Data & Privacy

TypeMeter records **only keystroke counts** — never the content of what you type.

- No characters or key names are stored
- All data stays on your device (local storage only)
- No network requests are made

## Tech stack

This is a desktop app built with [Tauri](https://tauri.app), a framework with a Rust-based backend and a Vue-based frontend.

| Layer           | Technology                              |
| --------------- | --------------------------------------- |
| Framework       | [Tauri 2](https://tauri.app)            |
| Frontend        | [Vue 3](https://vuejs.org) + TypeScript |
| Backend         | Rust                                    |
| Package manager | [Bun](https://bun.sh)                   |
| Build tool      | [Vite](https://vite.dev)                |

## How to start development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Bun](https://bun.sh)
- Platform-specific dependencies (see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/))

### Steps

1. Install dependencies.
   ```sh
   bun install
   ```
2. Start up a dev server.
   ```sh
   bun run tauri dev
   ```

### Using mock data

To develop with pre-generated data instead of real keystroke logs:

1. Generate a mock database (60 days of random data).
   ```sh
   python scripts/gen_mock_db.py
   ```
2. Start the dev server with the mock database.
   ```sh
   bun run tauri:mock
   ```

## Build

```sh
bun run tauri build
```

The installer is generated in `src-tauri/target/release/bundle/`.
