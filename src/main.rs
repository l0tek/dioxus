use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        main {
            style: "padding: 24px; font-family: sans-serif;",
            h1 { "Dioxus Android App" }
            p { "Die App wurde erfolgreich erstellt." }
        }
    }
}
