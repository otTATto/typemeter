/**
 * @function DateオブジェクトをYYYY-MM-DD形式の文字列に変換する
 *
 * @param date 変換対象の Date オブジェクト
 * @returns YYYY-MM-DD 形式の日付文字列
 */
export const formatDate = (date: Date): string => {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
};
