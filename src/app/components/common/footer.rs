use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[component]
pub fn footer() -> impl IntoView {
    view! {
        <footer>
            <h1>"Footer Component"</h1>
        </footer>
    }
}
