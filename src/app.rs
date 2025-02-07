// pub mod example;
// use example::Example;

// #[component]
// pub fn App() -> impl IntoView {
//     example::Example()
// }

pub mod components;

use components::layout;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <layout::Layout />
    }
}
