# Required output_key + Symmetric Output Allowlist Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Retire `S3_OUTPUT_PREFIX` — every output key is agent-supplied and workspace-scoped, and output validation obeys the same `S3_KEY_PREFIXES` allowlist as input validation.

**Architecture:** Three small moves in one direction. (1) `CreateXlsxArgs.output_key` becomes required, deleting the generated-key fallback. (2) `normalize_output_key` gains the `S3Config` parameter and applies `validate_input_key`'s segment-aware allowlist — write access never broader than read access. (3) `Config.output_prefix`, its env read, `storage::generated_output_key` and the `uuid` dependency all go away.

**Tech Stack:** Rust, rmcp (MCP server + `#[tool]` macros), schemars/serde for arg deserialization, object_store for S3, testcontainers-MinIO for `--ignored` integration tests.

**Spec:** `docs/superpowers/specs/2026-11-11-required-output-key-design.md`

**Baseline commands** (the whole plan uses these):
- Unit tests: `cargo test --lib --bins` (runs all embedded `#[cfg(test)]` modules)
- Format/lint gate before every commit: `cargo fmt && cargo clippy --all-targets`
- Integration suite (needs Docker; Task 9 only): `cargo test --features integration -- --ignored`

**Conventions:** Every commit ends with the attribution trailer:

```
Co-Authored-By: Claude Code <noreply@anthropic.com>
```

---

### Task 1: `normalize_output_key` enforces the allowlist (s3keys.rs)

**Files:**
- Modify: `src/s3keys.rs:112-134` (`normalize_output_key`)
- Modify: `src/s3keys.rs:108-111` (doc comment)
- Test: `src/s3keys.rs` embedded tests

- [ ] **Step 1: Write the failing tests**

In `src/s3keys.rs`'s `mod tests`, replace the two `normalize_output_key` tests (currently `output_key_requires_xlsx_suffix` and `output_key_rejects_traversal`, lines ~195-206) with these four; the `s3()` helper (bucket "b", empty-prefixes constructor at ~line 140) already exists:

```rust
    #[test]
    fn output_key_requires_xlsx_suffix() {
        assert!(normalize_output_key("out/a.xlsx", &s3(&[])).is_ok());
        assert!(normalize_output_key("out/a.csv", &s3(&[])).is_err());
        assert!(normalize_output_key("out/a.XLSX", &s3(&[])).is_ok());
    }

    #[test]
    fn output_key_rejects_traversal() {
        assert!(normalize_output_key("out/../etc/passwd.xlsx", &s3(&[])).is_err());
        assert!(normalize_output_key("/abs.xlsx", &s3(&[])).is_err());
    }

    #[test]
    fn output_key_allowlist_symmetric_with_input() {
        // Writes obey the same segment-aware allowlist as reads: keys
        // outside every configured prefix are refused.
        let cfg = s3(&["workspaces/ws-1"]);
        assert!(normalize_output_key("workspaces/ws-1/rep.xlsx", &cfg).is_ok());
        assert!(normalize_output_key("workspaces/ws-1", &cfg).is_err()); // not .xlsx
        assert!(normalize_output_key("workspaces/ws-11/rep.xlsx", &cfg).is_err());
        assert!(normalize_output_key("excel-mcp-out/a.xlsx", &cfg).is_err());
        assert!(normalize_output_key("other/a.xlsx", &cfg).is_err());
    }

    #[test]
    fn output_key_unrestricted_without_allowlist() {
        let cfg = s3(&[]);
        assert!(normalize_output_key("any/where/a.xlsx", &cfg).is_ok());
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --lib s3keys`
Expected: compile error — `normalize_output_key` takes 1 argument but the new tests call it with 2 (that is the failing state; do not change the signature yet).

- [ ] **Step 3: Implement**

Replace `normalize_output_key` and its doc comment (src/s3keys.rs:108-134) with:

```rust
/// Normalize an output key: the full input rule set — format, traversal,
/// dangerous extensions, segment-aware allowlist — plus the .xlsx suffix
/// rule. Validation is symmetric: writes can never reach where reads
/// cannot.
pub fn normalize_output_key(key: &str, s3: &S3Config) -> Result<String, InvalidKey> {
    let key = key.trim();
    if key.is_empty() {
        return Err(InvalidKey("key is empty".into()));
    }
    if key.starts_with('/')
        || key.contains('\\')
        || key.contains("//")
        || key
            .split('/')
            .any(|seg| seg.is_empty() || seg == "." || seg == "..")
    {
        return Err(InvalidKey("key must be a plain '/'-separated path".into()));
    }
    let lower = key.to_lowercase();
    if DANGEROUS_EXTENSIONS.iter().any(|ext| lower.ends_with(ext)) {
        return Err(InvalidKey("key extension is refused".into()));
    }
    if !lower.ends_with(".xlsx") {
        return Err(InvalidKey("output key must end with .xlsx".into()));
    }
    if !s3.allowed_key_prefixes.is_empty()
        && !s3.allowed_key_prefixes.iter().any(|p| {
            key == p.as_str()
                || key.starts_with(p.as_str())
                    && (p.ends_with('/') || key.as_bytes().get(p.len()) == Some(&b'/'))
        })
    {
        return Err(InvalidKey("key is outside the configured allowlist".into()));
    }
    Ok(key.to_string())
}
```

The allowlist block is byte-for-byte `validate_input_key`'s (src/s3keys.rs:46-54). Deliberately keep the duplicate rather than extracting a helper: the shared core is one 9-line predicate, and the comment on `validate_input_key` documents the input side.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --lib s3keys`
Expected: all s3keys tests PASS. Note `tools.rs` and `integration.rs` do not compile yet (call sites still pass 1 argument) — expected; fixed in Task 3/9.

- [ ] **Step 5: Commit**

```bash
git add src/s3keys.rs
git commit -m "feat: output keys obey the S3_KEY_PREFIXES allowlist

normalize_output_key now takes the S3 config and enforces the same
segment-aware prefix allowlist as validate_input_key, so write access
is never broader than read access.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 2: `create_xlsx` requires `output_key` (tools.rs)

**Files:**
- Modify: `src/tools.rs:19-31` (`CreateXlsxArgs`)
- Test: `src/tools.rs` embedded tests

- [ ] **Step 1: Write the failing test**

In `src/tools.rs`'s `mod tests`, add:

```rust
    #[tokio::test]
    async fn create_xlsx_rejects_missing_output_key() {
        // The generated-key fallback is gone: a missing output_key must
        // fail at deserialization, before any engine or S3 work.
        let raw = serde_json::json!({"data": [[1]]});
        let parsed: Result<CreateXlsxArgs, _> = serde_json::from_value(raw);
        let err = parsed.expect_err("missing output_key must fail to deserialize");
        assert!(
            err.to_string().contains("output_key"),
            "error names the field: {err}"
        );
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --lib create_xlsx_rejects_missing`
Expected: FAIL — the field is `Option<String>` with `#[serde(default)]`, so deserialization succeeds.

- [ ] **Step 3: Implement**

In `src/tools.rs`, change `CreateXlsxArgs.output_key` (lines 27-30) from:

```rust
    /// Optional explicit output key (must end with .xlsx); default is
    /// generated under S3_OUTPUT_PREFIX.
    #[serde(default)]
    pub output_key: Option<String>,
```

to:

```rust
    /// Full S3 key for the new workbook (required, must end with .xlsx);
    /// use the caller's workspace path, e.g. workspaces/<ws-id>/<name>.xlsx.
    pub output_key: String,
```

The old `create_xlsx_errors_without_s3` test (lines 285-307) constructs `CreateXlsxArgs` directly with `output_key: None` — update it to `output_key: "excel-mcp-out/a.xlsx".into()`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --lib tools`
Expected: FAIL to compile — the tool body (Task 3) and `output_prefix` removal (Task 4) come next. `create_xlsx_rejects_missing_output_key` still cannot run.
**Verify only the deserialization test compiles and passes in isolation:**

Run: `cargo test --lib create_xlsx_rejects_missing_output_key`
Expected: if still uncompilable due to the tool body, proceed to Task 3 and return; if the crate compiles, PASS.

- [ ] **Step 5: Commit**

```bash
git add src/tools.rs
git commit -m "feat: create_xlsx output_key is required

A missing output_key now fails deserialization with a field-naming
error instead of silently falling back to a uuid key under the shared
S3_OUTPUT_PREFIX — the deliverable can no longer escape the caller's
workspace by omission.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 3: Tool bodies drop the fallback (tools.rs)

**Files:**
- Modify: `src/tools.rs:78-133` (`create_xlsx`), `src/tools.rs:175-240` (`sheet_apply`)
- Test: `src/tools.rs` embedded tests

- [ ] **Step 1: Write the failing test**

In `src/tools.rs`'s `mod tests`, add:

```rust
    #[tokio::test]
    async fn create_xlsx_rejects_out_of_allowlist_output_key() {
        // With an allowlist configured, a write outside it is refused
        // before any network call — same guarantee reads have.
        let mut cfg = url_cfg();
        cfg.s3.as_mut().unwrap().allowed_key_prefixes = vec!["workspaces/".into()];
        let server = ExcelMcp::new(cfg);
        let result = server
            .create_xlsx(Parameters(CreateXlsxArgs {
                data: serde_json::json!([[1]]),
                sheet_name: None,
                output_key: "excel-mcp-out/a.xlsx".into(),
            }))
            .await;
        let msg = match &result {
            Err(e) => e.message.to_string(),
            Ok(r) => extract_text(r).unwrap_or_default(),
        };
        assert!(
            msg.contains("allowlist"),
            "expected allowlist rejection, got: {msg}"
        );
    }
```

`url_cfg()` (tools.rs:388-406) is reused as the base: S3 configured but endpoint unreachable (`http://127.0.0.1:1`), so a validation pass would surface as a network error, not an allowlist error. The existing `create_xlsx_rejects_bad_output_key` (lines 310-345) keeps its `url_cfg()` and should still pass unchanged — it rejects on the `.xlsx` suffix rule, which precedes the allowlist.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --lib create_xlsx_rejects_out_of_allowlist`
Expected: FAIL to compile (`normalize_output_key` arity) — after implementing Step 3 it must run and fail on the assertion, proving validation currently doesn't restrict outputs. Sequence: implement Step 3, then run again before Task 4; assert FAIL on `msg.contains("allowlist")` (fallback path returns a generated key, message is empty → assertion fails).

- [ ] **Step 3: Implement**

In `create_xlsx` (src/tools.rs:99-104), replace the resolution block:

```rust
        // Resolve output key: explicit (validated) or generated.
        let output_key = match args.output_key.as_deref() {
            Some(explicit) => crate::s3keys::normalize_output_key(explicit)
                .map_err(|e| McpError::invalid_params(e.to_string(), None))?,
            None => crate::storage::generated_output_key(&self.cfg.output_prefix),
        };
```

with:

```rust
        // Output key: agent-supplied, validated with the full input rule
        // set plus the .xlsx suffix rule.
        let output_key = crate::s3keys::normalize_output_key(&args.output_key, &s3)
            .map_err(|e| McpError::invalid_params(e.to_string(), None))?;
```

Note the moved validation: it needs `&s3`, and `s3` is cloned later in the body — the `let s3 = self.cfg.s3.clone()...` block (lines 105-110) moves ABOVE the validation so it is in scope. Full ordering of the reworked body head:

```rust
        let cells = crate::engine::create::count_cells(&sheets);
        let bytes = tokio::task::spawn_blocking(move || crate::engine::create::build_xlsx(&sheets))
            .await
            .map_err(|_| McpError::internal_error("engine task failed", None))?
            .map_err(|e| McpError::invalid_params(e.to_string(), None))?;
        let s3 = self.cfg.s3.clone().ok_or_else(|| {
            McpError::internal_error(
                "S3 output is not configured on this server; ask the operator to set S3_BUCKET and credentials",
                None,
            )
        })?;
        let output_key = crate::s3keys::normalize_output_key(&args.output_key, &s3)
            .map_err(|e| McpError::invalid_params(e.to_string(), None))?;
```

In `sheet_apply` (src/tools.rs:190-191), the call becomes:

```rust
        let output_key = crate::s3keys::normalize_output_key(&args.output_key, &s3)
            .map_err(|e| McpError::invalid_params(e.to_string(), None))?;
```

(`s3` is already resolved above it — only the argument changes.)

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --lib`
Expected: FAIL to compile — `test_cfg()` and `url_cfg()` still construct `Config { output_prefix, … }` (fixed in Task 4). After Task 4 Step 3, this suite must fully PASS.

- [ ] **Step 5: Commit**

The tool body (this task) and the `Config` field removal (Task 4) land as one compilable unit: if the crate does not compile at this point, hold this commit and make it together with Task 4's.

```bash
git add src/tools.rs
git commit -m "feat: create_xlsx validates agent-supplied output keys

The generated-key fallback branch is deleted; both create_xlsx and
sheet_apply now pass the S3 config to normalize_output_key so output
keys are checked against the full symmetric rule set.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

(Commit the tool-body change together with Task 4 if the crate doesn't compile yet — see above.)

---

### Task 4: Retire `Config.output_prefix` and the uuid generator (config.rs, storage.rs, Cargo.toml, server.rs)

**Files:**
- Modify: `src/config.rs:15` (struct field), `src/config.rs:60-62` (env read), `src/config.rs:145` (defaults test), `src/config.rs:65-73` (constructor)
- Modify: `src/storage.rs:101-110` (`generated_output_key`), `src/storage.rs:147-158` (its tests)
- Modify: `src/server.rs:49` (`test_config`)
- Modify: `src/tools.rs` `test_cfg()`/`url_cfg()` (output_prefix fields)
- Modify: `Cargo.toml` — no change (uuid stays; see Step 1)

- [ ] **Step 1: Remove the field, env read and struct literal**

In `src/config.rs`:
- Delete `pub output_prefix: String,` (line 15) from `struct Config`.
- Delete lines 60-62:

```rust
        let output_prefix = get("S3_OUTPUT_PREFIX")
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "excel-mcp-out/".into());
```

- In the `Config { … }` literal (lines 65-73), delete `output_prefix,`.
- In `defaults_when_env_empty` (test, line 145), delete `assert_eq!(cfg.output_prefix, "excel-mcp-out/");`.

In `src/server.rs` `test_config` (line 49): delete `output_prefix: "excel-mcp-out/".into(),`.

In `src/tools.rs` `test_cfg()` (line 272) and `url_cfg()` (line 402): delete their `output_prefix: …` lines.

In `src/storage.rs`: delete `generated_output_key` (lines 101-110) and its two tests `output_key_generation` / `output_key_generation_adds_slash` (lines 148-158).

In `Cargo.toml`: **keep** `uuid = { version = "1", features = ["v4"] }` (line 21) — `src/engine/apply.rs:207,214` uses `uuid::Uuid::new_v4()` for temp-file names. The spec retires only the S3-key generator, not the uuid crate; no manifest change.

- [ ] **Step 2: Run full unit suite**

Run: `cargo test --lib --bins && cargo fmt && cargo clippy --all-targets`
Expected: all PASS, no warnings. If `uuid` unused warnings appear anywhere, they won't (apply.rs keeps it).

- [ ] **Step 3: Commit**

```bash
git add src/config.rs src/storage.rs src/server.rs src/tools.rs
git commit -m "refactor: retire S3_OUTPUT_PREFIX and generated output keys

Config.output_prefix, its env read and storage::generated_output_key
are deleted: every output key is agent-supplied. The uuid dependency
stays (engine/apply.rs temp files).

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

(Note: no Cargo.toml change — the uuid correction above.)

---

### Task 5: Rewrite tool copy for required outputs (tools.rs)

**Files:**
- Modify: `src/tools.rs:78-80` (`create_xlsx` description), `src/tools.rs:135` table row for output copy, `src/tools.rs:244-259` (server instructions)

- [ ] **Step 1: Update the `create_xlsx` `#[tool(description)]`**

From the current text (line 79), change the tail after "…when the deliverable is a real spreadsheet." — append:

```rust
    #[tool(
        description = "Create a new Excel (.xlsx) workbook from data: a 2-D array of cell values, or {\"sheets\":[{\"name\",\"rows\"}]} for several sheets. Strings starting with \"=\" are formulas. Prefer this over emitting CSV/markdown tables when the deliverable is a real spreadsheet. output_key is required: give the full destination S3 key ending in .xlsx, using the caller's workspace path (workspaces/<ws-id>/<name>.xlsx). Existing objects at that key are overwritten. Returns the output s3_key and the file bytes base64-encoded when small."
    )]
```

- [ ] **Step 2: Update the `sheet_apply` description (line 176)**

Current: "…output_key must be an S3 key ending in .xlsx."
New: "…output_key must be the full destination S3 key ending in .xlsx, using the caller's workspace path (workspaces/<ws-id>/<name>.xlsx); an existing object there is overwritten."

- [ ] **Step 3: Update the server `instructions` (lines 247-253)**

Append to the existing paragraph:

```rust
                 " Every output goes to an explicit output_key you choose under \
                  the caller's workspace path (workspaces/<ws-id>/…); outputs \
                  never land in a shared default prefix.",
```

- [ ] **Step 4: Run tests**

Run: `cargo test --lib && cargo fmt && cargo clippy --all-targets`
Expected: PASS (descriptions are data, not logic).

- [ ] **Step 5: Commit**

```bash
git add src/tools.rs
git commit -m "docs: tool copy reflects required workspace-scoped output_key

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 6: Update `.env.example` and README

**Files:**
- Modify: `.env.example:38-40`
- Modify: `README.md:25-71` (example calls), `README.md:88` (config table row), `README.md:127-185` (中文说明 mention if needed)

- [ ] **Step 1: `.env.example` — delete the S3_OUTPUT_PREFIX block (lines 38-40):**

```text
# S3_OUTPUT_PREFIX: prefix under which generated output keys are created
# (default excel-mcp-out/)
S3_OUTPUT_PREFIX=excel-mcp-out/
```

and extend the S3_KEY_PREFIXES comment to cover both directions. Replace lines 32-35 with:

```text
# S3_KEY_PREFIXES: comma-segment key-prefix allowlist constraining BOTH
# readable inputs and writable outputs (e.g. "workspaces/"). Empty = no
# restriction: only safe when every MCP_AUTH_KEY holder may read and
# write the whole bucket.
S3_KEY_PREFIXES=
```

- [ ] **Step 2: README — config table row 88 becomes:**

```markdown
| `S3_KEY_PREFIXES` | *(empty)* | segment-aware allowlist for readable inputs AND writable outputs |
```

(delete the `S3_OUTPUT_PREFIX` row entirely), and replace the `create_xlsx` example call (lines 29-44) with a workspace-shaped key:

```json
{
  "jsonrpc": "2.0", "id": 1, "method": "tools/call",
  "params": {
    "name": "create_xlsx",
    "arguments": {
      "data": [
        ["item", "qty", "total"],
        ["apple", 3, "=B2*2"]
      ],
      "sheet_name": "Sheet1",
      "output_key": "workspaces/ws-42/reports/fruit.xlsx"
    }
  }
}
```

Update the response line and the `sheet_read`/`sheet_apply` examples' keys (lines 50-71) to the same workspace shape (`workspaces/ws-42/reports/fruit.xlsx`, `workspaces/ws-42/reports/fruit-v2.xlsx`), plus a one-line note after the examples: "If an object already exists at `output_key`, it is overwritten (idempotent re-runs)."

- [ ] **Step 3: Verify docs build sanity**

Run: `grep -n "S3_OUTPUT_PREFIX\|excel-mcp-out" README.md .env.example`
Expected: no output (all references gone).

- [ ] **Step 4: Commit**

```bash
git add .env.example README.md
git commit -m "docs: required workspace-scoped output_key in README and env example

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 7: README version bump 0.1.2 → 0.2.0

**Files:**
- Modify: `Cargo.toml:3` (`version`)
- Modify: `README.md` — image tag (line 145: `excel-mcp:0.1.2` → `:0.2.0`), health-endpoint example (line 93: `"version":"0.1.2"` → `"0.2.0"`)

- [ ] **Step 1: Bump version**

`Cargo.toml` line 3: `version = "0.2.0"`. This is a breaking contract change: callers that omit `output_key` now fail.

- [ ] **Step 2: Run tests**

Run: `cargo test --lib --bins && cargo fmt && cargo clippy --all-targets`
Expected: PASS. `VERSION` (server.rs:14) reads `CARGO_PKG_VERSION`, so `/health` reports 0.2.0 automatically.

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock README.md
git commit -m "chore: bump 0.2.0 (breaking: output_key required)

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

(`Cargo.lock` updates from the version bump; include it.)

---

### Task 8: Integration test updates (tests/integration.rs)

**Files:**
- Modify: `tests/integration.rs:393-457` (roundtrip), `tests/integration.rs:428` (apply key)

- [ ] **Step 1: Update the roundtrip test**

In `create_read_apply_roundtrip` (lines 393-457):
- `create_xlsx` call (lines 397-403): add `"output_key": "workspaces/ws-42/inventory.xlsx"` to the arguments json.
- Line 405: `assert!(s3_key.starts_with("excel-mcp-out/"), …)` → `assert_eq!(s3_key, "workspaces/ws-42/inventory.xlsx");`
- Line 428: `"output_key": "excel-mcp-out/edited.xlsx"` → `"output_key": "workspaces/ws-42/edited.xlsx"`
- Lines 436, 440, 446: replace the three `"excel-mcp-out/edited.xlsx"` literals with `"workspaces/ws-42/edited.xlsx"`.

- [ ] **Step 2: Verify with the ignored suite (needs Docker)**

Run: `cargo test --features integration -- --ignored`
Expected: all integration tests PASS (the integration Stack does not set S3_KEY_PREFIXES, so workspace keys validate freely).

- [ ] **Step 3: Commit**

```bash
git add tests/integration.rs
git commit -m "test: integration roundtrip uses workspace-shaped output keys

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

## Self-Review

**Spec coverage check:**

| Spec section | Task |
|---|---|
| §1 API contract: required field, fallback deleted, doc comments, tool descriptions, instructions | Tasks 2, 3, 5 |
| §2 Validation merge: signature, symmetric allowlist, module comment, call sites | Task 1, 3 |
| §3 Prefix retirement: config field, env read, generated_output_key, .env.example, README | Tasks 4, 6 |
| §4 Deployment: breaking-change loud failure, overwrite documented, 0.2.0 | Tasks 6, 7 |
| Testing: s3keys allowlist unit tests, tools missing-field + allowlist rejection tests, config defaults, integration roundtrip workspace keys | Tasks 1-4, 8 |
| Rollout: version bump, README examples, agent-team notice | Task 7, 6 |

**Placeholder scan:** Task 3 Step 5 contained a stray `git compile 2>/dev/null || true` line in a draft — flagged and replaced with the conditional-commit instruction. No TBD/TODO/"add tests" placeholders remain. The only "similar to" reuse is `url_cfg()` (a real existing helper at tools.rs:388), not a cross-task forward reference.

**Type consistency check:** `normalize_output_key(key: &str, s3: &S3Config)` — Task 1 defines it, Tasks 3 and 9 (no Task 9 exists; integration is Task 8) call it with `(key, &s3)`. `CreateXlsxArgs.output_key: String` — Task 2 defines, Tasks 3/8 construct with `.into()`. `test_cfg()`/`url_cfg()` field removal in Task 4 is referenced by Tasks 2's compile note. Verified consistent.

**One correction applied during self-review:** Cargo.toml's `uuid` dependency is NOT removed (spec §3 says delete the generator; `engine/apply.rs` legitimately keeps the uuid crate for temp-file names). Task 4 reflects this.
