# AI Terminal

**Natural language terminal powered by AI.**

![License](https://img.shields.io/badge/License-MIT-green)
![Rust](https://img.shields.io/badge/Rust-1.75-black)

## What it does

Type what you mean in English, not shell syntax.

```bash
# Instead of memorizing:
sudo systemctl restart nginx
sudo journalctl -u nginx -n 50
tail -f /var/log/nginx/error.log

# Just say:
ai-terminal "restart nginx and check the logs"
```

## Features

- 🎯 **Natural Language** — Type your intent, not commands
- 🔒 **Safe by Default** — Preview before execution
- 📖 **Educational** — Shows what commands will run
- 🌐 **Multi-Language** — Claude supports 100+ languages
- 🔒 **Privacy Mode** — Local model option available

## Install

### From Binary

Download from [Releases](https://github.com/Hardonian/ai-terminal/releases)

### From Source

```bash
cargo build --release
cargo install --path .
```

### From Crates.io

```bash
cargo install ai-terminal
```

## Usage

```bash
# Single command
ai-terminal "restart nginx and check the error logs"

# Interactive mode
ai-terminal --interactive

# Dry run (preview only)
ai-terminal --dry-run "delete old logs"

# Use specific AI model
ai-terminal --model claude-sonnet "find large files"
```

## Examples

```
$ ai-terminal "show me the largest files in /var/log"
→ Suggested commands:
   1. sudo du -sh /var/log/* | sort -rh | head -10
   2. sudo find /var/log -type f -exec ls -lh {} \; | sort -k5 -rh | head -10

$ ai-terminal "help me set up nginx"
→ Suggested commands:
   1. sudo apt update && sudo apt install nginx
   2. sudo nano /etc/nginx/sites-available/default
   3. sudo systemctl restart nginx
```

## Privacy

Your commands are sent to Anthropic's API by default. Use `--local` for local model mode (coming soon).

## Tech Stack

- **Language:** Rust
- **AI:** Anthropic Claude API
- **Runtime:** Tokio async

## License

MIT © Scott Hardie
