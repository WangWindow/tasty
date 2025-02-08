use super::super::common;
use super::super::layout;
use leptos::prelude::*;

#[component]
pub fn home_page() -> impl IntoView {
    view! {
        <div class="flex flex-col h-screen bg-white dark:bg-gray-900">
            <common::Header />

            <common::Content />

            <common::Footer />
        </div>
    }
}
