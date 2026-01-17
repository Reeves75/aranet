<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/cameronrye/aranet/main/assets/aranet-logo-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/cameronrye/aranet/main/assets/aranet-logo-light.svg">
    <img alt="Aranet" src="https://raw.githubusercontent.com/cameronrye/aranet/main/assets/aranet-logo-light.svg" height="60">
  </picture>
</p>

# aranet-cli

Command-line interface for Aranet environmental sensors.

A fast, scriptable CLI for reading sensor data, downloading history, and configuring Aranet devices (Aranet4, Aranet2, AranetRn+, Aranet Radiation).

## Installation

```bash
cargo install aranet-cli
```

Or build from source:

```bash
git clone https://github.com/cameronrye/aranet.git
cd aranet
cargo build --release --package aranet-cli
```

## Usage

### Scan for devices

```bash
aranet scan
```

### Read current measurements

```bash
aranet read --device <DEVICE_ADDRESS>
```

### Download measurement history

```bash
aranet history --device <DEVICE_ADDRESS>
aranet history --device <DEVICE_ADDRESS> --count 100 --format csv --output history.csv
```

### Watch real-time data

```bash
aranet watch --device <DEVICE_ADDRESS> --interval 60
```

### View device information

```bash
aranet info --device <DEVICE_ADDRESS>
```

### Configure device settings

```bash
aranet set --device <DEVICE_ADDRESS> interval 5
aranet set --device <DEVICE_ADDRESS> range extended
```

## Configuration

The CLI supports persistent configuration via a TOML file:

```bash
# Initialize config file
aranet config init

# Set a default device
aranet config set device <DEVICE_ADDRESS>

# Set default output format
aranet config set format json

# Show current config
aranet config show
```

Configuration options:
- `device` — Default device address
- `format` — Default output format (`text`, `json`, `csv`)
- `timeout` — Connection timeout in seconds
- `no_color` — Disable colored output
- `fahrenheit` — Use Fahrenheit for temperature display

## Output Formats

| Format | Description |
|--------|-------------|
| `text` | Human-readable colored output (default) |
| `json` | JSON for scripting and APIs |
| `csv` | CSV for spreadsheets and data analysis |

```bash
aranet read --device <DEVICE> --format json
aranet read --device <DEVICE> --json    # shorthand
```

## Shell Completions

Generate shell completions for your preferred shell:

```bash
aranet completions bash > ~/.local/share/bash-completion/completions/aranet
aranet completions zsh > ~/.zfunc/_aranet
aranet completions fish > ~/.config/fish/completions/aranet.fish
```

## Related Crates

This CLI is part of the [aranet](https://github.com/cameronrye/aranet) workspace:

| Crate | crates.io | Description |
|-------|-----------|-------------|
| [aranet-core](../aranet-core/) | [![crates.io](https://img.shields.io/crates/v/aranet-core.svg)](https://crates.io/crates/aranet-core) | Core BLE library for device communication |
| [aranet-types](../aranet-types/) | [![crates.io](https://img.shields.io/crates/v/aranet-types.svg)](https://crates.io/crates/aranet-types) | Shared types for sensor data |
| [aranet-tui](../aranet-tui/) | [![crates.io](https://img.shields.io/crates/v/aranet-tui.svg)](https://crates.io/crates/aranet-tui) | Terminal UI dashboard |
| [aranet-gui](../aranet-gui/) | [![crates.io](https://img.shields.io/crates/v/aranet-gui.svg)](https://crates.io/crates/aranet-gui) | Desktop application (egui) |

## License

MIT

---

Made with ❤️ by [Cameron Rye](https://rye.dev/)

