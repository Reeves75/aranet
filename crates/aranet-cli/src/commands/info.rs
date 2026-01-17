//! Info command implementation.

use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};

use crate::cli::OutputFormat;
use crate::format::{FormatOptions, format_info_csv, format_info_text};
use crate::util::{connect_device, require_device, write_output};

pub async fn cmd_info(
    device: Option<String>,
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

    let info = device
        .read_device_info()
        .await
        .context("Failed to read device info")?;

    device.disconnect().await.ok();

    let content = match format {
        OutputFormat::Json => opts.as_json(&info)?,
        OutputFormat::Text => format_info_text(&info, opts),
        OutputFormat::Csv => format_info_csv(&info, opts),
    };

    write_output(output, &content)?;
    Ok(())
}
