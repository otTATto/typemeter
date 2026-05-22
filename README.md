# typemeter

typemeter is a local activity meter for your typing, like a pedometer for your work at the keyboard.

<img width="2059" height="986" alt="typemeter dashboard screenshot" src="https://github.com/user-attachments/assets/30bc40a4-bde0-48d5-ba2f-725182760510" />

## Data & Privacy

typemeter records **only keystroke counts** — never the content of what you type.

- No characters or key names are stored
- All data stays on your device (local storage only)
- No network requests are made

## Tech Stack

This is a desktop app built with [Tauri](https://tauri.app), a framework with a Rust-based backend and a Vue-based frontend.

| Layer           | Technology                              |
| --------------- | --------------------------------------- |
| Framework       | [Tauri 2](https://tauri.app)            |
| Frontend        | [Vue 3](https://vuejs.org) + TypeScript |
| Backend         | Rust                                    |
| Package manager | [Bun](https://bun.sh)                   |
| Build tool      | [Vite](https://vite.dev)                |

## How to Develop

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

### Using Mock Data

To develop with pre-generated data instead of real keystroke logs:

1. Generate a mock database (60 days of random data).
   ```sh
   python scripts/gen_mock_db.py
   ```
2. Start the dev server with the mock database.
   ```sh
   bun run tauri:mock
   ```

## How to Build

```sh
bun run tauri build
```

The installer is generated in `src-tauri/target/release/bundle/`.

## How to Release

### 1. Cut a release branch

```sh
git checkout main && git pull
git checkout -b release/v0.2.0
```

### 2. Bump the version

Update the following two fields in `src-tauri/tauri.conf.json`:

| Field                        | Platform      | Example                                  |
| ---------------------------- | ------------- | ---------------------------------------- |
| `version`                    | macOS / Linux | `0.2.0`                                  |
| `bundle.windows.wix.version` | Windows       | `0.2.0.0` (must follow `x.y.z.w` format) |

### 3. Create the release note

```sh
bun run release:note v0.2.0
```

This generates `docs/releases/v0.2.0.md` from the template in `.github/RELEASE_TEMPLATE.md`.

### 4. Edit the release note

Fill in the **New Features** and **Bug Fixes** sections in `docs/releases/v0.2.0.md`.

### 5. Commit and merge into main

```sh
git add src-tauri/tauri.conf.json docs/releases/v0.2.0.md
git commit -m "release: v0.2.0"
```

Open a PR from `release/v0.2.0` into `main` and merge it.

### 6. Push a tag

After merging, push a version tag from `main`. GitHub Actions will then build the app for all platforms and create a **draft** GitHub Release using the release note as the body.

```sh
git checkout main && git pull
git tag v0.2.0
git push origin v0.2.0
```

### 7. Publish the release

Once all builds succeed, open the draft release on GitHub, verify the attached assets, and publish it.
