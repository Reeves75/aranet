//! Status command implementation.

use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use serde::Serialize;

use crate::cli::OutputFormat;
use crate::format::{bq_to_pci, csv_escape, format_status, FormatOptions};
use crate::util::{connect_device, require_device, write_output};

pub async fn cmd_status(
    device: Option<String>,
    timeout: Duration,
    format: OutputFormat,
    output: Option<&PathBuf>,
    opts: &FormatOptions,
) -> Result<()> {
    let identifier = require_device(device)?;

    let device = connect_device(&identifier, timeout).await?;

    let name = device.name().map(|s| s.to_string());
    let reading = device
        .read_current()
        .await
        .context("Failed to read current values")?;

    device.disconnect().await.ok();

    let device_name = name.clone().unwrap_or_else(|| identifier.clone());

    let content = match format {
        OutputFormat::Json => format_status_json(&device_name, &reading, opts)?,
        OutputFormat::Csv => format_status_csv(&device_name, &reading, opts),
        OutputFormat::Text => format_status_text(&device_name, &reading, opts),
    };

    write_output(output, &content)?;
    Ok(())
}

/// Format status as one-line text output
fn format_status_text(
    device_name: &str,
    reading: &aranet_types::CurrentReading,
    opts: &FormatOptions,
) -> String {
    let status_str = format_status(reading.status, opts.no_color);
    let temp = opts.format_temp(reading.temperature);

    if reading.co2 > 0 {
        // Aranet4
        format!(
            "{}: {} ppm {} {} {}% {:.1}hPa\n",
            device_name,
            reading.co2,
            status_str,
            temp,
            reading.humidity,
            reading.pressure
        )
    } else if let Some(radon) = reading.radon {
        // AranetRn+
        format!(
            "{}: {} {} {} {}% {:.1}hPa\n",
            device_name, opts.format_radon(radon), status_str, temp, reading.humidity, reading.pressure
        )
    } else if let Some(rate) = reading.radiation_rate {
        // Aranet Radiation
        format!("{}: {:.3} µSv/h\n", device_name, rate)
    } else {
        // Aranet2 or unknown
        format!(
            "{}: {} {}%\n",
            device_name, temp, reading.humidity
        )
    }
}

/// Format status as JSON output
fn format_status_json(
    device_name: &str,
    reading: &aranet_types::CurrentReading,
    opts: &FormatOptions,
) -> Result<String> {
    #[derive(Serialize)]
    struct StatusJson<'a> {
        device: &'a str,
        co2: u16,
        temperature: f32,
        temperature_unit: &'static str,
        humidity: u8,
        pressure: f32,
        battery: u8,
        status: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        radon_bq: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        radon_pci: Option<f32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        radiation_rate: Option<f32>,
    }

    let json = StatusJson {
        device: device_name,
        co2: reading.co2,
        temperature: opts.convert_temp(reading.temperature),
        temperature_unit: if opts.fahrenheit { "F" } else { "C" },
        humidity: reading.humidity,
        pressure: reading.pressure,
        battery: reading.battery,
        status: format!("{:?}", reading.status),
        radon_bq: reading.radon,
        radon_pci: reading.radon.map(bq_to_pci),
        radiation_rate: reading.radiation_rate,
    };

    opts.as_json(&json)
}

/// Format status as CSV output
fn format_status_csv(
    device_name: &str,
    reading: &aranet_types::CurrentReading,
    opts: &FormatOptions,
) -> String {
    let temp_header = if opts.fahrenheit { "temperature_f" } else { "temperature_c" };
    let radon_value = reading.radon.map(|r| format!("{:.2}", opts.convert_radon(r))).unwrap_or_default();
    if opts.no_header {
        format!(
            "{},{},{:.1},{},{:.1},{},{:?},{},{}\n",
            csv_escape(device_name),
            reading.co2,
            opts.convert_temp(reading.temperature),
            reading.humidity,
            reading.pressure,
            reading.battery,
            reading.status,
            radon_value,
            reading.radiation_rate.map(|r| format!("{:.3}", r)).unwrap_or_default()
        )
    } else {
        format!(
            "device,co2,{},humidity,pressure,battery,status,{},radiation_usvh\n\
             {},{},{:.1},{},{:.1},{},{:?},{},{}\n",
            temp_header,
            opts.radon_csv_header(),
            csv_escape(device_name),
            reading.co2,
            opts.convert_temp(reading.temperature),
            reading.humidity,
            reading.pressure,
            reading.battery,
            reading.status,
            radon_value,
            reading.radiation_rate.map(|r| format!("{:.3}", r)).unwrap_or_default()
        )
    }
}

