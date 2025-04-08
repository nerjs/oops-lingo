use std::usize;

use dioxus::prelude::*;
use dioxus_logger::tracing::debug;

use crate::{
    lingo::state::LingoState,
    ui::{
        card::{
            Card, CardBody, CardContent, CardControlButton, CardControls, CardControlsSeparator,
            CardFooter, CardGroup, CardGroupHeader, CardHeader,
        },
        icons::{CancelIcon, CheckIcon, EditIcon, RefreshIcon},
        input::CardInput,
    },
};

use super::{
    state::{ProcessingState, StateInfo},
    worker::Action,
};

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

fn card_block_edited(
    text: String,
    onsubmit: EventHandler<String>,
    oncanceledit: EventHandler,
) -> Element {
    let mut text_signal = use_signal(|| text.clone());
    let mut disabled_submit = use_signal(|| false);

    let cloned_text = text.clone();
    use_effect(move || {
        let cloned_text = cloned_text.to_owned();
        let text_signal = text_signal.read().clone();
        let edited_text = text_signal.trim();
        if cloned_text == edited_text {
            disabled_submit.set(true);
        } else {
            disabled_submit.set(false);
        }
    });

    rsx! {
        CardInput {
            input: text_signal,
            placeholder: "Type in the text here",
            minrows: 1,
            autofocus: true,
            disabled_submit: *disabled_submit.read(),
            onsubmit: move |_| onsubmit.call(text_signal.read().clone()),

            CardControlButton {
                onclick: move |_| {
                    let text = text.to_owned();
                    text_signal.set(text.clone());
                    oncanceledit.call(());
                },
                CancelIcon {}
            }

            CardControlsSeparator {}
        }
    }
}

fn card_block_showed(
    text: String,
    can_edit: bool,
    onstartedit: EventHandler,
    children: Element,
) -> Element {
    rsx! {

        CardBody {
            CardContent {
                for line in text.lines() {
                    "{line}"
                    br {}
                }
            }
            CardControls {
                CardControlButton {
                    disabled: !can_edit,
                    onclick: move |_| {
                        if can_edit {
                            onstartedit.call(());
                        }
                    },
                    EditIcon {}
                }
                CardControlsSeparator {}

                {children}
            }
        }
    }
}

#[component]
fn CardBlock(
    text: String,
    editable: bool,
    can_edit: bool,
    onstartedit: EventHandler,
    onsubmit: EventHandler<String>,
    oncanceledit: EventHandler,
    children: Element,
) -> Element {
    if editable {
        card_block_edited(text, onsubmit, oncanceledit)
    } else {
        card_block_showed(text, can_edit, onstartedit, children)
    }
}

#[component]
fn UserInputBlock(
    text: String,
    editable: bool,
    can_edit: bool,
    onsubmit: EventHandler<String>,
    onrefresh: EventHandler,
    onstartedit: EventHandler,
    oncanceledit: EventHandler,
) -> Element {
    rsx! {
        Card {
            CardHeader { "User input" }
            CardBlock {
                text,
                editable,
                can_edit,
                onstartedit,
                oncanceledit,
                onsubmit,

                CardControlButton { onclick: move |_| onrefresh.call(()), RefreshIcon {} }
            }
        }
    }
}

fn interpritation_options(user_input: String, items: Vec<String>, loading: bool) -> Element {
    let mut user_input_edited_signal = use_signal(|| false);
    let mut current_item_edit_signal = use_signal::<i8>(|| -1);

    use_effect(move || {
        if *user_input_edited_signal.read() {
            current_item_edit_signal.set(-1);
        }
    });

    let user_input_edited = *user_input_edited_signal.read();
    let current_item_edit = *current_item_edit_signal.read() as usize;
    let can_item_edit = !loading && !user_input_edited;

    rsx! {
        CardGroup {
            UserInputBlock {
                text: user_input,
                editable: !loading && user_input_edited,
                can_edit: !loading,
                onstartedit: move |_| user_input_edited_signal.set(true),
                oncanceledit: move |_| user_input_edited_signal.set(false),
                onsubmit: move |text| {
                    user_input_edited_signal.set(false);
                    debug!("user input submit: {text}")
                },
                onrefresh: move |_| debug!("refresh"),
            }


            CardGroupHeader { title: "Interpritation options" }

            for (index , item) in items.into_iter().enumerate() {
                Card {
                    CardBlock {
                        text: item.clone(),
                        editable: can_item_edit && current_item_edit == index,
                        can_edit: can_item_edit,
                        onstartedit: move |_| current_item_edit_signal.set(index as i8),
                        oncanceledit: move |_| current_item_edit_signal.set(-1),
                        onsubmit: move |_| {
                            current_item_edit_signal.set(-1);
                            debug!("item submit");
                        },

                        CardControlButton {
                            disabled: !can_item_edit,
                            onclick: move |_| debug!("item select"),
                            CheckIcon {}
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ui_page(state: StateInfo) -> Element {
    match state.state {
        LingoState::UserInput => user_input(state.loading),
        LingoState::Processing {
            user_input,
            processing_state,
        } => match processing_state {
            ProcessingState::InterpretationOptions(items) => {
                interpritation_options(user_input, items, state.loading)
            }
        },
    }
}
