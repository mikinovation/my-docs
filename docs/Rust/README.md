# Rust

## バージョンアップ

```bash
cargo --version # cargoのバージョン
rustc -V # rustcのバージョン
rustup --V # rustupのバージョン
rustup show # インストール済みのツールチェインの一覧
rustup update # ツールチェインの更新
```

## コーディングルール

- mod.rsは使わない
  - mod.rsはedition2018以降で過去の互換のためだけに残っている方法
- 階層的なモジュール宣言を同一ファイル内ではしない
  - 原則として1ファイル1モジュール