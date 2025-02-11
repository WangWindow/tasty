use leptos::prelude::*;

#[component]
pub fn sider() -> impl IntoView {
    view! {
        <div class="sider-container h-full flex flex-col border-r border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800">
            <div class="flex-grow overflow-y-auto">
                <History />
            </div>
            <div class="flex-shrink-0 border-t border-gray-200 dark:border-gray-700">
                <Settings />
            </div>
        </div>
    }
}

#[component]
pub fn history() -> impl IntoView {
    view! {
        <div class="history-list p-4 space-y-2">
            <h2 class="text-lg font-medium mb-4">历史记录</h2>
            <div class="task-item flex items-center p-2 rounded cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700">
                <div class="task-icon mr-3">
                    <i class="fas fa-message"></i>
                </div>
                <div class="task-content">
                    <div class="task-title font-medium">任务 1</div>
                    <div class="task-time text-sm text-gray-500">2小时前</div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn settings() -> impl IntoView {
    view! {
        <div class="settings-container p-4">
            <button class="w-full flex items-center space-x-3 p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-700">
                <div class="avatar w-8 h-8 rounded-full bg-gray-300">
                </div>
                <div class="user-info flex-grow text-left">
                    <div class="username font-medium">用户名</div>
                </div>
                <div class="settings-icon">
                    <i class="fas fa-cog"></i>
                </div>
            </button>
        </div>
    }
}
