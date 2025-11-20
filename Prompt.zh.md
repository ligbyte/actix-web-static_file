# 新需求

请为该应用添加JWT Authentication支持，对所有的`/api/v1/*`路由进行保护，
只有携带有效JWT令牌的请求才能访问这些路由。请提供实现该功能的代码示例。

JWT的算法为HS256，密钥为`my_secret_key`。JWT令牌应包含用户的ID和过期时间(exp)声明。

# 新需求

请问该应用添加SQLite数据库支持，使用`Diesel`作为ORM工具，要求如下：

- 数据库文件名为`app.sqlite`，存储在`./db/`目录下。
- 请创建`account`表，包括典型的字段，如nick, password, email, phone, status, created_at, updated_at等，建表语句保存在
  `db/schema.ddl`文件中。
- 请提供创建数据库连接池的代码示例。
- 创建account_repo模块，包含基本的CRUD操作代码示例。
- 创建account_controller模块，包含处理HTTP请求的代码示例。
