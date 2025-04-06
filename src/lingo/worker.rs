use std::time::Duration;

use anyhow::Result;
use dioxus::prelude::*;
use dioxus_logger::tracing::debug;
use tokio::time::sleep;

use super::state::{LingoState, ProcessingState, StateInfo, UpdateSignal};

#[derive(Debug)]
pub enum Action {
    UserInput(String),
}

async fn handle_user_input(text: String, state: &mut Signal<StateInfo>) -> Result<()> {
    state.update().with_loading(true).without_error().update();

    sleep(Duration::from_secs(5)).await;

    state
        .update()
        .with_loading(false)
        .with_state(LingoState::Processing {
            user_input: text,
            processing_state: ProcessingState::InterpretationOptions(vec![
                "tratata".to_string(),
                "qwerty".to_string(),
            ]),
        })
        .update();

    Ok(())
}

pub async fn handle_action(mut state: Signal<StateInfo>, action: Action) -> Result<()> {
    debug!("handle action: {action:?}");

    match action {
        Action::UserInput(text) => handle_user_input(text, &mut state).await?,
    };

    Ok(())
}
