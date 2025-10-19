#[component]
fn HeaderLayout() -> Element {
    rsx! {
        <div class="header">
            <div class="logo">神农开心农场</div>
            <nav class="nav">
                <a href="/dashboard" class="nav-item">📊 数据监控</a>
                <a href="/farm-management" class="nav-item">🌾 农场管理</a>
                <a href="/user-management" class="nav-item">👥 用户管理</a>
                <a href="/system-settings" class="nav-item">⚙️ 系统设置</a>
            </nav>
            <div class="user-info">
                <span>管理员</span>
                <span>系统管理员</span>
            </div>
        </div>

        <div class="main-content">
            <Outlet::<Route> />  // ← 这里会渲染当前页面的内容
        </div>
    }
}