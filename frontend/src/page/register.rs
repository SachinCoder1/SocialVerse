use dioxus::prelude::*;

use crate::{components::button::Button, sync_handler};

pub struct PageState {
    username: UseState<String>,
    password: UseState<String>,
}

impl PageState {
    pub fn new(cx: Scope) -> Self {
        Self {
            username: use_state(cx, String::new).clone(),
            password: use_state(cx, String::new).clone(),
        }
    }
}

#[inline_props]
pub fn UsernameInput<'a>(
    cx: Scope<'a>,
    state: UseState<String>,
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            label {
                r#for: "username",
                "Username"
            },
            input {
                id: "username",
                name: "username",
                class: "input",
                placeholder: "e.g. johndoe",
                value:   "{state.current()}" ,
                oninput: move |e| oninput.call(e)
            }
        }
    })
}

#[inline_props]
pub fn PasswordInput<'a>(
    cx: Scope<'a>,
    state: UseState<String>,
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            label {
                r#for: "password",
                "password"
            },
            input {
                id: "password",
                name: "password",
                r#type: "password",
                class: "input",
                placeholder: "enter password",
                value:   "{state.current()}" ,
                oninput: move |e| oninput.call(e)
            }
        }
    })
}
pub fn Register(cx: Scope) -> Element {
    let page_state = PageState::new(cx);
    let page_state = use_ref(cx, || page_state);

    let username_oninput = sync_handler!([page_state], move |e: FormEvent| {
        page_state.with_mut(|state| state.username.set(e.value.clone()))
    });
    let password_oninput = sync_handler!([page_state], move |e: FormEvent| {
        page_state.with_mut(|state| state.password.set(e.value.clone()))
    });

    cx.render(rsx! {
        form {
            class: "flex flex-col gap-5 p-2",
            prevent_default: "onsubmit",
            onsubmit: move |_| {},

            UsernameInput {
                state: page_state.with(|state| state.username.clone()),
                oninput: username_oninput
            },
            PasswordInput {
                state: page_state.with(|state| state.password.clone()),
                oninput: password_oninput
            },

            Button {
                button_type: "submit",
                "Signup"
            }
    }
    })
}
