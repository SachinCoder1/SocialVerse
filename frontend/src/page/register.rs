use dioxus::prelude::*;

use crate::{
    components::button::Button,
    elements::keyed_notification_box::{KeyedNotificationBox, KeyedNotifications},
    sync_handler,
};

pub struct PageState {
    username: UseState<String>,
    password: UseState<String>,
    form_errors: KeyedNotifications,
}

impl PageState {
    pub fn new(cx: Scope) -> Self {
        Self {
            username: use_state(cx, String::new).clone(),
            password: use_state(cx, String::new).clone(),
            form_errors: KeyedNotifications::default(),
        }
    }

    pub fn can_submit(&self) -> bool {
        !(self.form_errors.has_messages()
            || self.username.current().is_empty()
            || self.password.current().is_empty())
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
        let username = socialverse_domain::Username::new(&e.value);
        match username {
            Ok(val) => page_state.with_mut(|state| state.form_errors.remove("bad-username")),
            Err(e) => {
                let error_message = format!("username: {}", e.to_string());
                page_state.with_mut(|state| state.form_errors.set("bad-username", error_message))
            }
        }
        page_state.with_mut(|state| state.username.set(e.value.clone()))
    });
    let password_oninput = sync_handler!([page_state], move |e: FormEvent| {
        let password = socialverse_domain::Password::new(&e.value);
        match password {
            Ok(val) => page_state.with_mut(|state| state.form_errors.remove("bad-password")),
            Err(e) => {
                let error_message = format!("password: {}", e.to_string());

                page_state.with_mut(|state| state.form_errors.set("bad-password", error_message))
            }
        }
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

            KeyedNotificationBox {
                legend: "Form Errors",
                notifications: page_state.clone().with(|state| state.form_errors.clone()),
            },

            Button {
                button_type: "submit",
                disabled: !page_state.with(|state| state.can_submit()),
                "Signup"
            }
    }
    })
}
