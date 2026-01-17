//! Watch command implementation.
//!
//! Uses a persistent BLE connection to reduce overhead. The connection is only
//! re-established when a read fails, indicating the device has disconnected.
//! Implements exponential backoff for reconnection attempts to reduce resource usage.

use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use aranet_core::Device;

use crate::cli::OutputFormat;
use crate::format::{
    format_reading_json, format_watch_csv_header, format_watch_csv_line, format_watch_line,
    FormatOptions,
};
use crate::util::{require_device, write_output};

/// Minimum backoff delay for reconnection attempts
const MIN_BACKOFF_SECS: u64 = 2;
/// Maximum backoff delay for reconnection attempts
const MAX_BACKOFF_SECS: u64 = 300; // 5 minutes

pub async fn cmd_watch(
    device: Option<String>,
    interval: u64,
    count: u32,
    timeout: Duration,
    format: OutputFormat,
    output: Option<&PathBuf>,
    opts: &FormatOptions,
) -> Result<()> {
    let identifier = require_device(device)?;

    if count > 0 {
        eprintln!(
            "Watching {} (interval: {}s, count: {}) - Press Ctrl+C to stop",
            identifier, interval, count
        );
    } else {
        eprintln!(
            "Watching {} (interval: {}s) - Press Ctrl+C to stop",
            identifier, interval
        );
    }

    let mut header_written = opts.no_header;
    let mut current_device: Option<Device> = None;
    let mut readings_taken: u32 = 0;
    let mut backoff_secs = MIN_BACKOFF_SECS;

    loop {
        // Check if we've reached the count limit
        if count > 0 && readings_taken >= count {
            eprintln!("Completed {} readings.", readings_taken);
            if let Some(d) = current_device.take() {
                d.disconnect().await.ok();
            }
            return Ok(());
        }

        // Connect if we don't have a connection
        let device = match &current_device {
            Some(d) if d.is_connected().await => {
                // Reset backoff on successful connection
                backoff_secs = MIN_BACKOFF_SECS;
                d
            }
            _ => {
                // Need to connect (or reconnect)
                if current_device.is_some() {
                    eprintln!("Connection lost. Reconnecting...");
                }
                match Device::connect_with_timeout(&identifier, timeout).await {
                    Ok(d) => {
                        // Reset backoff on successful connection
                        backoff_secs = MIN_BACKOFF_SECS;
                        current_device = Some(d);
                        current_device.as_ref().unwrap()
                    }
                    Err(e) => {
                        eprintln!(
                            "Connection failed: {}. Retrying in {}s...",
                            e, backoff_secs
                        );
                        current_device = None;

                        // Wait with graceful shutdown support using exponential backoff
                        tokio::select! {
                            _ = tokio::signal::ctrl_c() => {
                                eprintln!("\nShutting down...");
                                return Ok(());
                            }
                            _ = tokio::time::sleep(Duration::from_secs(backoff_secs)) => {}
                        }

                        // Increase backoff for next attempt (exponential with cap)
                        backoff_secs = (backoff_secs * 2).min(MAX_BACKOFF_SECS);
                        continue;
                    }
                }
            }
        };

        // Read current values
        match device.read_current().await {
            Ok(reading) => {
                readings_taken += 1;
                let content = match format {
                    OutputFormat::Json => format_reading_json(&reading, opts)?,
                    OutputFormat::Csv => {
                        let mut out = String::new();
                        if !header_written {
                            out.push_str(&format_watch_csv_header(opts));
                            header_written = true;
                        }
                        out.push_str(&format_watch_csv_line(&reading, opts));
                        out
                    }
                    OutputFormat::Text => format_watch_line(&reading, opts),
                };
                write_output(output, &content)?;
            }
            Err(e) => {
                eprintln!("Read failed: {}. Will reconnect on next poll.", e);
                // Mark connection as lost so we reconnect on next iteration
                if let Some(d) = current_device.take() {
                    d.disconnect().await.ok();
                }
            }
        }

        // Check if we've reached the count limit after this reading
        if count > 0 && readings_taken >= count {
            continue; // Loop will exit at the top
        }

        // Wait for next interval with graceful shutdown support
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                eprintln!("\nShutting down...");
                // Clean up connection before exit
                if let Some(d) = current_device.take() {
                    d.disconnect().await.ok();
                }
                return Ok(());
            }
            _ = tokio::time::sleep(Duration::from_secs(interval)) => {}
        }
    }
}

