import eslintConfigPrettier from 'eslint-config-prettier';
import importPlugin from 'eslint-plugin-import';
import unusedImports from 'eslint-plugin-unused-imports';
import pluginVue from 'eslint-plugin-vue';
import tseslint from 'typescript-eslint';

const eslintConfig = tseslint.config(
  // Vue ルール
  ...pluginVue.configs['flat/recommended'],

  // TypeScript ルール
  {
    files: ['**/*.ts'],
    extends: tseslint.configs.recommended,
  },

  // Vue SFC 内の TypeScript を解析するためのサブパーサー設定
  {
    files: ['**/*.vue'],
    languageOptions: {
      parserOptions: {
        parser: tseslint.parser,
      },
    },
  },

  {
    /*
      プラグイン
      - import         : import の順番を rules のとおりに整理する
      - unused-imports : 使われていない import を削除する
    */
    plugins: {
      'import': importPlugin,
      'unused-imports': unusedImports,
    },

    rules: {
      // import の並び順
      'import/order': [
        'error',
        {
          /*
            import の並び順とグルーピングのルール
            - builtin  : ex) from 'path'
            - external : ex) from 'vue'
            - internal : ex) from '@/components/Button.vue'
            - parent   : ex) from '../services'
            - sibling  : ex) from './validateForm'
            - index    : ex) from './'
            - object
            - type     : ex) from '@/types/users'
          */
          'groups': [
            'builtin',
            'external',
            'internal',
            'parent',
            'sibling',
            'index',
            'object',
            'type',
          ],
          // グループ間の改行なし
          'newlines-between': 'never',
          // グループ内は ABC 順
          'alphabetize': { order: 'asc' },
        },
      ],
      // 使われていない import を削除
      'unused-imports/no-unused-imports': 'error',
      // 同じライブラリから import されている module を 1 行に
      'import/no-duplicates': 'error',
    },
  },

  {
    ignores: ['node_modules/**', 'dist/**', 'src-tauri/**'],
  },

  // Prettier と競合する ESLint ルールを無効化
  eslintConfigPrettier,
);

export default eslintConfig;
