use crate::services::permission_service::PermissionService;
use crate::models::permission::*;
use dioxus::prelude::*;
use serde_json::json;

/// 权限管理API控制器
pub struct PermissionController {
    permission_service: PermissionService,
}

impl PermissionController {
    pub fn new(permission_service: PermissionService) -> Self {
        Self { permission_service }
    }

    /// 检查权限API
    #[server]
    pub async fn check_permission(
        user_id: i32,
        resource_type: String,
        action: String,
        resource_id: Option<i32>,
    ) -> Result<PermissionCheckResult, ServerFnError> {
        let permission_service = PermissionService::new(crate::db::get_pool());
        let request = PermissionCheckRequest {
            user_id,
            resource_type,
            resource_id,
            action,
        };

        permission_service
            .check_permission(&request)
            .await
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    }

    /// 获取用户权限信息API
    #[server]
    pub async fn get_user_permissions(user_id: i32) -> Result<UserWithPermissions, ServerFnError> {
        let permission_service = PermissionService::new(crate::db::get_pool());
        permission_service
            .get_user_with_permissions(user_id)
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    }

    /// 分配角色给用户API
    #[server]
    pub async fn assign_role(user_id: i32, role_id: i32) -> Result<(), ServerFnError> {
        let permission_service = PermissionService::new(crate::db::get_pool());
        permission_service
            .assign_role_to_user(user_id, role_id)
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    }

    /// 分配权限给角色API
    #[server]
    pub async fn assign_permission(role_id: i32, permission_id: i32) -> Result<(), ServerFnError> {
        let permission_service = PermissionService::new(crate::db::get_pool());
        permission_service
            .assign_permission_to_role(role_id, permission_id)
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    }

    /// 分配数据范围给用户API
    #[server]
    pub async fn assign_data_scope(user_id: i32, data_scope_id: i32) -> Result<(), ServerFnError> {
        let permission_service = PermissionService::new(crate::db::get_pool());
        permission_service
            .assign_data_scope_to_user(user_id, data_scope_id)
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    }

    /// 权限管理界面组件
    #[component]
    pub fn PermissionManagement() -> Element {
        let user_id = 1; // 从登录信息获取
        let user_permissions = use_resource(move || PermissionController::get_user_permissions(user_id));

        rsx! {
            div {
                class: "permission-management",
                
                h1 { "权限管理系统" }
                
                // 用户权限信息
                div {
                    class: "user-permissions",
                    h2 { "当前用户权限" }
                    
                    match user_permissions.value() {
                        Some(Ok(permissions)) => rsx! {
                            div {
                                p { "用户名: {permissions.user.username}" }
                                p { "邮箱: {permissions.user.email}" }
                                
                                h3 { "角色:" }
                                ul {
                                    for role in &permissions.roles {
                                        li { "{role.name} ({role.code})" }
                                    }
                                }
                                
                                h3 { "权限:" }
                                ul {
                                    for perm in &permissions.permissions {
                                        li { "{perm.name} - {perm.resource_type}.{perm.action}" }
                                    }
                                }
                            }
                        },
                        Some(Err(e)) => rsx! { p { "加载失败: {e}" } },
                        None => rsx! { p { "加载中..." } },
                    }
                }
                
                // 权限检查演示
                div {
                    class: "permission-check-demo",
                    h2 { "权限检查演示" }
                    
                    PermissionCheckDemo {}
                }
            }
        }
    }

    /// 权限检查演示组件
    #[component]
    fn PermissionCheckDemo() -> Element {
        let user_id = 1;
        let resource_type = use_signal(|| "order".to_string());
        let action = use_signal(|| "read".to_string());
        let resource_id = use_signal(|| Some(1));
        let check_result = use_signal(|| None);

        let check_permission = move |_| {
            spawn(async move {
                let result = PermissionController::check_permission(
                    user_id,
                    resource_type(),
                    action(),
                    resource_id(),
                ).await;
                check_result.set(Some(result));
            });
        };

        rsx! {
            div {
                class: "permission-check-form",
                
                div {
                    label { "资源类型:" }
                    input {
                        r#type: "text",
                        value: "{resource_type}",
                        oninput: move |e| resource_type.set(e.value())
                    }
                }
                
                div {
                    label { "操作:" }
                    input {
                        r#type: "text",
                        value: "{action}",
                        oninput: move |e| action.set(e.value())
                    }
                }
                
                div {
                    label { "资源ID (可选):" }
                    input {
                        r#type: "number",
                        value: "{resource_id().unwrap_or(0)}",
                        oninput: move |e| {
                            let id: i32 = e.value().parse().unwrap_or(0);
                            resource_id.set(if id > 0 { Some(id) } else { None })
                        }
                    }
                }
                
                button {
                    onclick: check_permission,
                    "检查权限"
                }
                
                match check_result() {
                    Some(Ok(result)) => rsx! {
                        div {
                            class: if result.has_permission { "permission-granted" } else { "permission-denied" },
                            
                            if result.has_permission {
                                p { "✅ 权限检查通过" }
                            } else {
                                p { "❌ 权限不足: {result.message.unwrap_or_default()}" }
                            }
                            
                            if let Some(filters) = result.data_filters {
                                p { "数据过滤条件: {filters}" }
                            }
                        }
                    },
                    Some(Err(e)) => rsx! { p { "检查失败: {e}" } },
                    None => rsx! {},
                }
            }
        }
    }
}