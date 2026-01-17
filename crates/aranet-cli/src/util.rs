//! Utility functions for CLI operations.

use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use aranet_core::{Device, ScanOptions, scan};
use dialoguer::{Select, theme::ColorfulTheme};

/// Get device identifier, with helpful error message.
/// Used for non-interactive contexts (e.g., scripts, piped input).
#[allow(dead_code)]
pub fn require_device(device: Option<String>) -> Result<String> {
    device.ok_or_else(|| {
        anyhow::anyhow!(
            "No device specified. Use --device <ADDRESS> or set ARANET_DEVICE environment variable.\n\
             Run 'aranet scan' to find nearby devices, or omit --device for interactive selection."
        )
    })
}

/// Get device identifier, scanning and prompting interactively if none specified.
pub async fn require_device_interactive(device: Option<String>) -> Result<String> {
    if let Some(dev) = device {
        return Ok(dev);
    }

    // Check if we're in an interactive terminal
    if !io::stdin().is_terminal() || !io::stderr().is_terminal() {
        bail!(
            "No device specified. Use --device <ADDRESS> or set ARANET_DEVICE environment variable.\n\
             Run 'aranet scan' to find nearby devices."
        );
    }

    eprintln!("No device specified. Scanning for nearby devices...");

    let options = ScanOptions {
        duration: Duration::from_secs(5),
        filter_aranet_only: true,
    };

    let devices = scan::scan_with_options(options)
        .await
        .context("Failed to scan for devices")?;

    if devices.is_empty() {
        bail!(
            "No Aranet devices found nearby.\n\
             Make sure your device is powered on and in range."
        );
    }

    if devices.len() == 1 {
        let dev = &devices[0];
        let name = dev.name.as_deref().unwrap_or("Unknown");
        eprintln!("Found 1 device: {} ({})", name, dev.identifier);
        return Ok(dev.identifier.clone());
    }

    // Build selection items
    let items: Vec<String> = devices
        .iter()
        .map(|d| {
            let name = d.name.as_deref().unwrap_or("Unknown");
            format!("{} ({})", name, d.identifier)
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a device")
        .items(&items)
        .default(0)
        .interact()
        .context("Failed to get user selection")?;

    Ok(devices[selection].identifier.clone())
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
