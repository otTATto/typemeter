<!-- dot-agents v0.1.0 から同期したファイルです。
     直接編集せず、https://github.com/otTATto/dot-agents を更新してください -->

# Vue に関するルールや考え方

## コンポーネント

- Vue コンポーネントのファイル名は PascalCase とする

## ライフサイクル

- コンポーネント内で登録したイベントリスナーやタイマーなどは、`onUnmounted` などのライフサイクルフックで解放する
