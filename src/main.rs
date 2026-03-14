use std::process::Command;
use std::path::Path;
use std::fs;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Trolley {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        project_name: String,
    },
    Build,
    Generate,
    Deploy,
    Test,
    #[command(name = "--help")]
    Help,
}

fn main() -> Result<()> {
    let cli = Trolley::try_parse()?;
    
    match cli.command {
        Commands::Init { project_name } => {
            println!("Initializing project: {}", project_name);
            let status = Command::new("anchor")
                .arg("init")
                .arg(project_name)
                .status()?;
            
            if !status.success() {
                anyhow::bail!("anchor init failed");
            } else {
                println!("anchor init succeeded");
            }
        }
        Commands::Build => {
            let status = Command::new("anchor")
                .arg("build")
                .spawn()?
                .wait()
                .with_context(|| "anchor build failed")?;
            
            if !status.success() {
                anyhow::bail!("anchor build failed");
            } else {
                println!("anchor build succeeded");
            }
        }
        Commands::Generate => {
            let status = Command::new("bun")
                .arg("run")
                .arg("generate")
                .spawn()?
                .wait()
                .with_context(|| "anchor generate failed")?;
            
            if !status.success() {
                anyhow::bail!("anchor generate failed");
            } else {
                println!("anchor generate succeeded");
            }
        }
        Commands::Deploy => {
            println!("deploying...");
            
            let target_deploy_dir = Path::new("target/deploy");
            if !target_deploy_dir.exists() {
                anyhow::bail!("target/deploy directory not found. Please run 'bunchor build' first.");
            }
            
            let mut so_file = None;
            for entry in fs::read_dir(target_deploy_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("so") {
                    so_file = Some(path);
                    break;
                }
            }

            so_file.ok_or_else(|| {
                anyhow::anyhow!(
                    "No .so file found in target/deploy. Please run 'bunchor build' first."
                )
            })?;
            
            let status = Command::new("anchor")
                .arg("deploy")
                .spawn()?
                .wait()
                .with_context(|| "anchor deploy failed")?;
            
            if !status.success() {
                anyhow::bail!("anchor deploy failed");
            } else {
                println!("anchor deploy succeeded");
            }
        }
        Commands::Test => {
            let status = Command::new("anchor")
                .arg("test")
                .spawn()?
                .wait()
                .with_context(|| "anchor test failed")?;
            
            if !status.success() {
                anyhow::bail!("anchor test failed");
            } else {
                println!("anchor test succeeded");
            }
        }
        Commands::Help => bunchor_help()?
    }
    
    Ok(())
}

fn bunchor_help() -> Result<()> {
    println!("Setup your anchor project with codama, bun and kit");
    println!("");
    println!("  bunchor init <project_name> - Initialize a new bunchor project");
    println!("  bunchor build               - Build the project");
    println!("  bunchor generate            - Generate codama client");
    println!("  bunchor deploy              - Deploy the project");
    println!("  bunchor test                - Test the project");
    
    Ok(())
}