use super::super::common::sider::Sider;
use leptos::prelude::*;

#[component]
pub fn home_page() -> impl IntoView {
    view! {
        <main class="container">
            <Sider/>
        </main>
    }
}
