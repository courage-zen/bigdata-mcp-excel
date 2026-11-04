# excel-mcp Design

A Rust-native MCP server that creates, reads, and edits Excel (.xlsx) workbooks
over Streamable HTTP, with MinIO/S3 as its only I/O surface. Deployed as a
stateless k8s Deployment; called by agents in other containers.

Rust was chosen over Go (engine crates: rust_xlsxwriter / calamine / ironcalc /
umya-spreadsheet; official `rmcp` SDK; no-GC streaming for large workbooks).
The project does NOT port genoffice-mcp's 80万行 TypeScript: the tool
contracts are borrowed, the implementation is rebuilt on crates.

## Goals

- Agents in other containers create and edit .xlsx via MCP tools.
- MinIO (in-cluster) is the only file transport: inputs fetched by s3_key,
  outputs uploaded and returned as s3_key (+ base64 when small).
- Stateless replicas; k8s-native ops (health probe, bearer auth, scaling).
- CI + release pipeline modeled on markitdown-mcp's (Rust translation).

## Non-goals (v1)

- Word/PowerPoint (docx stays on the existing TS MCP service).
- Rendering xlsx to PDF/HTML.
- Charts, images, pivot tables in sheet_apply ops (formatting-level only).
- Local-filesystem IO (S3 only; narrower attack surface).
- Streaming/row-by-row reads of huge workbooks (100MB cap like markitdown).

## Architecture

```
k8s Pod (stateless, horizontally scalable)
┌─────────────────────────────────────────────────────┐
│  excel-mcp (single binary: axum + rmcp)             │
│                                                     │
│  POST /mcp    Streamable HTTP MCP (sessions)        │
│  GET  /health liveness (auth-exempt)                │
│  middleware:  Bearer multi-key auth (MCP_AUTH_KEY)  │
│                                                     │
│  tools (rmcp #[tool]):                              │
│    create_xlsx / sheet_read / sheet_apply           │
│                                                     │
│  storage (object_store → MinIO):                    │
│    input:  fetch key → tempfile (100MB cap)         │
│    output: upload key → return (+ b64 if <2MB)      │
│                                                     │
│  engines (CPU work in spawn_blocking):              │
│    rust_xlsxwriter  create                         │
│    calamine + ironcalc  read (formula recompute)   │
│    umya-spreadsheet  edit (read-modify-write)      │
└─────────────────────────────────────────────────────┘
```

Single crate. Modules: `main.rs`, `config.rs`, `server.rs`, `auth.rs`,
`s3.rs`, `tools/` (one file per tool), `engine/` (create.rs, read.rs,
apply.rs). ~2–3k lines expected.

## Configuration (env)

Reuse markitdown-mcp names verbatim so ops doesn't relearn:

| Env | Meaning |
|---|---|
| `MCP_HOST` / `MCP_PORT` / `MCP_LOG_LEVEL` | bind, port, tracing filter |
| `MCP_AUTH_KEY` | comma-separated bearer keys; empty = no auth |
| `S3_BUCKET` / `S3_REGION` | input bucket |
| `AWS_ACCESS_KEY_ID` / `AWS_SECRET_ACCESS_KEY` | credentials |
| `AWS_ENDPOINT_URL` | MinIO endpoint; empty = real AWS |
| `S3_USE_PATH_STYLE` | default: true iff endpoint set |
| `S3_KEY_PREFIXES` | segment-aware input allowlist (comma-separated) |
| `S3_OUTPUT_PREFIX` | upload prefix, default `excel-mcp-out/` |
| `EXCEL_INLINE_MAX_BYTES` | base64-attach threshold, default 2MB |
| `EXCEL_MAX_FILE_BYTES` | download/upload cap, default 100MB |

## Tools

### create_xlsx

Input: `data` — a 2-D array of cell values, or `{"sheets": [{"name",
"rows"}]}`; strings starting with `=` are formulas (written as-is). Optional
`sheet_name` (single-sheet case). Optional `output_key` (explicit destination
key; default generated under `S3_OUTPUT_PREFIX`).

Output: `{s3_key, content_b64?, cells_written}` — b64 attached when the file
is under `EXCEL_INLINE_MAX_BYTES`.

Engine: rust_xlsxwriter. New workbooks only.

### sheet_read

Input: `s3_key`; optional `sheet` (name or index; default active), `range`
(A1 or row/col bounds), `max_rows` (default 500), `format` (`json` default |
`markdown`).

Output (json): sheet list with dimensions; `cells` keyed by A1 address with
`value` (recomputed), `formula` (when present); merged ranges; truncation
notices. Output (markdown): one table per sheet, values only.

Engine: calamine for structure/cached values → ironcalc recompute → merged
result. Formulas without cached values still return their computed number.

### sheet_apply

Input: `s3_key`; exactly one of `cells` (array of `{cell, sheet?, value? |
formula?, style?}`) or `ops` (v1 op set: `format_range`, `insert_rows`,
`insert_cols`, `add_sheet`, `rename_sheet`). Required `output_key` — edits
ALWAYS produce a new key; the source object is never overwritten.

Output: `{s3_key, cells_written | ops_applied}`.

Engine: umya-spreadsheet (read-modify-write; rust_xlsxwriter cannot reopen
files). Styles accepted in `cells[].style` and `format_range` use the same
field vocabulary.

## Security (inherited from markitdown-mcp)

- `validate_s3_key`: reject empty keys, path traversal (`..`), dangerous
  extensions, keys outside `S3_KEY_PREFIXES` (segment-aware matching).
- Download/upload bounded by `EXCEL_MAX_FILE_BYTES`.
- Download and engine work wrapped in timeouts.
- Error messages sanitized at the tool boundary: internal paths, hostnames,
  stack details only go to logs; agents see generic messages.

## HTTP surface

- rmcp `transport-streamable-http-server` feature over axum.
- Sessions: in-memory; random session ids; stateless request handling (any
  replica can serve any request, like markitdown-mcp).
- Bearer auth middleware: `Authorization: Bearer <key>` against the
  `MCP_AUTH_KEY` set; `/health` exempt; 401 JSON otherwise.
- `/health`: `{"ok": true, "version", "s3": "configured"|"not-configured"}`.

## Error handling & observability

- Tool errors → MCP `isError: true` with sanitized message.
- `tracing` + env-filter from `MCP_LOG_LEVEL`. One structured line per
  request/tool call/upload/download: key, bytes, duration, outcome.
- Panic isolation: engine work in spawn_blocking; a panic fails the call,
  not the server.

## CI & Release

Two workflows, structurally identical to markitdown-mcp's, translated to Rust.

### build.yml (push to main + PRs)

- `dtolnay/rust-toolchain@stable` + `Swatinem/rust-cache@v2`.
- `cargo fmt --check` → `cargo clippy -- -D warnings` → `cargo test`.
- Unit tests only here (MinIO integration tests are `#[ignore]`-gated;
  they need Docker, run locally and in release smoke).

### release.yml (tag `v*`)

| job | steps |
|---|---|
| `test` | same gate as build.yml |
| `image` | matrix amd64/arm64: QEMU + buildx build `deploy/Dockerfile` → `docker save \| gzip` → artifact `excel-mcp-image-linux-<arch>.tar.gz` |
| `smoke` | moto_server (pip) as the S3 backend on 0.0.0.0:9000; docker load amd64 image; run with env; wait /health; assert: no-token → 401, initialize OK, tools/list exposes exactly the 3 tools, create_xlsx → sheet_read → sheet_apply round-trip through s3 keys, bad key rejected with guidance |
| `package` | verify tag matches `Cargo.toml` version; `cargo build --release` per arch (via targets or QEMU); artifacts `excel-mcp-linux-<arch>` binaries |
| `publish` | merge artifacts → `sha256sum checksums.txt` → `softprops/action-gh-release@v2` with generate-release-notes |

Dockerfile: cargo-chef multi-stage (`rust:1-bookworm` builder →
`debian:bookworm-slim` runtime), HEALTHCHECK on `/health`, `CMD` runs the
binary with `MCP_HOST=0.0.0.0`.

## Testing

- **Unit**: key validation; tool param parsing; create→read round-trip
  (xlsx bytes in memory → calamine read back); formula recompute assertions;
  error sanitization.
- **Integration** (`#[ignore]`, needs Docker): testcontainers MinIO
  container; full fetch→create→read→apply→upload flow through the real HTTP
  server.
- **Smoke** (CI release only): moto as S3 backend against the real container
  image — see release.yml above.

## Deployment

`deploy/Dockerfile` + `deploy/k8s.yaml` mirroring markitdown-mcp's: 2
replicas, `/health` probes, resource limits, S3 + auth secrets injected as
env. Service on 80 → container 8080.

## Milestones

1. M1 skeleton + config + auth + health + S3 client + create_xlsx
2. M2 sheet_read (calamine + ironcalc)
3. M3 sheet_apply (umya) + v1 op set
4. M4 integration tests (MinIO) + Dockerfile + k8s.yaml + CI workflows
