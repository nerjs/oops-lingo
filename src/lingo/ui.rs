use dioxus::prelude::*;

use crate::lingo::state::LingoState;

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

fn user_input() -> Element {
    let tx = use_coroutine_handle::<Action>();
    let mut input_signal = use_signal(|| String::new());

    rsx! {
        div { class: "w-full flex flex-col p-3 gap-1 items-center",

            textarea {
                value: "{input_signal}",
                oninput: move |e| input_signal.set(e.value()),
                onkeypress: move |e| {
                    if e.code() == Code::Enter && e.modifiers() == Modifiers::CONTROL {
                        e.prevent_default();
                        tx.send(Action::UserInput(input_signal.read().clone()));
                    }
                },
                class: "border-2 rounded-md w-full p-1",
                rows: 4,
            }
            button {
                r#type: "submit",
                class: "w-md border-4 rounded-3xl",
                onclick: move |_| tx.send(Action::UserInput(input_signal.read().clone())),

                "Send"
            }
        }
    }
}

#[component]
pub fn ui_page(state: LingoState) -> Element {
    match state {
        LingoState::UserInput => user_input(),
    }
}
