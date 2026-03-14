use std::fs;
use std::path::Path;
use std::{path::PathBuf, process::Command};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

mod consts;
use crate::consts::{
    COUNTER_DECREMENT, COUNTER_ERRORS, COUNTER_INCREMENT, COUNTER_INITIALIZE,
    COUNTER_INSTRUCTIONS_MOD, COUNTER_LIB, COUNTER_STATE_COUNTER, COUNTER_STATE_MOD, TEST_CONTENT,
};

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
        Commands::Help => bunchor_help()?,
    }

    Ok(())
}

fn bunchor_init(project_name: String) -> Result<()> {
    println!("Initializing project: {}", project_name);

    run_command(
        Command::new("anchor").arg("init").arg(&project_name),
        "anchor init",
        Some("Successfully initialized default anchor project"),
    )?;

    let project_path = Path::new(&project_name);
    let tests_path = project_path.join("tests");

    clean_project(project_path, &tests_path)?;
    write_anchor_program(project_path, &project_name)?;

    let test_file = tests_path.join(format!("{}.test.ts", &project_name));
    fs::write(&test_file, TEST_CONTENT).context("Failed to write test file")?;

    run_command(
        Command::new("bun").arg("install").current_dir(project_path),
        "bun install",
        Some("Successfully installed dependencies"),
    )?;

    Ok(())
}

fn bunchor_build() -> Result<()> {
    run_command(
        Command::new("anchor").arg("build"),
        "anchor build",
        Some("Successfully built anchor program"),
    )
}

fn bunchor_generate() -> Result<()> {
    run_command(
        Command::new("bun").arg("run").arg("generate"),
        "bun run generate",
        Some("Successfully generated codama client"),
    )
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
        anyhow::anyhow!("No .so file found in target/deploy. Please run 'bunchor build' first.")
    })?;

    run_command(
        Command::new("anchor").arg("deploy"),
        "anchor deploy",
        Some("Successfully deployed anchor program"),
    )
}

fn bunchor_test() -> Result<()> {
    run_command(
        Command::new("anchor").arg("test"),
        "anchor test",
        Some("Successfully tested anchor program"),
    )
}

fn bunchor_help() -> Result<()> {
    println!("Setup your anchor project with codama, bun and kit");
    println!("\n");
    println!("  bunchor init <project_name> - Initialize a new bunchor project");
    println!("  bunchor build               - Build the project");
    println!("  bunchor generate            - Generate codama client");
    println!("  bunchor deploy              - Deploy the project");
    println!("  bunchor test                - Test the project");

    Ok(())
}

fn run_command(cmd: &mut Command, cmd_str: &str, success_msg: Option<&str>) -> Result<()> {
    let status = cmd
        .status()
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

fn clean_project(project_path: &Path, tests_path: &PathBuf) -> Result<()> {
    let yarn_lock = project_path.join("yarn.lock");
    if yarn_lock.exists() {
        fs::remove_file(&yarn_lock).context("Failed to remove yarn.lock")?;
    }

    let migrations = project_path.join("migrations");
    if migrations.exists() {
        fs::remove_dir_all(&migrations).context("Failed to remove migrations")?;
    }

    let app_folder_path = project_path.join("app");
    if app_folder_path.exists() {
        fs::remove_dir_all(&app_folder_path).context("Failed to remove app dir")?;
    }

    if tests_path.exists() {
        fs::remove_dir_all(tests_path).context("Failed to remove tests")?;
    }
    fs::create_dir_all(tests_path).context("Failed to create tests dir")?;

    Ok(())
}

fn write_anchor_program(project_path: &Path, project_name: &str) -> Result<()> {
    let program_path = project_path.join("programs").join(project_name).join("src");
    let instructions_path = program_path.join("instructions");
    let state_path = program_path.join("state");

    if program_path.exists() {
        fs::remove_dir_all(&program_path).context("Failed to remove program src dir")?;
    }
    fs::create_dir(&program_path).context("Failed to create program dir")?;
    fs::create_dir(&state_path).context("Failed to create state dir")?;
    fs::create_dir(&instructions_path).context("Failed to create instructions dir")?;

    fs::write(program_path.join("lib.rs"), COUNTER_LIB)?;
    fs::write(program_path.join("errors.rs"), COUNTER_ERRORS)?;

    fs::write(state_path.join("mod.rs"), COUNTER_STATE_MOD)?;
    fs::write(state_path.join("counter.rs"), COUNTER_STATE_COUNTER)?;

    fs::write(instructions_path.join("mod.rs"), COUNTER_INSTRUCTIONS_MOD)?;
    fs::write(instructions_path.join("initialize.rs"), COUNTER_INITIALIZE)?;
    fs::write(instructions_path.join("increment.rs"), COUNTER_INCREMENT)?;
    fs::write(instructions_path.join("decrement.rs"), COUNTER_DECREMENT)?;

    Ok(())
}
