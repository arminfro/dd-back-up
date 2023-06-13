use std::process::{Command, Output};

/// Executes a command and captures its output.
///
/// # Arguments
///
/// * `command_parts` - The parts of the command.
/// * `description` - The description of the command.
/// * `use_sudo` - Indicates whether sudo should be used for the command (if available).
pub fn command_output(
    command_parts: Vec<&str>,
    description: &str,
    is_sudo_needed: Option<bool>,
) -> Result<Output, String> {
    let command_parts = {
        if is_sudo_needed.unwrap_or(false) {
            append_sudo_if_available(command_parts, Some(&description))
        } else {
            command_parts
        }
    };

    Command::new(&command_parts[0])
        .args(&command_parts[1..])
        .output()
        .map_err(|e| format!("Failed to {}: {}", description, e.to_string()))
}

fn append_sudo_if_available<'a>(
    command_parts: Vec<&'a str>,
    description: Option<&str>,
) -> Vec<&'a str> {
    let mut updated_command_parts = Vec::new();

    if is_sudo_available() {
        updated_command_parts.push("sudo");
        let sudo_message = "Sudo is needed";
        match description {
            Some(description) => println!("{} to {}", sudo_message, description),
            None => println!("{}", sudo_message),
        };
    }

    updated_command_parts.extend_from_slice(command_parts.as_slice());
    updated_command_parts
}

fn is_sudo_available() -> bool {
    Command::new("sudo").arg("--version").output().is_ok()
}
