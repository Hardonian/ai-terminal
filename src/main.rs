use anyhow::Result;
<<<<<<< HEAD
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
=======
use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(name = "ai-terminal")]
#[command(version = "0.1.0")]
#[command(about = "Natural language terminal powered by AI")]
struct Args {
    /// Natural language command to execute
    #[arg(short, long)]
    command: Option<String>,
    
    /// Run in interactive mode
    #[arg(short, long)]
    interactive: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let args = Args::parse();
    
    if let Some(cmd) = args.command {
        run_command(&cmd).await?;
    } else if args.interactive {
        run_interactive().await?;
    } else {
        println!("AI Terminal v0.1.0");
        println!("Usage: ai-terminal --command 'restart nginx'");
        println!("       ai-terminal --interactive");
    }
    
    Ok(())
}

async fn run_command(cmd: &str) -> Result<()> {
    println!("Command: {}", cmd);
    println!("Analyzing...");
    
    // Parse natural language to shell commands
    let shell_commands = parse_command(cmd).await?;
    
    println!("\nSuggested commands:");
    for (i, command) in shell_commands.iter().enumerate() {
        println!("  {}. {}", i + 1, command);
    }
    
    // In v0.2: Execute with confirmation
    println!("\n[Full version will execute with --confirm]");
    
    Ok(())
}

async fn run_interactive() -> Result<()> {
    println!("AI Terminal Interactive Mode");
    println!("Type your intent in plain English. Type 'exit' to quit.\n");
    
    loop {
        print!("> ");
        std::io::Write::flush(&mut std::io::stdout())?;
        
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input)? == 0 {
            break;
        }
        
        let input = input.trim();
        if input.is_empty() || input == "exit" {
            break;
        }
        
        run_command(input).await?;
        println!();
    }
    
    Ok(())
}

async fn parse_command(natural: &str) -> Result<Vec<String>> {
    // In production, call Claude API
    // For now, return example commands
    let lower = natural.to_lowercase();
    
    if lower.contains("nginx") && lower.contains("restart") {
        Ok(vec![
            "sudo systemctl restart nginx".to_string(),
            "sudo systemctl status nginx".to_string(),
            "tail -20 /var/log/nginx/error.log".to_string()
        ])
    } else if lower.contains("disk") && lower.contains("usage") {
        Ok(vec![
            "df -h".to_string(),
            "du -sh * | sort -rh | head -10".to_string()
        ])
    } else if lower.contains("memory") && lower.contains("usage") {
        Ok(vec![
            "free -h".to_string(),
            "ps aux --sort=-%mem | head -10".to_string()
        ])
    } else {
        Ok(vec![format!("# Could not parse: {}", natural)])
>>>>>>> origin/main
    }
}
