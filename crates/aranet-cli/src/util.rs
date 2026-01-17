//! Utility functions for CLI operations.

use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use aranet_core::Device;

/// Get device identifier, with helpful error message
pub fn require_device(device: Option<String>) -> Result<String> {
    device.ok_or_else(|| {
        anyhow::anyhow!(
            "No device specified. Use --device <ADDRESS> or set ARANET_DEVICE environment variable.\n\
             Run 'aranet scan' to find nearby devices."
        )
    })
}

/// Connect to a device with timeout and improved error messages.
pub async fn connect_device(identifier: &str, timeout: Duration) -> Result<Device> {
    Device::connect_with_timeout(identifier, timeout)
        .await
        .map_err(|e| {
            let base_msg = format!("Failed to connect to device: {}", identifier);
            let suggestion = "\n\nPossible causes:\n  \
                • Bluetooth may be disabled — check system settings\n  \
                • Device may be out of range — try moving closer\n  \
                • Device may be connected to another host\n  \
                • Device address may be incorrect — run 'aranet scan' to verify";
            anyhow::anyhow!("{}\n\nCause: {}{}", base_msg, e, suggestion)
        })
}

/// Write output to file or stdout
pub fn write_output(output: Option<&PathBuf>, content: &str) -> Result<()> {
    match output {
        Some(path) => {
            std::fs::write(path, content)
                .with_context(|| format!("Failed to write to {}", path.display()))?;
        }
        None => {
            print!("{}", content);
            io::stdout().flush()?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_device_with_some() {
        let result = require_device(Some("AA:BB:CC:DD:EE:FF".to_string()));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "AA:BB:CC:DD:EE:FF");
    }

    #[test]
    fn test_require_device_with_none() {
        let result = require_device(None);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("No device specified"));
        assert!(err.contains("ARANET_DEVICE"));
    }
}
