use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    Err(anyhow!("Missing attribute: {}", 12))
    // let path = "/home/daniele/.bash_history";
    //
    // let content =
    //     std::fs::read(path).with_context(|| format!("Failed to read instrs from {}", path))?;
    // Ok(())
}
