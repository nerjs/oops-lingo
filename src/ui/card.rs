use dioxus::{document::Title, prelude::*};

#[component]
pub fn CardGroup(children: Element) -> Element {
    rsx! {
        div { class: "w-full max-h-full overflow-auto flex flex-col relative gap-1 px-1.5",
            {children}
        }
    }
}

#[component]
pub fn CardGroupHeader(title: String) -> Element {
    rsx! {
        Title { {title.clone()} }
        h2 { class: "sticky top-0 bg-slate-200/65 text-slate-800 p-1 align-middle text-lg font-bold border-b-2 border-slate-400",
            "{title}"
        }
    }
}

#[component]
pub fn Card(children: Element) -> Element {
    rsx! {
        div { class: "w-full px-1.5 py-1 bg-slate-400 text-slate-900 flex flex-col rounded-md text-sm gap-1",
            {children}
        }
    }
}

#[component]
pub fn CardHeader(children: Element) -> Element {
    rsx! {
        h3 { class: "font-bold text-sm text-slate-800 pl-3", {children} }
    }
}

#[component]
pub fn CardBody(children: Element) -> Element {
    rsx! {
        div { class: "w-full h-auto flex flex-1 flex-row gap-2 rounded-md", {children} }
    }
}

#[component]
pub fn CardFooter(children: Element) -> Element {
    rsx! {
        div { class: "text-xs", {children} }
    }
}

#[component]
pub fn CardContent(children: Element) -> Element {
    rsx! {
        div { class: "flex-1 bg-slate-100 rounded-l-md", {children} }
    }
}

#[component]
pub fn CardControls(children: Element) -> Element {
    rsx! {
        div { class: "w-8 min-w-8 rounded-r-md flex flex-col gap-1 justify-end", {children} }
    }
}

#[component]
pub fn CardControlButton(
    children: Element,
    onclick: EventHandler<Event<MouseData>>,
    disabled: Option<bool>,
) -> Element {
    rsx! {
        button {
            class: "w-7 h-7 ursor-pointer flex justify-center items-center group/button disabled:opacity-30 disabled:cursor-default",
            onclick,
            disabled,

            {children}
        }
    }
}

#[component]
pub fn CardControlsSeparator() -> Element {
    rsx! {
        div { class: "flex-1" }
    }
}
