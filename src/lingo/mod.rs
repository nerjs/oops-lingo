use dioxus::prelude::*;
use dioxus_logger::tracing::error;
use futures_util::StreamExt;
use state::{StateInfo, UpdateSignal};
use ui::{ui_container, ui_page};
use worker::{handle_action, Action};

mod api;
mod state;
mod ui;
mod worker;

pub fn lingo_container() -> Element {
    let state_signal = use_context_provider(|| Signal::new(StateInfo::default()));
    use_coroutine(move |mut rx: UnboundedReceiver<Action>| async move {
        let state_signal = state_signal.clone();
        while let Some(action) = rx.next().await {
            if let Err(error) = handle_action(state_signal, action).await {
                error!("handle error: {:?}", error);

                state_signal
                    .update()
                    .with_loading(false)
                    .with_error(error.to_string())
                    .update();
            }
        }
    });

    rsx! {
        ui_container { state: state_signal,
            ui_page { state: state_signal.read().clone() }
        }
    }
}
