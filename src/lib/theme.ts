import { cacheThemeForInitialPaint, getTheme, onThemeChange, type Theme } from '@/lib/settings';

/**
 * @function テーマ設定からダークモードかどうかを解決する
 *
 * @param theme 保存されているテーマ。null（未設定）の場合は OS 設定に追従する
 * @returns ダークモードなら true
 */
export const resolveIsDark = (theme: Theme | null): boolean =>
  theme === 'dark' || (theme === null && window.matchMedia('(prefers-color-scheme: dark)').matches);

const applyTheme = (theme: Theme | null) => {
  document.documentElement.classList.toggle('dark', resolveIsDark(theme));
};

/**
 * @function テーマを初期適用し、設定ストアの変更とシステムテーマ変更を監視する
 *
 * @returns クリーンアップ関数（onUnmounted で呼び出す）
 *
 * NOTE:
 *   - 明示的なテーマ設定（theme キー）が無い間のみ prefers-color-scheme の変更を反映する
 *   - ウィンドウ間の同期は設定ストアの onKeyChange 購読が担う
 */
export const initTheme = (): (() => void) => {
  let explicitTheme: Theme | null = null;
  let isDisposed = false;
  let unlistenThemeChange: (() => void) | null = null;

  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  const applyFromMediaQuery = () => {
    if (explicitTheme === null) applyTheme(null);
  };
  mediaQuery.addEventListener('change', applyFromMediaQuery);

  (async () => {
    explicitTheme = await getTheme();
    applyTheme(explicitTheme);
    // キャッシュの欠損や手動編集からの自己修復
    cacheThemeForInitialPaint(explicitTheme);

    const unlisten = await onThemeChange((theme) => {
      explicitTheme = theme;
      applyTheme(theme);
      cacheThemeForInitialPaint(theme);
    });

    // 購読完了前に cleanup が呼ばれていた場合は即座に unlisten する
    if (isDisposed) {
      unlisten();
    } else {
      unlistenThemeChange = unlisten;
    }
  })();

  return () => {
    isDisposed = true;
    unlistenThemeChange?.();
    mediaQuery.removeEventListener('change', applyFromMediaQuery);
  };
};
