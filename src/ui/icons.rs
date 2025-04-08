use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::{
        bs_icons::BsSendFill,
        md_image_icons::MdEdit,
        md_navigation_icons::{MdCancel, MdCheck, MdRefresh},
    },
    Icon, IconShape,
};

#[component]
fn BaseIcon<T: IconShape + Clone + PartialEq + 'static>(icon: T, class: Option<String>) -> Element {
    rsx! {
        Icon {
            class: "w-6 h-6 fill-stone-800 group-hover/button:fill-slate-800/80 group-active/button:fill-slate-800/50  group-active/button:scale-110 group-disabled/button:fill-slate-700/90",
            icon,
        }
    }
}

#[component]
pub fn SendIcon() -> Element {
    rsx! {
        BaseIcon { icon: BsSendFill }
    }
}

#[component]
pub fn CancelIcon() -> Element {
    rsx! {
        BaseIcon { icon: MdCancel }
    }
}

#[component]
pub fn RefreshIcon() -> Element {
    rsx! {
        BaseIcon { icon: MdRefresh }
    }
}

#[component]
pub fn EditIcon() -> Element {
    rsx! {
        BaseIcon { icon: MdEdit }
    }
}

#[component]
pub fn CheckIcon() -> Element {
    rsx! {
        BaseIcon { icon: MdCheck }
    }
}
