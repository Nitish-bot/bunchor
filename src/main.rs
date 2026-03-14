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
        Commands::Init { project_name } => bunchor_init(project_name)?,
        Commands::Build => bunchor_build()?,
        Commands::Generate => bunchor_generate()?,
        Commands::Deploy => bunchor_deploy()?,
        Commands::Test => bunchor_test()?,
        Commands::Help => bunchor_help()?
    }
    
    Ok(())
}

fn bunchor_init(project_name: String) -> Result<()> {
    println!("Initializing project: {}", project_name);
    
    run_command(Command::new("anchor").arg("init").arg(project_name), "anchor init", Some("Successfully initialized default anchor project"))?;
    
    // run_command(Command::new("rm").arg("yarn.lock"), "rm yarn.lock", None)?;
    // run_command(Command::new("bun").arg("install"), "bun install", Some("Successfully installed dependencies"))?;
    
    Ok(())
}

fn bunchor_build() -> Result<()> {
    run_command(Command::new("anchor").arg("build"), "anchor build", Some("Successfully built anchor program"))
}

fn bunchor_generate() -> Result<()> {
    run_command(Command::new("bun").arg("run").arg("generate"), "bun run generate", Some("Successfully generated codama client"))
}

fn bunchor_deploy() -> Result<()> {
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
    
    run_command(Command::new("anchor").arg("deploy"), "anchor deploy", Some("Successfully deployed anchor program"))
}

fn bunchor_test() -> Result<()> {
    run_command(Command::new("anchor").arg("test"), "anchor test", Some("Successfully tested anchor program"))
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

fn run_command(cmd: &mut Command, cmd_str: &str, success_msg: Option<&str>) -> Result<()> {
    let status = cmd.status()
        .with_context(|| format!("Failed to execute process: {}", cmd_str))?;

    if status.success() {
        if let Some(success_msg) = success_msg {
            println!("{}", success_msg);
        }
        Ok(())
    } else {
        // The child process already printed its error to stderr.
        // We exit with its code so the user doesn't get a redundant 
        // "anyhow" error message from our wrapper.
        std::process::exit(status.code().unwrap_or(1));
    }
}