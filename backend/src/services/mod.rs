use serde::{Deserialize, Serialize};

pub mod report;
pub mod scanner;
pub mod state;

// TODO: fix error
pub fn init_scanners() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: fix init scanners (init folders for scan results)
    Ok(())
}