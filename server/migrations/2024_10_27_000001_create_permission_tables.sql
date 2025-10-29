-- 创建权限系统相关表

-- 租户表
CREATE TABLE tenants (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    code VARCHAR(50) UNIQUE NOT NULL,
    status INTEGER NOT NULL DEFAULT 1, -- 1: 启用, 0: 禁用
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 用户表
CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    tenant_id INTEGER NOT NULL,
    username VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    status INTEGER NOT NULL DEFAULT 1, -- 1: 启用, 0: 禁用
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, username),
    UNIQUE(tenant_id, email),
    FOREIGN KEY (tenant_id) REFERENCES tenants(id)
);

-- 角色表
CREATE TABLE roles (
    id INT AUTO_INCREMENT PRIMARY KEY,
    tenant_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL,
    code VARCHAR(50) NOT NULL,
    description TEXT,
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, code),
    FOREIGN KEY (tenant_id) REFERENCES tenants(id)
);

-- 权限表
CREATE TABLE permissions (
    id INT AUTO_INCREMENT PRIMARY KEY,
    tenant_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL,
    code VARCHAR(100) NOT NULL,
    resource_type VARCHAR(50) NOT NULL, -- 资源类型: order, user, product等
    resource_id INTEGER, -- 具体资源ID，为空表示所有资源
    action VARCHAR(50) NOT NULL, -- 操作: create, read, update, delete等
    description TEXT,
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, code),
    FOREIGN KEY (tenant_id) REFERENCES tenants(id)
);

-- 用户角色关联表
CREATE TABLE user_roles (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (role_id) REFERENCES roles(id)
);

-- 角色权限关联表
CREATE TABLE role_permissions (
    id INT AUTO_INCREMENT PRIMARY KEY,
    role_id INTEGER NOT NULL,
    permission_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(role_id, permission_id),
    FOREIGN KEY (role_id) REFERENCES roles(id),
    FOREIGN KEY (permission_id) REFERENCES permissions(id)
);

-- 数据范围表
CREATE TABLE data_scopes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    tenant_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL,
    code VARCHAR(50) NOT NULL,
    resource_type VARCHAR(50) NOT NULL,
    scope_rules TEXT NOT NULL, -- JSON格式的规则定义
    description TEXT,
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, code),
    FOREIGN KEY (tenant_id) REFERENCES tenants(id)
);

-- 用户数据范围关联表
CREATE TABLE user_data_scopes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INTEGER NOT NULL,
    data_scope_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, data_scope_id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (data_scope_id) REFERENCES data_scopes(id)
);

-- 创建索引
CREATE INDEX idx_users_tenant_id ON users(tenant_id);
CREATE INDEX idx_users_status ON users(status);
CREATE INDEX idx_roles_tenant_id ON roles(tenant_id);
CREATE INDEX idx_permissions_tenant_id ON permissions(tenant_id);
CREATE INDEX idx_permissions_resource_type ON permissions(resource_type);
CREATE INDEX idx_data_scopes_tenant_id ON data_scopes(tenant_id);
CREATE INDEX idx_data_scopes_resource_type ON data_scopes(resource_type);

-- 插入默认数据
INSERT INTO tenants (name, code, status) VALUES 
('默认租户', 'default', 1);

-- 插入默认角色
INSERT INTO roles (tenant_id, name, code, description, status) VALUES 
(1, '超级管理员', 'super_admin', '系统超级管理员，拥有所有权限', 1),
(1, '管理员', 'admin', '系统管理员', 1),
(1, '普通用户', 'user', '普通用户', 1);

-- 插入默认权限
INSERT INTO permissions (tenant_id, name, code, resource_type, action, description) VALUES 
-- 用户管理权限
(1, '查看用户', 'user:read', 'user', 'read', '查看用户信息'),
(1, '创建用户', 'user:create', 'user', 'create', '创建新用户'),
(1, '修改用户', 'user:update', 'user', 'update', '修改用户信息'),
(1, '删除用户', 'user:delete', 'user', 'delete', '删除用户'),

-- 订单管理权限
(1, '查看订单', 'order:read', 'order', 'read', '查看订单信息'),
(1, '创建订单', 'order:create', 'order', 'create', '创建新订单'),
(1, '修改订单', 'order:update', 'order', 'update', '修改订单信息'),
(1, '删除订单', 'order:delete', 'order', 'delete', '删除订单'),

-- 产品管理权限
(1, '查看产品', 'product:read', 'product', 'read', '查看产品信息'),
(1, '创建产品', 'product:create', 'product', 'create', '创建新产品'),
(1, '修改产品', 'product:update', 'product', 'update', '修改产品信息'),
(1, '删除产品', 'product:delete', 'product', 'delete', '删除产品'),

-- 权限管理权限
(1, '管理权限', 'permission:manage', 'permission', 'manage', '管理系统权限');

-- 为超级管理员角色分配所有权限
INSERT INTO role_permissions (role_id, permission_id)
SELECT 1, id FROM permissions WHERE tenant_id = 1;

-- 插入默认数据范围
INSERT INTO data_scopes (tenant_id, name, code, resource_type, scope_rules, description) VALUES 
(1, '所有数据', 'all_data', 'order', '{"type": "all"}', '可以访问所有数据'),
(1, '部门数据', 'department_data', 'order', '{"type": "department", "field": "department_id"}', '只能访问本部门数据'),
(1, '个人数据', 'personal_data', 'order', '{"type": "personal", "field": "user_id"}', '只能访问个人数据');
