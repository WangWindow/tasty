use super::super::common;
use leptos::prelude::*;

#[component]
pub fn home_page() -> impl IntoView {
    let (sidebar_open, set_sidebar_open) = signal(false);

    view! {
        <div class="relative">
            <div class="flex flex-col h-screen bg-white dark:bg-gray-900">
                <common::Header on_sidebar_toggle=move || set_sidebar_open.set(true) />
                <common::Content />
                <common::Footer />
            </div>

            <Show
                when=move || sidebar_open.get()
                fallback=|| view! { <div></div> }
            >
                <div
                    class="fixed inset-0 bg-black bg-opacity-50 transition-opacity duration-300 ease-in-out z-40"
                    on:click=move |_| set_sidebar_open.set(false)
                >
                </div>
            </Show>

            <div
                class=move || {
                    let base_classes = "fixed top-0 left-0 h-full w-80 transform transition-transform duration-300 ease-in-out z-50";
                    if sidebar_open.get() {
                        format!("{} translate-x-0", base_classes)
                    } else {
                        format!("{} -translate-x-full", base_classes)
                    }
                }
            >
                <common::Sider />
            </div>
        </div>
    }
}
