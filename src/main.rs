mod components;

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        components::GuideApp { breed: "corgi" }
    }
}
