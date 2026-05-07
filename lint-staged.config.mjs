export default {
  '*.vue': ['eslint --fix', 'stylelint --fix', 'oxfmt --write'],
  '*.ts': ['eslint --fix', 'oxfmt --write'],
  '*.css': ['stylelint --fix', 'oxfmt --write'],
  '*.{json,md}': (files) => {
    const applicable = files.filter((f) => !/[/\\]src-tauri[/\\]/.test(f));
    if (!applicable.length) return [];
    return [
      `oxfmt --write --ignore-path=.oxfmtignore ${applicable.map((f) => JSON.stringify(f)).join(' ')}`,
    ];
  },
};
