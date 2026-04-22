# AI Terminal

**Natural language terminal powered by AI.**

## What it does

Type what you mean, not shell syntax.

```bash
# Instead of:
sudo systemctl restart nginx && tail -f /var/log/nginx/error.log

# Just type:
ai-terminal --command "restart nginx and check the error logs"
```

## Features

- Natural language to shell command translation
- Command explanation (see what it will do)
- Safety: confirm before execution
- Context-aware suggestions

## Install

```bash
# From source
cargo build --release
cp target/release/ai-terminal ~/.local/bin/

# Or
cargo install --path .
```

## Usage

```bash
# Single command
ai-terminal --command "restart nginx"

# Interactive mode
ai-terminal --interactive
```

## Tech Stack

- Rust
- Anthropic Claude API
- tokio async runtime

## License

MIT
