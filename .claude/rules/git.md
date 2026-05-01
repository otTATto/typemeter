# Git に関するルール

## ブランチについて

- 大まかには master, feature/, fix/ の 3 種類のブランチから構成される
- 最新の master ブランチから作業ブランチ `feature/**` または `fix/**` を切って作業を開始する
- 例: `feature/user-account-creation-page` 

## コミットメッセージについて

- 1行目に Commit Subject, 2行目に任意で Description を書く
- Commit Subject は Conventional Commits に準拠した形式を意識する
  - 本文は日本語
  - スコープは任意
- 差分ファイル名をそのままコミットメッセージに書くだけで満足せず、実装の背景にあるアルゴリズムやメンタルモデルを汲み取れるように上手く要約する
  - 悪い例: `fix: アカウントの新規登録ページを修正`
  - 良い例: `fix(register): 利用規約への同意を意味するチェックボックスをより目立たせた`
- 1行のコミットメッセージにまとめられない場合は、任意で Description を箇条書きで書く

## Pull Request について

- PR はテンプレート `.github/PULL_REQUEST_TEMPLATE.md` に沿って記述する
- PR には `.github/labels.yml` で定義されるラベルを付与することができる
- PR タイトルは `feat(register): アカウント登録画面の実装` のように、Conventional Commits に準拠した形式で書く
- レビュワーのレビュー負担軽減のため、1PR あたりのコード差分は原則 500 行以内とする
- 特に差分が 1,000 行を超える場合は、必ず複数の PR に分割する
- レビュワーにより Approve されるまで master には merge しない
