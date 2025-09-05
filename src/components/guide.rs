use dioxus::prelude::*;

#[component]
pub fn GuideApp(breed: String) -> Element {
    rsx! {
        "Breed: {breed}"
    }
}
