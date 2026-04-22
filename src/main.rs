use anyhow::Result;
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
    }
}
