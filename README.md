# exrs
![crates.io](https://img.shields.io/crates/v/exrs)
![version](https://img.shields.io/github/v/tag/suquiya/combu)
![license](https://img.shields.io/github/license/suquiya/combu)

exrs is a command executor library by using `std::process::Command`.
exrsは`std::process::Command`を使用したコマンド実行用ライブラリです。
This is for executing command from rust code eazily than `std::process::Command` and get result.
ちょっと楽にコマンドをrustコードから実行し、結果を取得するためのものです。

# Documentation

[Here](https://docs.rs/exrs/)

# Installation to your project (プロジェクトでの使用方法)

exrs exists on crates.io.
You can use(or import) this crate like other crate that exists on crates.io.

exrs は crates.io に登録してありますので、他の crates.io 上のクレートと同じように使用（インポート）することが可能です。

## Edit cargo.toml manually (手動での cargo.toml への追加)

Add

```toml
exrs="[version you want to use]"
```

to cargo.toml.

上記コードでバージョンを指定して、cargo.toml に追加してください。

## Use cargo-edit (Recommended) (cargo-edit でプロジェクトに追加する(推奨))

If you installed cargo-edit, exec below command under the target project:

```bash
cargo add exrs
```

cargo-edit をインストールしてある場合は、上記のコマンドを実行することで使用可能です。