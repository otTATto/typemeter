/**
 * @see https://prettier.io/docs/configuration
 * @type {import("prettier").Config}
 */
const config = {
  // インデントのサイズはスペース 2 つ
  tabWidth: 2,
  // 半角スペースによるインデント
  useTabs: false,
  // 末尾にセミコロン
  semi: true,
  // 文字列の囲みはシングルクォーテーション
  singleQuote: true,
  // オブジェクトのプロパティ内に引用符の必要なプロパティがある場合は
  // すべてのプロパティに引用符を付ける
  quoteProps: 'consistent',
  // 折り返す文字数
  printWidth: 100,
};

export default config;
