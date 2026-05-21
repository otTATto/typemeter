const applyTheme = () => {
  const saved = localStorage.getItem('theme');
  const isDark =
    saved === 'dark' || saved === 'light'
      ? saved === 'dark'
      : window.matchMedia('(prefers-color-scheme: dark)').matches;
  document.documentElement.classList.toggle('dark', isDark);
};

/**
 * @function テーマを初期適用し、ストレージ変更とシステムテーマ変更を監視する
 *
 * @returns クリーンアップ関数（onUnmounted で呼び出す）
 */
export const initTheme = (): (() => void) => {
  applyTheme();

  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  window.addEventListener('storage', applyTheme);
  mediaQuery.addEventListener('change', applyTheme);

  return () => {
    window.removeEventListener('storage', applyTheme);
    mediaQuery.removeEventListener('change', applyTheme);
  };
};
