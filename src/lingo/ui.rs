use dioxus::prelude::*;

use crate::{
    lingo::state::LingoState,
    ui::{
        card::{Card, CardFooter, CardGroup, CardGroupHeader},
        input::CardInput,
    },
};

use super::{state::StateInfo, worker::Action};

#[component]
fn error_alert(text: String) -> Element {
    rsx! {
        div { class: "w-full h-full to-red-50 text-red-950 font-bold overflow-auto",

            "{text}"
        }
    }
}

#[component]
fn loading_indicator() -> Element {
    rsx! {
        div { class: "static text-sm t-0 r-0 l-0 px-2 py-5", "Loading..." }
    }
}

#[component]
pub fn ui_container(state: Signal<StateInfo>, children: Element) -> Element {
    let state = state.read();

    rsx! {
        div { class: "w-full h-full overflow-hidden relative",
            {children}

            if let Some(eror_text) = &state.error {
                error_alert { text: eror_text }
            }

            if state.loading {
                loading_indicator {}
            }
        }
    }
}

fn user_input(loading: bool) -> Element {
    let tx = use_coroutine_handle::<Action>();
    let input_signal = use_signal(|| String::new());
    let mut disabled_submit = use_signal(|| false);

    use_effect(move || {
        let text_length = input_signal.read().clone().trim().len();
        if text_length == 0 {
            disabled_submit.set(true);
        } else {
            disabled_submit.set(false);
        }
    });

    rsx! {
        CardGroup {
            CardGroupHeader { title: "Input text" }

            Card {
                CardInput {
                    input: input_signal,
                    placeholder: "Type in the text here",
                    disabled: loading,
                    disabled_submit: *disabled_submit.read(),
                    onsubmit: move |_| tx.send(Action::UserInput(input_signal.read().clone())),
                }

                if loading {
                    CardFooter { "Loading..." }
                }
            
            }
        
        }
    }
}

#[component]
pub fn ui_page(state: StateInfo) -> Element {
    match &state.state {
        LingoState::UserInput => user_input(state.loading),
        LingoState::Processing {
            user_input,
            processing_state,
        } => todo!(),
    }
}
