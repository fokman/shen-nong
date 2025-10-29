use crate::services::permission_service::PermissionService;
use crate::models::permission::PermissionCheckRequest;
use dioxus::prelude::*;
use std::collections::HashMap;

/// 权限检查中间件
pub struct PermissionMiddleware {
    permission_service: PermissionService,
}

impl PermissionMiddleware {
    pub fn new(permission_service: PermissionService) -> Self {
        Self { permission_service }
    }

    /// 检查API权限
    pub async fn check_api_permission(
        &self,
        user_id: i32,
        resource: &str,
        action: &str,
        resource_id: Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let request = PermissionCheckRequest {
            user_id,
            resource_type: resource.to_string(),
            resource_id,
            action: action.to_string(),
        };

        let result = self.permission_service.check_permission(&request).await?;
        Ok(result.has_permission)
    }

    /// 获取数据过滤条件
    pub async fn get_data_filters(
        &self,
        user_id: i32,
        resource_type: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let request = PermissionCheckRequest {
            user_id,
            resource_type: resource_type.to_string(),
            resource_id: None,
            action: "read".to_string(), // 默认使用read操作获取数据过滤
        };

        let result = self.permission_service.check_permission(&request).await?;
        Ok(result.data_filters)
    }
}

/// 权限守卫组件
#[component]
pub fn PermissionGuard<'a>(
    user_id: i32,
    resource: &'a str,
    action: &'a str,
    resource_id: Option<i32>,
    children: Element<'a>,
    fallback: Element<'a>,
) -> Element {
    let has_permission = use_resource(move || {
        let permission_service = PermissionService::new(crate::db::get_pool());
        let middleware = PermissionMiddleware::new(permission_service);
        async move {
            middleware
                .check_api_permission(user_id, resource, action, resource_id)
                .await
                .unwrap_or(false)
        }
    });

    match has_permission.value() {
        Some(true) => rsx! { {children} },
        Some(false) => rsx! { {fallback} },
        None => rsx! { div { "检查权限中..." } },
    }
}