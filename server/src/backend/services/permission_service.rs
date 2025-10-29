use crate::models::permission::*;
use crate::db::DatabasePool;
use diesel::prelude::*;
use std::collections::HashSet;

pub struct PermissionService {
    pool: DatabasePool,
}

impl PermissionService {
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    /// 检查用户权限
    pub async fn check_permission(
        &self,
        request: &PermissionCheckRequest,
    ) -> Result<PermissionCheckResult, Box<dyn std::error::Error>> {
        let conn = &mut self.pool.get()?;
        
        // 获取用户的所有权限
        let user_permissions = self.get_user_permissions(request.user_id, conn)?;
        
        // 检查是否有匹配的权限
        let has_permission = user_permissions.iter().any(|perm| {
            perm.resource_type == request.resource_type &&
            perm.action == request.action &&
            (perm.resource_id.is_none() || perm.resource_id == request.resource_id)
        });
        
        // 获取数据过滤条件
        let data_filters = if has_permission {
            self.get_data_filters(request.user_id, &request.resource_type, conn).ok()
        } else {
            None
        };
        
        Ok(PermissionCheckResult {
            has_permission,
            data_filters,
            message: if has_permission {
                None
            } else {
                Some("权限不足".to_string())
            },
        })
    }

    /// 获取用户的所有权限（包括角色权限）
    fn get_user_permissions(
        &self,
        user_id: i32,
        conn: &mut PgConnection,
    ) -> Result<Vec<Permission>, diesel::result::Error> {
        use crate::schema::*;
        
        let permissions = permissions::table
            .inner_join(role_permissions::table.on(permissions::id.eq(role_permissions::permission_id)))
            .inner_join(user_roles::table.on(role_permissions::role_id.eq(user_roles::role_id)))
            .filter(user_roles::user_id.eq(user_id))
            .filter(permissions::status.eq(1)) // 只获取启用的权限
            .select(permissions::all_columns)
            .load::<Permission>(conn)?;
        
        Ok(permissions)
    }

    /// 获取用户的数据过滤条件
    fn get_data_filters(
        &self,
        user_id: i32,
        resource_type: &str,
        conn: &mut PgConnection,
    ) -> Result<String, diesel::result::Error> {
        use crate::schema::*;
        
        let data_scopes = data_scopes::table
            .inner_join(user_data_scopes::table.on(data_scopes::id.eq(user_data_scopes::data_scope_id)))
            .filter(user_data_scopes::user_id.eq(user_id))
            .filter(data_scopes::resource_type.eq(resource_type))
            .filter(data_scopes::status.eq(1))
            .select(data_scopes::scope_rules)
            .load::<String>(conn)?;
        
        // 合并所有数据范围规则
        let combined_rules = if data_scopes.is_empty() {
            "{}".to_string()
        } else {
            // 这里可以更复杂的规则合并逻辑
            data_scopes.join(",")
        };
        
        Ok(combined_rules)
    }

    /// 获取用户的完整权限信息
    pub fn get_user_with_permissions(
        &self,
        user_id: i32,
    ) -> Result<UserWithPermissions, Box<dyn std::error::Error>> {
        let conn = &mut self.pool.get()?;
        use crate::schema::*;
        
        // 获取用户信息
        let user = users::table
            .filter(users::id.eq(user_id))
            .first::<User>(conn)?;
        
        // 获取用户角色
        let roles = roles::table
            .inner_join(user_roles::table.on(roles::id.eq(user_roles::role_id)))
            .filter(user_roles::user_id.eq(user_id))
            .filter(roles::status.eq(1))
            .select(roles::all_columns)
            .load::<Role>(conn)?;
        
        // 获取用户权限
        let permissions = self.get_user_permissions(user_id, conn)?;
        
        // 获取用户数据范围
        let data_scopes = data_scopes::table
            .inner_join(user_data_scopes::table.on(data_scopes::id.eq(user_data_scopes::data_scope_id)))
            .filter(user_data_scopes::user_id.eq(user_id))
            .filter(data_scopes::status.eq(1))
            .select(data_scopes::all_columns)
            .load::<DataScope>(conn)?;
        
        Ok(UserWithPermissions {
            user,
            roles,
            permissions,
            data_scopes,
        })
    }

    /// 为用户分配角色
    pub fn assign_role_to_user(
        &self,
        user_id: i32,
        role_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = &mut self.pool.get()?;
        use crate::schema::user_roles;
        
        diesel::insert_into(user_roles::table)
            .values((user_roles::user_id.eq(user_id), user_roles::role_id.eq(role_id)))
            .execute(conn)?;
        
        Ok(())
    }

    /// 为角色分配权限
    pub fn assign_permission_to_role(
        &self,
        role_id: i32,
        permission_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = &mut self.pool.get()?;
        use crate::schema::role_permissions;
        
        diesel::insert_into(role_permissions::table)
            .values((role_permissions::role_id.eq(role_id), role_permissions::permission_id.eq(permission_id)))
            .execute(conn)?;
        
        Ok(())
    }

    /// 为用户分配数据范围
    pub fn assign_data_scope_to_user(
        &self,
        user_id: i32,
        data_scope_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = &mut self.pool.get()?;
        use crate::schema::user_data_scopes;
        
        diesel::insert_into(user_data_scopes::table)
            .values((user_data_scopes::user_id.eq(user_id), user_data_scopes::data_scope_id.eq(data_scope_id)))
            .execute(conn)?;
        
        Ok(())
    }
}