# crates.io publish status

| Item | Status |
| --- | --- |
| Package name | `image-to-pixel-art` |
| crates.io availability | **Available** (official API `GET /api/v1/crates/image-to-pixel-art` → 404; sparse index same) as of crate scaffolding |
| GitHub repo | https://github.com/sherotree/image-to-pixel-art-rs (public; confirmed) |
| TypeScript source | https://github.com/sherotree/image-to-pixel-art |
| Version publication | **Not published yet** — dry-run only until explicitly requested |

## Remaining release steps

1. Confirm target GitHub repo URL and that `Cargo.toml` `repository` / `homepage` match.
2. Confirm dry-run still passes:
   ```bash
   cargo package --allow-dirty
   cargo publish --dry-run --allow-dirty
   ```
3. As an account with crates.io publish rights for this crate name (owner session):
   ```bash
   cargo login
   # paste a crates.io API token from https://crates.io/settings/tokens
   cargo publish
   ```
   Do **not** paste tokens into agent chats or CI logs.
4. Tag the release (e.g. `v0.1.0`) after a successful publish.

Do **not** publish a version from an automated agent unless the owner explicitly asks for version publication.
