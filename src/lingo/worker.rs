use std::time::Duration;

use dioxus::prelude::*;
use dioxus_logger::tracing::debug;
use tokio::time::sleep;

use super::result::Error;

use super::{result::Result, state::StateInfo};

#[derive(Debug)]
pub enum Action {
    UserInput(String),
}

async fn handle_user_input(text: String, state: &mut Signal<StateInfo>) -> Result<()> {
    sleep(Duration::from_secs(3)).await;
    Err(Error::EmptyInput)
}

pub async fn handle_action(mut state: Signal<StateInfo>, action: Action) -> Result<()> {
    debug!("handle action: {action:?}");

    match action {
        Action::UserInput(text) => handle_user_input(text, &mut state).await?,
    };

    Ok(())
}
