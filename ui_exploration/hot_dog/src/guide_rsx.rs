use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");
// static ICON: Asset = asset!("/assets/favicon.ico");

#[component]
pub(crate) fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
        div { id: "dogview",
            // img { src: ICON }
            img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
        }
        div { id: "buttons",
            button { id: "skip", "skip" }
            button { id: "save", "save!" }
        }
    }
}
