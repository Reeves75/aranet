//! Scan command implementation.

use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use aranet_core::{ScanOptions, scan};

use crate::cli::OutputFormat;
use crate::format::{FormatOptions, format_scan_csv, format_scan_json, format_scan_text};
use crate::style;
use crate::util::write_output;

pub async fn cmd_scan(
    timeout: u64,
    format: OutputFormat,
    output: Option<&PathBuf>,
    quiet: bool,
    opts: &FormatOptions,
) -> Result<()> {
    // Show spinner for text output (unless quiet)
    let spinner = if !quiet && matches!(format, OutputFormat::Text) {
        Some(style::scanning_spinner(timeout))
    } else {
        None
    };

    let options = ScanOptions {
        duration: Duration::from_secs(timeout),
        filter_aranet_only: true,
    };

    let devices = scan::scan_with_options(options)
        .await
        .context("Failed to scan for devices")?;

    // Clear spinner before output
    if let Some(sp) = spinner {
        sp.finish_and_clear();
    }

    let content = match format {
        OutputFormat::Json => format_scan_json(&devices, opts)?,
        OutputFormat::Text => format_scan_text(&devices, opts),
        OutputFormat::Csv => format_scan_csv(&devices, opts),
    };

    write_output(output, &content)?;
    Ok(())
}
