//! History command implementation.

use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};

use crate::cli::OutputFormat;
use crate::format::{format_history_csv, format_history_json, format_history_text, FormatOptions};
use crate::util::{connect_device, require_device, write_output};

pub async fn cmd_history(
    device: Option<String>,
    count: u32,
    timeout: Duration,
    format: OutputFormat,
    output: Option<&PathBuf>,
    quiet: bool,
    opts: &FormatOptions,
) -> Result<()> {
    let identifier = require_device(device)?;

    if !quiet && matches!(format, OutputFormat::Text) {
        eprintln!("Connecting to {}...", identifier);
    }

    let device = connect_device(&identifier, timeout).await?;

    if !quiet && matches!(format, OutputFormat::Text) {
        eprintln!("Downloading history...");
    }

    let history = device
        .download_history()
        .await
        .context("Failed to download history")?;

    device.disconnect().await.ok();

    // Apply count limit if specified (0 means all)
    let history: Vec<_> = if count > 0 {
        history.into_iter().take(count as usize).collect()
    } else {
        history
    };

    if !quiet && matches!(format, OutputFormat::Text) {
        eprintln!("Downloaded {} records.", history.len());
    }

    let content = match format {
        OutputFormat::Json => format_history_json(&history, opts)?,
        OutputFormat::Text => format_history_text(&history, opts),
        OutputFormat::Csv => format_history_csv(&history, opts),
    };

    write_output(output, &content)?;
    Ok(())
}

