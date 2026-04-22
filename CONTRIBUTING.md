# Contributing

## Setup

```bash
cargo build
cargo run -- --help
```

## Development

```bash
cargo watch --run cargo test
```

## Code Style

```bash
cargo fmt
cargo clippy -- -D warnings
```

## Testing

```bash
cargo test
```

## Release

```bash
# Update version in Cargo.toml
git tag v0.x.x
git push --tags
```
