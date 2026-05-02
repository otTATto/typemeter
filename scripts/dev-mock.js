// mock/keystroke.db の絶対パスを TYPEMETER_DB_PATH に設定して `tauri dev` を起動する
// （相対パスのままだと tauri dev の内部 CWD が src-tauri/ になりパスがずれるため）
import { execSync } from 'child_process'
import { dirname, resolve } from 'path'
import { fileURLToPath } from 'url'

const __dirname = dirname(fileURLToPath(import.meta.url))

execSync('tauri dev', {
  stdio: 'inherit',
  env: {
    ...process.env,
    TYPEMETER_DB_PATH: resolve(__dirname, '..', 'mock', 'keystroke.db'),
  },
})
