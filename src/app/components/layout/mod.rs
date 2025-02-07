use super::pages::{home_page, settings_page};
use leptos::prelude::*;

#[component]
pub fn layout() -> impl IntoView {
    view! {
        <home_page::HomePage />
    }
}

#[component]
pub fn mobile_layout() -> impl IntoView {
    view! {}
}

#[component]
pub fn desktop_layout() -> impl IntoView {
    view! {}
}
