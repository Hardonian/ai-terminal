# Architecture

## CLI Parsing

```
Input → Clap → Command Handler → Claude API → Output
                                      ↓
                              Shell Executor
```

## Key Modules

### main.rs
- Entry point
- Argument parsing
- Mode selection (command/interactive)

### parser.rs
- Natural language understanding
- Command mapping
- Safety validation

### executor.rs
- Shell command execution
- Permission handling
- Output formatting
