#!/usr/bin/env bun
/**
 * リリースノートのひな形を生成するスクリプト。
 *
 * `.github/RELEASE_TEMPLATE.md` をもとに `docs/releases/v{version}.md` を作成する。
 * `{version}` プレースホルダーは指定されたバージョン番号に置換される。
 *
 * 生成したファイルを編集・コミットしておくことで、release ブランチへのプッシュ時に
 * GitHub Actions がそのファイルを GitHub Release の本文として自動的に使用する。
 *
 * Usage:
 *   bun run release:note <version>
 *
 * Example:
 *   bun run release:note v0.2.0
 *   → docs/releases/v0.2.0.md を生成
 */
import { readFileSync, writeFileSync, mkdirSync, existsSync } from 'fs'
import { join, dirname } from 'path'
import { fileURLToPath } from 'url'

const version = process.argv[2]
if (!version || version.includes('/') || version.includes('\\')) {
  console.error('Usage: bun scripts/create-release-note.js <version>')
  console.error('Example: bun scripts/create-release-note.js v0.1.0')
  process.exit(1)
}

const normalizedVersion = version.startsWith('v') ? version.slice(1) : version
const tagVersion = `v${normalizedVersion}`

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const outputPath = join(root, 'docs', 'releases', `${tagVersion}.md`)

if (existsSync(outputPath)) {
  console.error(`Already exists: ${outputPath}`)
  process.exit(1)
}

const templatePath = join(root, '.github', 'RELEASE_TEMPLATE.md')
if (!existsSync(templatePath)) {
  console.error(`Template file not found: ${templatePath}`)
  process.exit(1)
}
const template = readFileSync(templatePath, 'utf-8')
const content = template.replaceAll('{version}', normalizedVersion)

mkdirSync(join(root, 'docs', 'releases'), { recursive: true })
writeFileSync(outputPath, content)
console.log(`Created: docs/releases/${tagVersion}.md`)
