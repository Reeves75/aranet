//! Alias command implementation.
//!
//! Manages friendly device names (aliases) that map to device addresses.

use anyhow::{Result, bail};

use crate::config::Config;

/// Alias subcommand actions
pub enum AliasAction {
    /// List all aliases
    List,
    /// Set an alias
    Set { name: String, address: String },
    /// Remove an alias
    Remove { name: String },
}

pub fn cmd_alias(action: AliasAction, quiet: bool) -> Result<()> {
    let mut config = Config::load();

    match action {
        AliasAction::List => {
            if config.aliases.is_empty() {
                if !quiet {
                    println!("No aliases configured.");
                    println!();
                    println!("Add an alias with: aranet alias set <name> <address>");
                }
            } else {
                println!("Device Aliases:");
                println!("────────────────────────────────────");
                let mut aliases: Vec<_> = config.aliases.iter().collect();
                aliases.sort_by_key(|(name, _)| name.as_str());
                for (name, address) in aliases {
                    println!("  {} → {}", name, address);
                }
            }
        }
        AliasAction::Set { name, address } => {
            // Validate the name doesn't look like a MAC address
            if looks_like_address(&name) {
                bail!(
                    "Alias name '{}' looks like a device address. \
                     Use a friendly name instead (e.g., 'living-room', 'office').",
                    name
                );
            }

            let was_update = config.aliases.contains_key(&name);
            config.aliases.insert(name.clone(), address.clone());
            config.save()?;

            if !quiet {
                if was_update {
                    println!("Updated alias '{}' → {}", name, address);
                } else {
                    println!("Added alias '{}' → {}", name, address);
                }
            }
        }
        AliasAction::Remove { name } => {
            if config.aliases.remove(&name).is_some() {
                config.save()?;
                if !quiet {
                    println!("Removed alias '{}'", name);
                }
            } else {
                bail!("Alias '{}' not found", name);
            }
        }
    }

    Ok(())
}

/// Check if a string looks like a device address (MAC or UUID).
fn looks_like_address(s: &str) -> bool {
    // MAC address pattern: XX:XX:XX:XX:XX:XX or XX-XX-XX-XX-XX-XX
    let mac_pattern = s.chars().filter(|c| *c == ':' || *c == '-').count() >= 5
        && s.chars()
            .all(|c| c.is_ascii_hexdigit() || c == ':' || c == '-');

    // UUID pattern: contains mostly hex and dashes, 32+ chars
    let uuid_pattern = s.len() >= 32 && s.chars().all(|c| c.is_ascii_hexdigit() || c == '-');

    mac_pattern || uuid_pattern
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_looks_like_address_mac() {
        assert!(looks_like_address("AA:BB:CC:DD:EE:FF"));
        assert!(looks_like_address("aa:bb:cc:dd:ee:ff"));
        assert!(looks_like_address("AA-BB-CC-DD-EE-FF"));
    }

    #[test]
    fn test_looks_like_address_uuid() {
        assert!(looks_like_address("12345678-1234-1234-1234-123456789abc"));
        assert!(looks_like_address("12345678123412341234123456789abc"));
    }

    #[test]
    fn test_looks_like_address_friendly_names() {
        assert!(!looks_like_address("living-room"));
        assert!(!looks_like_address("office"));
        assert!(!looks_like_address("bedroom-sensor"));
        assert!(!looks_like_address("Aranet4"));
    }
}
