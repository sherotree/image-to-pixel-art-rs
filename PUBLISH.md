# crates.io publish status

| Item | Status |
| --- | --- |
| Crate name | `image-to-pixel-art` |
| crates.io availability | **Available** (official API `GET /api/v1/crates/image-to-pixel-art` → 404 “does not exist”, checked with User-Agent) |
| GitHub repo | https://github.com/sherotree/image-to-pixel-art-rs (public) |
| Version publication | **Not published yet** — dry-run only until owner auth + metadata are settled |

## Remaining release steps (owner)

1. Confirm packaging dry-run still passes from this repository root:
   ```bash
   cargo fmt --check
   cargo test
   cargo clippy --all-targets -- -D warnings
   cargo publish --dry-run
   ```
2. Log in to crates.io as the intended owner (GitHub OAuth), then create an API token:
   - https://crates.io/settings/tokens
3. Authenticate locally (do **not** commit tokens):
   ```bash
   cargo login
   # paste the crates.io API token
   ```
4. Confirm `Cargo.toml` owners / authors / repository / homepage point at this repo, then publish **only** when explicitly requested:
   ```bash
   cargo publish
   ```
5. Tag the release (e.g. `v0.1.0`) after a successful publish.

Do **not** publish a version from an automated agent unless the owner explicitly asks for crates.io publication.
