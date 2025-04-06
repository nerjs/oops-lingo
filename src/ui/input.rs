use dioxus::prelude::*;

use crate::ui::{
    card::{CardBody, CardContent, CardControlButton, CardControls},
    icons::SendIcon,
};

#[component]
pub fn CardInput(
    input: Signal<String>,
    onsubmit: Option<EventHandler>,
    minrows: Option<u8>,
    maxrows: Option<u8>,
    disabled: Option<bool>,
    disabled_submit: Option<bool>,
    placeholder: Option<String>,
    children: Element,
) -> Element {
    let rows = use_signal(|| minrows.unwrap_or(4));
    let disabled = disabled.unwrap_or(false);
    let disabled_submit = disabled || disabled_submit.unwrap_or(false);

    use_effect(move || {
        let minrows = minrows.unwrap_or(4) as usize;
        let maxrows = maxrows.unwrap_or(100) as usize;
        let mut count = input.read().clone().lines().count() + 1;
        let mut rows = rows.to_owned();
        if count < minrows {
            count = minrows;
        } else if count > maxrows {
            count = maxrows;
        }

        rows.set(count as u8);
    });

    rsx! {
        CardBody {
            CardContent {
                textarea {
                    class: "w-full outline-0 border-0 rounded-md resize-none text-lg px-2 py-1 font-normal text-slate-900 disabled:opacity-90 disabled:text-slate-800/90",
                    value: "{input}",
                    rows,
                    disabled,
                    placeholder,

                    oninput: move |e| input.set(e.value()),
                    onkeypress: move |e| {
                        if e.code() == Code::Enter && e.modifiers() == Modifiers::CONTROL
                            && !disabled_submit
                        {
                            e.prevent_default();
                            if let Some(handler) = onsubmit {
                                handler.call(());
                            }
                        }
                    },
                }
            }
            CardControls {
                {children}
                CardControlButton {
                    disabled: disabled_submit,
                    onclick: move |_| {
                        if !disabled_submit {
                            if let Some(handler) = onsubmit {
                                handler.call(());
                            }
                        }
                    },


                    SendIcon {}
                }
            }
        }
    }
}
