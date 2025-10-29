## 权限模型设计方案

### 1. 核心概念设计

**多租户架构**：
- 每个租户拥有独立的数据隔离
- 租户间数据完全隔离
- 支持租户级别的自定义配置

**RBAC模型**：
- 用户(User) - 角色(Role) - 权限(Permission)三层结构
- 支持角色继承和权限组合
- 动态权限分配

**数据权限**：
- 基于数据范围的访问控制
- 支持行级和列级数据权限
- 动态数据过滤

### 2. 数据库表结构设计

让我为您创建完整的数据库模型：
现在创建权限服务层：
现在创建权限中间件和API控制器：
现在创建权限API控制器：
现在创建数据库迁移文件：
现在创建数据库schema文件：
现在创建数据库连接和模块导出文件：
现在创建模块导出文件：
我已经为您设计并实现了一个完整的通用权限模型，结合了RBAC、数据权限和多租户架构。以下是完整的方案总结：

## 🎯 核心特性

### 1. **多租户架构**
- 每个租户数据完全隔离
- 支持租户级别的自定义配置
- 租户间权限和数据完全独立

### 2. **RBAC模型**
- 用户(User) - 角色(Role) - 权限(Permission)三层结构
- 支持动态权限分配和角色继承
- 灵活的权限组合机制

### 3. **数据权限**
- 基于数据范围的访问控制
- 支持行级和列级数据权限
- 动态数据过滤条件

## 📁 项目结构

```
server/
├── src/
│   ├── models/permission.rs          # 数据模型定义
│   ├── services/permission_service.rs # 权限服务层
│   ├── middleware/permission_middleware.rs # 权限中间件
│   ├── controllers/permission_controller.rs # API控制器
│   ├── db.rs                         # 数据库连接
│   ├── schema.rs                     # 数据库schema
│   └── lib.rs                        # 模块导出
├── migrations/                       # 数据库迁移文件
└── Cargo.toml
```

## 🗄️ 数据库表结构

1. **tenants** - 租户表
2. **users** - 用户表（关联租户）
3. **roles** - 角色表（关联租户）
4. **permissions** - 权限表（关联租户）
5. **user_roles** - 用户角色关联表
6. **role_permissions** - 角色权限关联表
7. **data_scopes** - 数据范围表
8. **user_data_scopes** - 用户数据范围关联表

## 🔧 核心功能

### 权限检查
```rust
// 检查用户是否有特定权限
permission_service.check_permission(&request).await
```

### 数据权限过滤
```rust
// 获取用户的数据过滤条件
middleware.get_data_filters(user_id, resource_type).await
```

### 权限管理界面
- 用户权限信息展示
- 实时权限检查演示
- 角色和权限分配功能

## 🚀 使用示例

```rust
// 权限检查
let result = permission_service.check_permission(&PermissionCheckRequest {
    user_id: 1,
    resource_type: "order".to_string(),
    action: "read".to_string(),
    resource_id: Some(123),
}).await?;

if result.has_permission {
    // 执行操作
} else {
    // 权限不足处理
}
```

## 📊 数据权限规则示例

```json
{
  "type": "department",
  "field": "department_id"
}
```

支持的数据权限类型：
- **all_data** - 所有数据
- **department_data** - 部门数据
- **personal_data** - 个人数据

## 🔒 安全特性

- 租户数据完全隔离
- 权限验证中间件
- 数据范围自动过滤
- 防止越权访问

这个权限模型设计灵活、可扩展，能够满足企业级应用的复杂权限需求。您可以根据具体业务场景进一步定制和扩展。