use leptos::prelude::*;

#[component]
pub fn header() -> impl IntoView {
    view! {
        // <!-- 顶部导航栏 -->
        <header class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
        // <!-- 侧边栏按钮，不显示文字，仅显示菜单图标 -->
        <button class="text-gray-500 dark:text-gray-300 hover:text-gray-700 dark:hover:text-white">
            <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M4 6h16M4 12h16M4 18h16" />
            </svg>
        </button>
        <h1 class="text-xl font-bold text-gray-900 dark:text-gray-100">标题</h1>
        // <!-- 新建按钮，不显示文字，仅显示加号图标 -->
        <button class="bg-blue-500 dark:bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-600 dark:hover:bg-blue-700">
            <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M12 4v16m8-8H4" />
            </svg>
        </button>
    </header>
    }
}
