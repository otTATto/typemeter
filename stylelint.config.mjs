/** @type {import('stylelint').Config} */
export default {
  extends: ['stylelint-config-standard'],
  rules: {
    'at-rule-no-unknown': [true, { ignoreAtRules: ['theme', 'source', 'utility', 'variant'] }],
    'import-notation': 'string',
  },
  overrides: [
    {
      files: ['*.vue'],
      customSyntax: 'postcss-html',
    },
  ],
};
