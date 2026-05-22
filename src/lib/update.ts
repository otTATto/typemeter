import { getVersion } from '@tauri-apps/api/app';

const RELEASES_API_URL = 'https://api.github.com/repos/otTATto/typemeter/releases?per_page=1';
const RELEASES_FALLBACK_URL = 'https://github.com/otTATto/typemeter/releases';

export type UpdateInfo = {
  version: string;
  currentVersion: string;
  releaseUrl: string;
};

type ParsedVersion = {
  major: number;
  minor: number;
  patch: number;
  pre: string | null;
};

const parseVersion = (v: string): ParsedVersion => {
  const clean = v.replace(/^v/, '');
  const [main, pre = null] = clean.split(/-(.+)/, 2) as [string, string | undefined];
  const [major = 0, minor = 0, patch = 0] = main.split('.').map(Number);
  return { major, minor, patch, pre: pre ?? null };
};

/**
 * @function latest が current より新しいバージョンかどうかを判定する
 *
 * NOTE:
 *   - プレリリースのセグメントが数値の場合は数値比較、それ以外は辞書順
 *   - リリース版 (pre=null) はプレリリース (pre!=null) より新しいとみなす
 */
const isNewerThan = (latest: ParsedVersion, current: ParsedVersion): boolean => {
  if (latest.major !== current.major) return latest.major > current.major;
  if (latest.minor !== current.minor) return latest.minor > current.minor;
  if (latest.patch !== current.patch) return latest.patch > current.patch;
  if (latest.pre === null && current.pre !== null) return true;
  if (latest.pre !== null && current.pre === null) return false;
  if (latest.pre === null && current.pre === null) return false;

  const lParts = (latest.pre ?? '').split('.');
  const cParts = (current.pre ?? '').split('.');
  for (let i = 0; i < Math.max(lParts.length, cParts.length); i++) {
    const l = lParts[i];
    const c = cParts[i];
    if (l === undefined) return false;
    if (c === undefined) return true;
    if (l === c) continue;
    const lNum = Number(l);
    const cNum = Number(c);
    if (!isNaN(lNum) && !isNaN(cNum)) return lNum > cNum;
    return l > c;
  }
  return false;
};

/**
 * @function GitHub Releases API で最新リリースを取得し、現バージョンより新しければ返す
 *
 * NOTE: ネットワークエラーや API エラー時は null を返す（アプリの動作を妨げない）
 */
export const checkForUpdate = async (): Promise<UpdateInfo | null> => {
  try {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), 5000);
    const [currentVersion, response] = await Promise.all([
      getVersion(),
      fetch(RELEASES_API_URL, { signal: controller.signal }),
    ]);
    clearTimeout(timeoutId);

    if (!response.ok) return null;

    const data: unknown[] = await response.json();
    const latest = data[0] as Record<string, string> | undefined;
    const latestTag = latest?.tag_name ?? '';

    if (!latestTag) return null;

    if (!isNewerThan(parseVersion(latestTag), parseVersion(currentVersion))) return null;

    return {
      version: latestTag.replace(/^v/, ''),
      currentVersion,
      releaseUrl: latest?.html_url ?? RELEASES_FALLBACK_URL,
    };
  } catch {
    return null;
  }
};
