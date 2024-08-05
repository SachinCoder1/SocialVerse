use dioxus::prelude::*;

use crate::components::button::Button;

pub fn Register(cx: Scope) -> Element {
    cx.render(rsx! {
        form {
            class: "flex flex-col gap-5",
            prevent_default: "onsubmit",
            onsubmit: move |_| {},

            Button {
                button_type: "submit",
                "Signup"
            }
    }
    })
}
