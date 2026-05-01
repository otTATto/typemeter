# コメントに関するルールや考え方

## 全般

- 自明な処理にコメントしない
- 既存のコメントを破壊しない
- 対話後の修正時に修正箇所に関する説明をコメントで強調しない（コードの最終的な読み手はその経緯を知らない）
- コメントの量はその影響範囲、つまりどれだけ多くの場所から呼び出されるかに依存する
  - 他の場所から呼び出されうる定数・関数は丁寧に説明する
  - TypeScript の関数は JSDoc スタイルを、Rust の関数は Rustdoc スタイルを使う

## コメントの例

### TypeScript

#### 型定義

```typescript
// src/types/user.ts

/**
 * 配送先住所
 *
 * - id           : 住所の一意識別子
 * - postalCode   : 郵便番号
 * - fullAddress  : 住所
 * - recipientName: 受取人名
 * - isDefault    : デフォルト住所かどうか
 */
export type Address = {
  id: string;
  postalCode: string;
  fullAddress: string;
  recipientName: string;
  isDefault: boolean;
};
```

#### 関数定義

```typescript
/**
 * @function 商品 ID と数量を受け取り、商品名や単価、小計を付け加えた形に整形して返す
 *
 * @param items 商品 ID をキー、購入数量を値とする Record
 * @returns 商品 ID をキーとし、商品名・単価・数量・小計を持つ Record
 *
 * NOTE:
 *   - 商品情報の取得に失敗した場合は null を返す
 *   - 入力された商品 ID のうち PRODUCT_IDS に合致するもののみ返す
 *   - つまり、返り値のキーに引数のキーすべてが存在するとは限らない
 *
 * e.g.
 *   - @param items { 'book': 2, 'pen': 3, 'unknownItem': 1 }
 *     ↓
 *     @returns {
 *       'book': { name: 'ノート', unitPrice: 300, quantity: 2, subtotal: 600 },
 *       'pen': { name: 'ペン', unitPrice: 120, quantity: 3, subtotal: 360 }
 *     }
 */
export const formatOrderItems = async (
  items: Partial<Record<ProductId, number>>,
): Promise<Partial<OrderItems> | null> => {
  ...
};
```

### Rust

- `///` で記述する（`/** */` は使わない）
- `@param` / `@returns` は使わない（JSDoc 記法は Rustdoc に認識されない）
- `# Section` 見出しを使う。`cargo doc` でレンダリングされる
- よく使うセクション:
  - `# Parameters` — 引数の説明
  - `# Returns` — 返り値の説明
  - `# Errors` — `Result::Err` になる条件
  - `# Panics` — パニックする条件
  - `# Behavior` — 動作の詳細・注意事項
  - `# Examples` — 使用例（`cargo test` で実行される）

#### 関数定義

```rust
/// `minute_count` の現在値を SQLite に保存し、保存成功時に減算・`today_db_count` を加算する
///
/// # Parameters
/// * `minute_count` - 直近の未保存キーストローク数
/// * `today_db_count` - 今日の DB 保存済み合計（書き込み成功時にインクリメントされる）
/// * `db_path` - データベースファイルのパス
///
/// # Behavior
/// * `minute_count` が 0 の場合は書き込みをスキップする
/// * DB 書き込み成功後に保存した分だけ減算する（失敗時はカウントを保持）
fn flush_minute_count(
    minute_count: &Arc<Mutex<u64>>,
    today_db_count: &Arc<Mutex<u64>>,
    db_path: &str,
) {
    ...
}
```
