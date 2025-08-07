#![doc = "Minimal subset of the AEI Framework for CLI integration."]

/// Returns the version of the AEI framework.
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
