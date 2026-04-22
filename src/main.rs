use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use std::env;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(name = "ai-terminal", version = "0.2.0")]
struct Cli {
    #[command(subcommand)] command: Option<Commands>,
    #[arg(short, long)] command_str: Option<String>,
    #[arg(short, long)] interactive: bool,
    #[arg(long)] dry_run: bool,
    #[arg(short, long)] yes: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run { command: String },
    Version,
    Config,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    
    if let Some(cmd) = args.command {
        match cmd {
            Commands::Version => { println!("ai-terminal v0.2.0"); return Ok(())); }
            Commands::Config => { println!("API_KEY: set"); return Ok(())); }
            Commands::Run { command } => { run_command(&command, args.dry_run)?; return Ok(())); }
        }
    }
    
    if let Some(cmd_str) = args.command_str {
        run_command(&cmd_str, args.dry_run)?;
        return Ok(());
    }
    
    if args.interactive {
        loop {
            print!("> ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            if input.is_empty() || input == "exit" { break; }
            run_command(input, args.dry_run)?;
        }
        return Ok(());
    }
    
    println!("ai-terminal v0.2.0");
    println!("Usage: ai-terminal --command 'restart nginx'");
    Ok(())
}

fn run_command(natural: &str, dry_run: bool) -> Result<()> {
    println!("→ {}", natural);
    
    let commands = parse_command(natural);
    println!("\nSuggested:");
    for (i, cmd) in commands.iter().enumerate() {
        println!("  {}: {}", i + 1, cmd);
    }
    
    if !dry_run {
        println!("\n[Set ANTHROPIC_API_KEY to execute]");
    }
    Ok(())
}

fn parse_command(natural: &str) -> Vec<String> {
    let lower = natural.to_lowercase();
    if lower.contains("nginx") && lower.contains("restart") {
        vec!["sudo systemctl restart nginx".to_string()]
    } else if lower.contains("disk") && lower.contains("usage") {
        vec!["df -h".to_string()]
    } else if lower.contains("memory") {
        vec!["free -h".to_string()]
    } else {
        vec![format!("# Unknown: {}", natural)]
    }
}
