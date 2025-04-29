//! #exrs
//! exrs is a command executor library by using `std::process::Command`.
//! exrsは`std::process::Command`を使用したコマンド実行用ライブラリです。
//!This is for executing command from rust code eazily than `std::process::Command` and get result.
//!ちょっと楽にコマンドをrustコードから実行し、結果を取得するためのものです。
//!
//! #features
//! - `cmd` - execute command from rust code eazily than `std::process::Command` and get result.
//! - `shell` - execute shell command from rust code eazily than `std::process::Command` and get result.
//!
//! for more information: see README.md

#[cfg(feature = "cmd")]
pub mod cmd;
#[cfg(feature = "shell")]
pub mod shell;
