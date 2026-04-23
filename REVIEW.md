# Code Review — heorot

This document lists every issue found during a review of the current codebase.
Each entry includes a suggested GitHub label so that issues can be triaged and
tracked consistently.

---

## Label Definitions

| Label | Description |
|---|---|
| `bug` | Code that will not compile, panics at runtime, or produces wrong results |
| `architecture` | Design-level concern: wrong abstraction, misused pattern, missing component |
| `code-quality` | Code that works but is fragile, hard to maintain, or violates best practice |
| `documentation` | Missing, incomplete, or misleading documentation / comments |

---

## Issues

### 1 — Missing `Cargo.toml` build manifest
**Label:** `bug` / `architecture`

There is no `Cargo.toml` at the root of the repository. The project cannot be
compiled or have its dependencies resolved without it.  A `Cargo.toml` must
declare the crate name, edition, and any future dependencies (e.g. `serde`,
`serde_json`, a real embedded-database crate).

---

### 2 — Missing crate root (`src/main.rs` or `src/lib.rs`)
**Label:** `bug` / `architecture`

Rust requires a crate root that declares modules with `mod` statements.
Currently there is no `src/main.rs` (binary) or `src/lib.rs` (library) at the
top level.  `src/warehouse/lib.rs` references `crate::models`,
`crate::storage_db`, and `crate::storage_file`, but no file stitches those
modules together under the crate root.

---

### 3 — `storage_db.rs`: `save_item` and `load_item` use `self` without a receiver parameter
**Label:** `bug`

`DatabaseStorage::save_item` (line 50) and `DatabaseStorage::load_item`
(line 85) both call `self.get_db()`, but neither function signature declares
`&self`.  This is a compile-time error in Rust.  The methods must either:
- add `&self` / `&mut self` to their signatures, **or**
- be converted to free functions that accept the database reference explicitly.

---

### 4 — `storage_db.rs`: `OnceLock::sync_once` does not exist
**Label:** `bug`

Line 30 calls `OnceLock::sync_once(|| db.clone()).map(|d| d)`, which is not a
method that exists on `std::sync::OnceLock`.  The correct API is
`once_lock.get_or_init(|| …)` for lazy initialisation, or `once_lock.set(…)`
for a one-time write.  The current code will not compile.

---

### 5 — `storage_db.rs`: `Mutex<…>` does not implement `Clone`
**Label:** `bug`

Line 30 calls `db.clone()` where `db: Mutex<HashMap<String, LlmItem>>`.
`Mutex<T>` deliberately does not implement `Clone` in the Rust standard
library.  The clone call is a compile error and must be removed.

---

### 6 — `storage_db.rs`: INSERT branch does not actually insert the item
**Label:** `bug`

In `save_item`, the `else` branch for a new item (lines 72-75) only logs a
message; it never calls `db.insert(item.id.clone(), item.clone())`.  New items
are silently discarded.

---

### 7 — `lib.rs`: `storage_db::save_item` / `load_item` called as free functions
**Label:** `bug` / `architecture`

`lib.rs` calls `storage_db::save_item(item)` (line 57) and
`storage_db::load_item(item_id)` (line 89) as if they were free functions in
the `storage_db` module.  They are defined only as associated functions on
`DatabaseStorage`, so these call sites do not resolve.  A global/static
`DatabaseStorage` instance is needed, or the `Warehouse` struct must hold an
instance and call methods on it.

---

### 8 — `lib.rs`: `storage_file::save_item` / `load_item` called as free functions
**Label:** `bug` / `architecture`

Same problem as issue 7 but for `storage_file`.  `lib.rs` calls
`storage_file::save_item(item)` (line 69) and `storage_file::load_item(item_id)`
(line 101) as free functions, but those methods are associated functions on
`FileStorage`.  `FileStorage` must either expose top-level free functions or
`Warehouse` must call them as `FileStorage::save_item(…)`.

---

### 9 — `lib.rs`: File-directory creation error is silently ignored
**Label:** `code-quality`

In `Warehouse::new`, when `std::fs::create_dir_all` fails (lines 36-38), only
a warning is printed and the error is dropped — execution continues as if the
directory exists.  Any subsequent `save_item` call will then fail with a
misleading I/O error.  The error should either be propagated via `return Err(…)`
or the method should be documented as tolerating missing directories (and the
callers adjusted accordingly).

---

### 10 — `storage_file.rs`: Double path separator in constructed file path
**Label:** `bug`

`storage_dir` is defined as `"data/warehouse/"` (trailing slash), and the file
path is built with `format!("{}/{}.json", storage_dir, item.id)`, producing
`data/warehouse//item.json` (double slash).  While most operating systems
tolerate this, it is unclean and can cause problems on some platforms or with
path comparison logic.  Either remove the trailing slash from `storage_dir` or
change the format string to `"{}{}.json"`.

---

### 11 — `storage_file.rs`: `save_item` truncates `chunk_text` — data loss on persist
**Label:** `bug`

The serialised JSON written to disk truncates `chunk_text` to 50 characters
and appends a literal `"..."` suffix (lines 52-64).  This means the full
content can never be recovered from the file, making the file-based fallback
path in `retrieve_context` useless for real workloads.  `serde` /
`serde_json` should be used to serialize the complete struct.

---

### 12 — `storage_file.rs`: `load_item` returns mock data instead of deserialising the file
**Label:** `bug`

`load_item` checks that the file exists but then ignores its content and
returns a hardcoded placeholder `LlmItem` (lines 107-113) with a dummy
timestamp, empty `original_source`, and fabricated `chunk_text`.  All data
stored to disk is effectively unreadable.  The method must read the file and
deserialise it (e.g. with `serde_json`).

---

### 13 — `models.rs`: `SourceType` enum is defined but never used in `LlmItem`
**Label:** `architecture` / `code-quality`

`LlmItem.source_type` is a plain `String` (line 42), while the `SourceType`
enum (lines 57-61) is defined in the same file but only used as a constructor
parameter in `LlmItem::new`.  The constructor immediately converts it back to a
debug string with `format!("{:?}", source_type)` (line 103), losing type
safety.  `LlmItem.source_type` should be typed as `SourceType`, and
`SourceType` should derive or implement `Display` for human-readable output.

---

### 14 — `models.rs`: `unwrap()` on `SystemTime::duration_since` can panic
**Label:** `code-quality`

Line 98 calls `.unwrap()` on the result of
`SystemTime::now().duration_since(UNIX_EPOCH)`.  While this specific call is
unlikely to fail in practice (the system clock should always be after the Unix
epoch), `unwrap()` on `Result` / `Option` is a code-smell in library code
because it causes a panic on unexpected input.  The error should be handled
with `unwrap_or_default()`, `expect("…")` with a clear message, or propagated.

---

### 15 — Excessive `println!` debug logging throughout all modules
**Label:** `code-quality`

Every function in every module emits multiple `println!` macro calls for
step-by-step tracing.  This produces extremely noisy output in any environment
and is not suitable for a library crate.  Structured logging (e.g. the `log`
crate with `tracing` or `env_logger`) should replace raw `println!`, allowing
callers to control verbosity.  `println!` from `From` trait impls (e.g.
`models.rs` line 29) is especially problematic because it fires on every error
conversion.

---

### 16 — README is nearly empty — no setup, build, or usage documentation
**Label:** `documentation`

`README.md` contains only the project name and a one-line description.  There
are no instructions for building the project, adding the required dependencies,
running tests, or understanding the public API.  At a minimum the README should
include: prerequisites, how to add to `Cargo.toml`, a quick-start example, and
a description of the module layout.

---

*Review conducted by GitHub Copilot — April 2026*
