use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = tenants)]
pub struct Tenant {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub status: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub tenant_id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub status: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: i32,
    pub tenant_id: i32,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub status: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = permissions)]
pub struct Permission {
    pub id: i32,
    pub tenant_id: i32,
    pub name: String,
    pub code: String,
    pub resource_type: String,
    pub resource_id: Option<i32>,
    pub action: String,
    pub description: Option<String>,
    pub status: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = user_roles)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = role_permissions)]
pub struct RolePermission {
    pub id: i32,
    pub role_id: i32,
    pub permission_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = data_scopes)]
pub struct DataScope {
    pub id: i32,
    pub tenant_id: i32,
    pub name: String,
    pub code: String,
    pub resource_type: String,
    pub scope_rules: String, // JSON格式的规则定义
    pub description: Option<String>,
    pub status: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = user_data_scopes)]
pub struct UserDataScope {
    pub id: i32,
    pub user_id: i32,
    pub data_scope_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

// 插入结构体
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = tenants)]
pub struct NewTenant {
    pub name: String,
    pub code: String,
    pub status: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub tenant_id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub status: i32,
}

// 权限相关的DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithPermissions {
    pub user: User,
    pub roles: Vec<Role>,
    pub permissions: Vec<Permission>,
    pub data_scopes: Vec<DataScope>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionCheckRequest {
    pub user_id: i32,
    pub resource_type: String,
    pub resource_id: Option<i32>,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionCheckResult {
    pub has_permission: bool,
    pub data_filters: Option<String>, // 数据过滤条件
    pub message: Option<String>,
}