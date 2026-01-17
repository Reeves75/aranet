//! Doctor command implementation.
//!
//! Performs BLE diagnostics and permission checks to help troubleshoot
//! connectivity issues.

use std::time::Duration;

use anyhow::Result;
use aranet_core::scan::{self, ScanOptions};

/// Check result with status and message.
struct Check {
    name: &'static str,
    passed: bool,
    message: String,
}

impl Check {
    fn pass(name: &'static str, message: impl Into<String>) -> Self {
        Self {
            name,
            passed: true,
            message: message.into(),
        }
    }

    fn fail(name: &'static str, message: impl Into<String>) -> Self {
        Self {
            name,
            passed: false,
            message: message.into(),
        }
    }
}

pub async fn cmd_doctor(verbose: bool) -> Result<()> {
    println!("Aranet Doctor - BLE Diagnostics\n");
    println!("Running diagnostics...\n");

    let mut checks: Vec<Check> = Vec::new();

    // Check 1: Bluetooth adapter availability
    let adapter_check = check_adapter().await;
    checks.push(adapter_check);

    // Check 2: Scan for devices (only if adapter is available)
    if checks.last().map(|c| c.passed).unwrap_or(false) {
        let scan_check = check_scan().await;
        checks.push(scan_check);
    }

    // Print results
    println!("Results:");
    println!("────────────────────────────────────");

    let mut all_passed = true;
    for check in &checks {
        let icon = if check.passed { "[PASS]" } else { "[FAIL]" };
        println!("{} {}: {}", icon, check.name, check.message);
        if !check.passed {
            all_passed = false;
        }
    }

    println!();

    // Print platform-specific help if there are failures
    if !all_passed {
        print_troubleshooting_help(verbose);
    } else {
        println!("All checks passed! Your system is ready to use Aranet devices.");
    }

    Ok(())
}

async fn check_adapter() -> Check {
    match scan::get_adapter().await {
        Ok(_adapter) => Check::pass("Bluetooth Adapter", "Found and accessible"),
        Err(e) => {
            let msg = format!("Not available ({})", e);
            Check::fail("Bluetooth Adapter", msg)
        }
    }
}

async fn check_scan() -> Check {
    let options = ScanOptions {
        duration: Duration::from_secs(3),
        filter_aranet_only: true,
    };

    match scan::scan_with_options(options).await {
        Ok(devices) => {
            if devices.is_empty() {
                Check::pass("BLE Scanning", "Works, but no Aranet devices found nearby")
            } else {
                let names: Vec<String> = devices.iter().filter_map(|d| d.name.clone()).collect();
                Check::pass(
                    "BLE Scanning",
                    format!("Found {} device(s): {}", devices.len(), names.join(", ")),
                )
            }
        }
        Err(e) => Check::fail("BLE Scanning", format!("Failed ({})", e)),
    }
}

fn print_troubleshooting_help(verbose: bool) {
    println!("Troubleshooting Tips:");
    println!();

    #[cfg(target_os = "macos")]
    {
        println!("macOS:");
        println!("  • Ensure Bluetooth is enabled in System Settings");
        println!("  • Grant Bluetooth permission to Terminal/your app");
        println!("  • Try: System Settings → Privacy & Security → Bluetooth");
        if verbose {
            println!("  • Check if other BLE apps work (e.g., LightBlue)");
            println!("  • Try resetting Bluetooth: sudo pkill bluetoothd");
        }
    }

    #[cfg(target_os = "linux")]
    {
        println!("Linux:");
        println!("  • Ensure BlueZ is installed: sudo apt install bluez");
        println!("  • Check Bluetooth service: systemctl status bluetooth");
        println!("  • Add user to bluetooth group: sudo usermod -aG bluetooth $USER");
        if verbose {
            println!("  • Check adapter: hciconfig -a");
            println!("  • Restart Bluetooth: sudo systemctl restart bluetooth");
        }
    }

    #[cfg(target_os = "windows")]
    {
        println!("Windows:");
        println!("  • Ensure Bluetooth is enabled in Settings");
        println!("  • Check Device Manager for Bluetooth adapter");
        println!("  • Update Bluetooth drivers if needed");
        if verbose {
            println!("  • Try: Settings → Bluetooth & devices → Bluetooth → On");
        }
    }

    println!();
}
