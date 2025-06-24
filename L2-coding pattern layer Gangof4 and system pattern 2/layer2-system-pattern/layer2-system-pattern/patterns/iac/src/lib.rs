//! Tiny wrapper to run Terraform from Rust.

use anyhow::{Context, Result};
use std::process::Command;

/// Execute `terraform init && terraform apply -auto-approve` in `dir`.
pub fn terraform_apply(dir: &str) -> Result<()> {
    run("terraform", &["-chdir", dir, "init"])?;
    run("terraform", &["-chdir", dir, "apply", "-auto-approve"])
}

fn run(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .with_context(|| format!("failed to execute {cmd}"))?;

    if !status.success() {
        return Err(anyhow::anyhow!("{cmd} exited with {status}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builds_cmdline() {
        // Just verify Command building succeeds on this platform
        assert!(run("echo", &["hi"]).is_ok());
    }
}
