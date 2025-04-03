use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, info, Level};

const STYLE_CSS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/target/styles/style.css"
));

fn app() -> Element {
    debug!("render app");
    rsx! {
        style { "{STYLE_CSS}" }
        div { class: "w-full h-screen overflow-hidden z-10", " content " }
    }
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app 1");

    LaunchBuilder::desktop().launch(app);
}
