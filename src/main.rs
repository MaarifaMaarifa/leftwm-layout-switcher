use std::error;
use std::process::Command;

use clap::{Parser, Subcommand};
use notify_rust::Notification;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    layout: LayoutChangeMessage,
}

#[derive(Debug, Subcommand)]
enum LayoutChangeMessage {
    /// Change to the Next layout
    NextLayout,

    /// Change to the Previous layout
    PreviousLayout,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    change_layout(cli.layout)?;

    Notification::new()
        .summary("Leftwm Layout")
        .body(&format!("Current Layout set to {}", leftwm_state::get_layout_name()?))
        .show()?;

    Ok(())
}

fn change_layout(layout_message: LayoutChangeMessage) -> Result<(), Box<dyn error::Error>> {
    match layout_message {
        LayoutChangeMessage::NextLayout => {
            Command::new("leftwm-command").arg("NextLayout").status()?;
        }
        LayoutChangeMessage::PreviousLayout => {
            Command::new("leftwm-command")
                .arg("PreviousLayout")
                .status()?;
        }
    }
    Ok(())
}

mod leftwm_state {
    use serde::Deserialize;
    use std::process;
    use std::error;

    #[derive(Debug, Deserialize)]
    struct LeftwmState {
        workspaces: Vec<LeftwmLayout>,
    }

    #[derive(Debug, Deserialize)]
    struct LeftwmLayout {
        layout: String
    }


    pub fn get_layout_name() -> Result<String, Box<dyn error::Error>> {
        let leftwm_state = match get_leftwm_state_json()? {
            Some(state_string) => state_string,
            None => {
                eprintln!("Error: Could not get the leftwm state");
                process::exit(1);
            }
        };

        let leftwm_state: LeftwmState = serde_json::from_str(&leftwm_state)?;

        let layout_name = match leftwm_state.workspaces.get(0) {
            Some(layout) => layout.layout.to_string(),
            None => "None".to_string()
        };

        Ok(layout_name)
    }

    fn get_leftwm_state_json() -> Result<Option<String>, Box<dyn error::Error>>{
        let output = process::Command::new("leftwm-state").arg("-q").output()?;

        if output.status.success() {
            return Ok(Some(String::from_utf8_lossy(&output.stdout).to_string()))
        }
        Ok(None)
    }
}