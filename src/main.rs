use dioxus::{
    desktop::{Config, LogicalSize, WindowBuilder},
    prelude::*,
};
use dioxus_logger::tracing::{debug, info, Level};
use lingo::lingo_container;

mod lingo;

const STYLE_CSS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/target/styles/style.css"
));

fn app() -> Element {
    debug!("render app");
    rsx! {
        style { "{STYLE_CSS}" }
        div { class: "w-full h-screen overflow-hidden z-10 bg-red-100", lingo_container {} }
    }
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    info!("starting app 1");

    LaunchBuilder::desktop()
        .with_cfg(
            Config::default().with_window(
                WindowBuilder::default()
                    .with_title("Oops Lingo")
                    .with_focused(false)
                    .with_inner_size(LogicalSize::new(500, 700))
                    .with_theme(None),
            ),
        )
        .launch(app);
}
