# Required output_key + Symmetric Output Allowlist

Retire `S3_OUTPUT_PREFIX`: every output key is agent-supplied and
workspace-scoped; output validation becomes symmetric with input validation.

## Background

`S3_OUTPUT_PREFIX` is static, process-level config. Workspace identity is
per-request knowledge that only the calling agent holds — no env var can
route outputs by workspace. Today an agent that omits `output_key` on
`create_xlsx` gets a uuid key under the shared prefix: the deliverable
silently escapes workspace isolation.

The pieces for the fix already exist: `create_xlsx` accepts an optional
`output_key` (`src/tools.rs:30`) and `sheet_apply` requires one. The
platform convention — same as bigdata-mcp-email's attachment model, where
the agent passes full workspace-scoped keys like
`workspace-42/reports/销量.xlsx` and the server treats them as opaque — is
that the agent holds complete keys. Making the same true for outputs aligns
both services on one mental model.

One hole closes along the way: `normalize_output_key` deliberately skips the
`S3_KEY_PREFIXES` allowlist (s3keys.rs comment: "outputs go to
`S3_OUTPUT_PREFIX`, not the input roots"). Once outputs are agent-chosen
keys landing in the `workspaces/` tree, that asymmetry means writes can
reach where reads cannot. Output validation should obey the same allowlist
as input validation.

## Goals

- Every output key is agent-supplied, workspace-scoped by convention
  (`workspaces/<ws-id>/<name>.xlsx`).
- Omitting `output_key` is a loud deserialization error, not a silent
  fallback to a shared prefix.
- Write access is no broader than read access: when `S3_KEY_PREFIXES` is
  set, output keys obey the same segment-aware allowlist.

## Non-goals

- Enforced per-workspace isolation: with a single `MCP_AUTH_KEY` the server
  cannot attribute a call to a workspace, so isolation stays
  convention-level. Stronger guarantees need per-workspace credentials or
  request context — out of scope, documented as such.
- URL tolerance for `output_key`: outputs are freshly minted keys, never
  attachment URLs (settled in the 2026-11-06 agent-hints spec).
- A separate `S3_OUTPUT_PREFIXES` config: rejected — the agent decides the
  destination, so operators cannot pre-enumerate legal output prefixes; the
  input allowlist already encodes the legal tree.
- Changing `sheet_apply`'s contract (already required; only its validation
  tightens).

## Design

### 1. API contract (`src/tools.rs`)

- `CreateXlsxArgs.output_key: Option<String>` → `String`. A missing field
  fails deserialization ("missing field `output_key`"), surfaced as an
  invalid-params MCP error — rejection happens before any S3 work.
- `create_xlsx` drops the generated-key fallback branch
  (`match args.output_key …` at `src/tools.rs:100`); it calls
  `normalize_output_key` directly, like `sheet_apply` already does.
- Arg doc comment becomes: "Full S3 key for the new workbook (required,
  must end with .xlsx); use the caller's workspace path, e.g.
  `workspaces/<ws-id>/<name>.xlsx`."
- Tool descriptions updated: `create_xlsx` states `output_key` is required
  and where it should point; `sheet_apply` gains the same workspace-path
  nudge. The server `instructions` paragraph mentions that outputs go to
  the caller's workspace path.

### 2. Validation merge (`src/s3keys.rs`)

- `normalize_output_key(key: &str, s3: &S3Config)` — signature gains the
  S3 config. Rules: existing format/traversal/dangerous-extension/`.xlsx`
  checks, PLUS the segment-aware `S3_KEY_PREFIXES` allowlist with semantics
  identical to `validate_input_key` (`workspaces/ws-1` matches itself and
  `workspaces/ws-1/a.xlsx`, not `workspaces/ws-11/a.xlsx`).
- No allowlist configured → outputs unrestricted, mirroring reads.
- The module comment explaining the old asymmetry is rewritten: validation
  is now symmetric — the same rule set guards both directions.
- Call sites in `create_xlsx` and `sheet_apply` pass `&s3`.

### 3. Prefix retirement (`src/config.rs`, `src/storage.rs`)

- Delete: `S3_OUTPUT_PREFIX` env read (`config.rs:60`),
  `Config.output_prefix` field, `storage::generated_output_key()` and its
  two unit tests.
- `.env.example` drops the `S3_OUTPUT_PREFIX` block; the README config
  table drops its row.

### 4. Deployment semantics

- Callers that relied on the generated-key default now fail explicitly —
  the intended loud failure, not a regression.
- Overwrite: an existing object at the destination key is overwritten
  (idempotent re-runs); the uuid generator's collision-freedom is gone by
  design. README states this.
- Isolation remains convention-level; README says so plainly, without
  overclaiming.
- Breaking API change → version bump to 0.2.0.

## Testing

- Unit (`s3keys.rs`): output key inside/outside the allowlist; segment
  boundary (`ws-1` vs `ws-11`); unrestricted when no allowlist; existing
  suffix/traversal cases updated for the new signature.
- Unit (`tools.rs`): `create_xlsx` without `output_key` rejected with a
  missing-field message before any S3 access; `create_xlsx` with an
  out-of-allowlist `output_key` rejected with the allowlist message using
  the unreachable-endpoint test config (proves rejection precedes network).
- Unit (`config.rs`): defaults test drops the `output_prefix` assertion.
- Integration (`--ignored` suite): existing cases' explicit output keys
  move from `excel-mcp-out/…` to `workspaces/…` shapes; new round-trip —
  `create_xlsx` to `workspaces/<ws>/<name>.xlsx`, then `sheet_read` it
  back.
- `tools.rs::test_cfg()` drops the `output_prefix` field.

## Rollout

- Version 0.2.0; README config table, usage examples
  (`reports/fruit.xlsx` → workspace-shaped keys) and the overwrite note
  updated in the same change.
- Platform agent team told: agents must pass
  `workspaces/<ws-id>/<name>.xlsx` as `output_key`.
