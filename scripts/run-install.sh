#!/usr/bin/env bash

# 第 1 引数と第 2 引数のコミット間で
# bun.lock / package.json に差分があれば `bun install` を、
# src-tauri/Cargo.toml に差分があれば `cargo fetch` を実行する

set -e

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || exit 0
cd "$REPO_ROOT"

OLD_REV="${1:-}"
NEW_REV="${2:-}"

if [[ -z "${OLD_REV}" || -z "${NEW_REV}" ]]; then
  exit 0
fi

CHANGED="$(git diff --name-only "${OLD_REV}" "${NEW_REV}" || true)"

if echo "${CHANGED}" | grep -E -q '^(package\.json|bun\.lock)$'; then
  echo "👀 JS dependency differences detected. Running \`bun install\`..."
  bun install
fi

if echo "${CHANGED}" | grep -E -q '^src-tauri/Cargo\.toml$'; then
  echo "👀 Rust dependency differences detected. Running \`cargo fetch\`..."
  cargo fetch --manifest-path src-tauri/Cargo.toml
fi
