use aei_framework::get_version;

/// Run the `version` subcommand.
pub fn run() {
    let version = std::panic::catch_unwind(|| get_version());

    match version {
        Ok(v) => println!("AEI Framework version: {v}"),
        Err(_) => eprintln!("Failed to retrieve AEI Framework version."),
    }
}
