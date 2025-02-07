use leptos::prelude::*;

#[component]
pub fn sider() -> impl IntoView {
    view! {
        <div class="sider-container h-full flex flex-col">
            <div class="flex-grow overflow-y-auto">
                <History />
            </div>
            <div class="flex-shrink-0">
                <Settings />
            </div>
        </div>
    }
}

#[component]
pub fn history() -> impl IntoView {
    view! {
        <div class="history-list p-4 space-y-2">
            <h2 class="text-lg font-medium mb-4">"历史记录"</h2>
            <div class="task-item flex items-center p-2 rounded cursor-pointer">
                <div class="task-icon mr-3">
                    <i class="fas fa-message"></i>
                </div>
                <div class="task-content">
                    <div class="task-title font-medium">"任务 1"</div>
                    <div class="task-time text-sm text-gray-500">"2小时前"</div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn settings() -> impl IntoView {
    view! {
        <div class="settings-container p-4">
            <button class="w-full flex items-center space-x-3 p-2 rounded">
                <div class="avatar w-8 h-8 rounded-full bg-gray-300">
                    // 可以替换为实际的用户头像
                </div>
                <div class="user-info flex-grow text-left">
                    <div class="username font-medium">"用户名"</div>
                </div>
                <div class="settings-icon">
                    <i class="fas fa-cog"></i>
                </div>
            </button>
        </div>
    }
}
