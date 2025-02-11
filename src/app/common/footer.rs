use leptos::prelude::*;

#[component]
pub fn footer() -> impl IntoView {
    view! {
        <footer class="p-4 border-t border-gray-200 dark:border-gray-700">
            <input
                type="text"
                placeholder="输入消息..."
                class="w-full px-4 py-2 border rounded bg-white dark:bg-gray-700 border-gray-300 dark:border-gray-600 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
        </footer>
    }
}
