# Command Reference

## Single Command Mode

```bash
ai-terminal [OPTIONS] --command "your intent"
```

| Option | Description |
|--------|-------------|
| `--command` | Natural language command |
| `--dry-run` | Preview without executing |
| `--model` | AI model to use |
| `--confirm` | Skip confirmation prompt |

## Interactive Mode

```bash
ai-terminal --interactive
```

Enter commands one at a time. Type `exit` to quit.

## Environment Variables

| Variable | Description |
|----------|-------------|
| `ANTHROPIC_API_KEY` | Anthropic API key |
| `AI_TERMINAL_MODEL` | Default model |
| `AI_TERMINAL_CONFIRM` | Auto-confirm (true/false) |
