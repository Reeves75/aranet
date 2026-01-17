# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

_No unreleased changes._

## [0.1.2] - 2026-01-16

### Added

- **Full AranetRn+ (Radon) sensor support**
  - Current readings: radon (Bq/m³), temperature, pressure, humidity, battery, interval, age
  - History download: 4-byte radon values (param 10), humidity in tenths (param 5)
  - `HistoryRecord.radon` field for radon history data
  - `HistoryParam::Radon` and `HistoryParam::Humidity2` enum variants
- Complete BLE communication stack with btleplug 0.11
  - Device scanning and discovery
  - Connection management with auto-reconnection
  - Current readings for all device types
  - History download (V1 notification-based, V2 read-based)
  - Device settings read/write (interval, Bluetooth range, Smart Home)
- Multi-device support (Aranet4, Aranet2, AranetRn+, Aranet Radiation)
- Core types (CurrentReading, DeviceInfo, HistoryRecord, Status, DeviceType)
- BLE UUIDs for Aranet devices (both old and new firmware)
- **CLI fully implemented** with all core commands:
  - `scan` — Discover nearby Aranet devices
  - `read` — Read current sensor measurements
  - `status` — Quick one-line reading with colored CO₂ status
  - `info` — Display device information
  - `history` — Download historical data (text, JSON, CSV)
  - `set` — Modify device settings (interval, range, smart_home)
  - `watch` — Continuous monitoring with auto-reconnect
  - `config` — Manage configuration file (`~/.config/aranet/config.toml`)
  - `completions` — Generate shell completions (bash, zsh, fish, PowerShell)
- TUI app shell with ratatui
- GUI app shell with egui/eframe
- WASM module scaffolding
- Error types with thiserror
- Real-time streaming with `ReadingStream`
- Event system with `EventDispatcher`
- Connection metrics and operation tracking
- Mock device for testing
- Data validation and CO2 threshold helpers
- JSON and CSV output formats for all CLI commands
- Colored CO₂ status indicators (green/amber/red)
- Config file support with device, format, no_color, fahrenheit options
- `ARANET_DEVICE` environment variable support
- `--no-color` flag and `NO_COLOR` env var support

### Fixed

- Corrected UUID mappings for history characteristics (V1 → 2003, V2 → 2005)
- Fixed V2 history response parsing (10-byte header format)
- Resolved async deadlock in device connection
- Increased connection timeout for reliable BLE connections
- Corrected AranetRn+ GATT data format parsing (device_type, interval, age fields)

### Changed

- Updated to Rust 1.90 minimum (edition 2024)
- History download now uses correct parameter values for each sensor type
