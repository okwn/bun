#![warn(unused_must_use)]
#![allow(unexpected_cfgs)]
// PORTING.md crate-map calls the string crate `bun_str`; the workspace package
// is `bun_string`. Alias once here so submodule `use bun_core::…` paths resolve.
#![warn(unreachable_pub)]
extern crate bun_core as bun_str;

mod json_lexer;

#[path = "json.rs"]
pub mod json;

/// Zig-side import path is `bun.json` (the parser module). Downstream Rust
/// crates name it both `json` and `json_parser`; alias the latter here.
pub use json as json_parser;

// ───── json5 ──────────────────────────────────────────────────────────────
#[path = "json5.rs"]
pub mod json5;

// ───── toml ───────────────────────────────────────────────────────────────
#[path = "toml.rs"]
pub mod toml;

// ───── yaml ───────────────────────────────────────────────────────────────
#[path = "yaml.rs"]
pub mod yaml;

// ported from: src/interchange/interchange.zig
