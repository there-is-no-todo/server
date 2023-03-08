# Server for TiNT

[![Rust](https://github.com/there-is-no-todo/server/actions/workflows/rust.yml/badge.svg)](https://github.com/there-is-no-todo/server/actions/workflows/rust.yml)

The server is responsible for storing plans.

```rust
struct Plan {
    #[serde(skip_deserializing)]
    id: Option<i32>,
    title: String,
    from_hr: Option<i32>,
    from_min: Option<i32>,
    to_hr: Option<i32>,
    to_min: Option<i32>,
}
```

It provides API for basic operation on the plans.

```
   >> (list) GET /
   >> (create) POST /
   >> (destroy) DELETE /
   >> (read) GET /<id>
   >> (delete) DELETE /<id>
```

## Compilation

This server makes use of SQLite. You'll need `sqlite3` and its development
headers installed:

- **macOS:** `brew install sqlite`
- **Debian**, **Ubuntu:** `apt-get install libsqlite3-dev`
- **Arch:** `pacman -S sqlite`
