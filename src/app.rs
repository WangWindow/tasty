// pub mod example;
// use example::Example;

// #[component]
// pub fn App() -> impl IntoView {
//     example::Example()
// }

pub mod common;
pub mod layout;
pub mod pages;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <pages::HomePage />
    }
}
