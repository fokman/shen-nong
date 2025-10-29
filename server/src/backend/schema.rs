// @generated automatically by Diesel CLI.

diesel::table! {
    data_scopes (id) {
        id -> Integer,
        tenant_id -> Integer,
        name -> Varchar,
        code -> Varchar,
        resource_type -> Varchar,
        scope_rules -> Text,
        description -> Nullable<Text>,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    permissions (id) {
        id -> Integer,
        tenant_id -> Integer,
        name -> Varchar,
        code -> Varchar,
        resource_type -> Varchar,
        resource_id -> Nullable<Integer>,
        action -> Varchar,
        description -> Nullable<Text>,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    role_permissions (id) {
        id -> Integer,
        role_id -> Integer,
        permission_id -> Integer,
        created_at -> Timestamp,
    }
}

diesel::table! {
    roles (id) {
        id -> Integer,
        tenant_id -> Integer,
        name -> Varchar,
        code -> Varchar,
        description -> Nullable<Text>,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tenants (id) {
        id -> Integer,
        name -> Varchar,
        code -> Varchar,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_data_scopes (id) {
        id -> Integer,
        user_id -> Integer,
        data_scope_id -> Integer,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_roles (id) {
        id -> Integer,
        user_id -> Integer,
        role_id -> Integer,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        tenant_id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(data_scopes -> tenants (tenant_id));
diesel::joinable!(permissions -> tenants (tenant_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(roles -> tenants (tenant_id));
diesel::joinable!(user_data_scopes -> data_scopes (data_scope_id));
diesel::joinable!(user_data_scopes -> users (user_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(users -> tenants (tenant_id));

diesel::allow_tables_to_appear_in_same_query!(
    data_scopes,
    permissions,
    role_permissions,
    roles,
    tenants,
    user_data_scopes,
    user_roles,
    users,
);