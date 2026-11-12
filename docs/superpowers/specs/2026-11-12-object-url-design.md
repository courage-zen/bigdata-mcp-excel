# Design: write tools return the object URL

Date: 2026-11-12
Status: approved by user (all decision points confirmed via questions)

## Goal

`create_xlsx` and `sheet_apply` — the two tools that write a new object —
return the object's full object-store URL alongside `s3_key`, so agents can
hand the reference straight to consumers (chat messages, links) without
assembling it themselves. The emitted URL is a canonical reference (no
credentials, no presigning) in the same shape as platform attachment URLs,
which `sheet_read` already accepts back as input.

## Decisions (confirmed)

| Decision | Choice |
|---|---|
| URL kind | canonical reference, no signature (readers need their own access) |
| Field name | `object_url` |
| Covered tools | both write tools: `create_xlsx` and `sheet_apply` |
| Pure AWS S3 (no custom endpoint) | still emit a URL (standard AWS addressing) |

## URL construction

New module `src/objecturl.rs`:

```rust
pub fn object_url(s3: &S3Config, key: &str) -> String
```

Pure string building over already-validated keys (no I/O, no Result).
Follows the resolved `use_path_style` flag, mirroring how the
object_store client addresses the same object:

| Configuration | URL shape |
|---|---|
| custom endpoint, path-style (MinIO default) | `{endpoint}/{bucket}/{key}` |
| custom endpoint, virtual-host | `{scheme}://{bucket}.{host}/{key}` |
| no endpoint (AWS), virtual-host (default) | `https://{bucket}.s3.{region}.amazonaws.com/{key}` |
| no endpoint (AWS), path-style | `https://s3.{region}.amazonaws.com/{bucket}/{key}` |

Details:

- Endpoint: trailing `/`s trimmed; a value without a scheme is treated as
  `http://{value}`. `{host}` is the endpoint's `host[:port]` with the
  scheme stripped.
- Key: split on `/`, each segment percent-encoded with the
  non-alphanumeric set minus unreserved (`-._~`), segments rejoined by
  literal `/`. Non-ASCII (e.g. Chinese filenames) encodes as UTF-8
  percent-escapes. This makes keys with spaces/CJK/`%` produce legal URLs
  (`报表.xlsx` → `%E6%8A%A5%E8%A1%A8.xlsx`).
- Adds `percent-encoding = "2"` to Cargo.toml (object_store's codec, made
  explicit).

## resolve_input learns to decode

`src/s3keys.rs::resolve_input` currently compares URL paths to keys
literally, so any percent-encoded segment never matches its object. Fix:
after dropping the query string, percent-decode each path segment, then
run the existing host/bucket logic and `validate_input_key` on the
decoded path.

- Order matters: query-strip first, decode second — an encoded `?`
  (`%3F`) in a key must not be mistaken for a query separator.
- Lenient decoding (percent-encoding crate semantics): malformed escapes
  pass through literally; invalid UTF-8 after decoding rejects the input
  as an invalid key.
- Validation runs on the decoded key — the key S3 actually addresses — so
  traversal smuggled through escapes (`%2e%2e`) is still caught by the
  segment rules. Plain (non-URL) key inputs are untouched: a key with no
  escapes decodes to itself.
- Round-trip invariant (unit-tested, no S3 needed):
  `resolve_input(object_url(s3, key)) == key` for every configuration
  shape and for keys with spaces, CJK and `%`.

## Tool payloads

`src/tools.rs`:

- `create_xlsx` → `{"s3_key", "cells_written", "object_url",
  "content_b64?"}`
- `sheet_apply` → `{"s3_key", "cells_edited", "ops_applied",
  "object_url"}`

Tool descriptions and the server instructions gain one clause: write
tools return `object_url`, the canonical object-store URL of the output.

## Testing

- `objecturl` unit tests: the four configuration shapes; special
  characters (space, CJK, `%`) encode correctly.
- `s3keys` unit tests: encoded URLs decode to keys; encoded traversal
  (`%2e%2e`) rejected; literal IRI (un-encoded CJK) still passes through;
  round-trip over all shapes.
- Integration tests (existing testcontainers MinIO harness, `--ignored`):
  `create_xlsx`/`sheet_apply` responses contain `object_url`, and the URL
  passed back to `sheet_read` resolves to the same object.
- Release CI smoke: read back via the returned `object_url` instead of
  the raw key — exercises the round-trip on every release.

## Release

- Cargo.toml 0.3.0 → 0.4.0 (additive, backwards-compatible field).
- README: document `object_url` in the return payloads and note the
  URL/accept symmetry.
- Tag `v0.4.0` → Release CI (version-match gate passes).

## Acceptance checks

- [ ] Both write tools return `object_url`; URL matches the deployed
      endpoint/bucket/addressing style.
- [ ] `sheet_read` accepts the emitted URL back (round-trip), including
      keys with spaces/CJK.
- [ ] All existing tests pass; new unit + integration tests green.
- [ ] CI release workflow green on v0.4.0 with the usual 5 assets.
