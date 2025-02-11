use leptos::prelude::*;

#[component]
pub fn content() -> impl IntoView {
    view! {
        <main class="flex-1 p-4 overflow-auto">
            <div class="p-4 bg-gray-100 dark:bg-gray-800 rounded h-full text-gray-900 dark:text-gray-100">
                内容
            </div>
        </main>
    }
}
