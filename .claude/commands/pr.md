現在チェックアウトしているブランチに紐づく GitHub PR のレビューコメントを読み、未対応のものを洗い出して実装計画を立ててください。

## 手順

1. `gh pr view --json number,title,headRefName` でカレントブランチの PR を特定する
2. 以下のコマンドで PR のレビューコメント（インラインコメントと全体コメントの両方）を取得する
   - インライン: `gh api /repos/{owner}/{repo}/pulls/{pr_number}/comments`
   - 全体: `gh api /repos/{owner}/{repo}/issues/{pr_number}/comments`
   - レビュー本体: `gh api /repos/{owner}/{repo}/pulls/{pr_number}/reviews`
3. owner と repo は `gh repo view --json owner,name` で取得する
4. 取得したコメントを分析し、**未対応のもの**を洗い出す
   - resolve 済みのインラインコメントは除外する（`gh api /repos/{owner}/{repo}/pulls/{pr_number}/comments` の `position` が `null` のものは resolve 済みの可能性が高い）
   - ボットや自動生成コメント（Gemini Code Assist 等）も含めて確認する
   - prefix（[MUST] / [WANT] / [IMO] / [Q] / [NITS] / [FYI]）があれば優先度の参考にする
     - [MUST] は必須対応
     - [WANT] / [IMO] は対応要否をユーザーに確認する
     - [Q] は回答が必要かユーザーに確認する
     - [FYI] / [NITS] は原則スキップ（対応する場合はユーザーに確認）

## 出力形式

未対応コメントの一覧を以下の形式でまとめる：

```
### 未対応のレビューコメント

#### [MUST] 必須対応
- `path/to/file.ts:行番号` — コメント内容の要約

#### [WANT] / [IMO] 要確認
- `path/to/file.ts:行番号` — コメント内容の要約（対応しますか？）

#### [Q] 質問
- コメント内容の要約（回答しますか？）
```

## 実装計画

未対応コメントの洗い出しが終わったら、**[MUST] の対応方針をまとめた実装計画を提示**し、ユーザーの承認を得てから実装を開始してください。[WANT] / [IMO] についてはその際に対応するかどうかを確認してください。

## 注意事項

- 承認なしに実装を開始しない
- コミットは行わない（ユーザーが確認後に指示する）
- コメントの意図が読み取れない場合は推測せず、ユーザーに確認する
