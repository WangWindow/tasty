use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[component]
pub fn header() -> impl IntoView {
    view! {
        <header>
            <h1>"Header Component"</h1>
        </header>
    }
}
