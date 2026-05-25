# AGENTS.md

## Versioning

The `version` in `Cargo.toml` tracks the current Platzio release minor (see
`platzio/dev` AGENTS.md, "Versioning across repos"). Keep it matching the
release line — e.g. `0.7.0` while Platzio releases `0.7.0-beta.1`. The
`-beta.N` marker is not significant for this crate; prefer the plain
version, and don't let it drift behind the release.
