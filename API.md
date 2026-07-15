# 前言

所有API都需要使用HTTPS协议请求，格式为JSON，需要添加请求头`Content-Type: application/json`

对于需要鉴权的部分，需要在请求头中添加`Authorization`字段，值为`Bearer <access_token>`

其中`access_token`为用户登录成功后返回的访问令牌

响应体总是包含`success`和`message`字段，`success`代表操作是否成功，`message`为人类可读的错误信息

# 用户认证

## 注册

根据用户输入的用户名和密码完成注册

```
POST /api/register
```

请求体
```json
{
  "username": string, // 用户名
  "password": string, // 密码
}
```

响应体
```json
{
  "success": boolean, // 是否成功
  "message": string, // 错误信息
}
```

## 登录

根据用户输入的用户名和密码完成登录，登录后保存`access_token`用于需要鉴权的操作

`permissions`为用户的权限，可能的值为`guest`和`manager`

```
POST /api/login
```

请求体
```json
{
  "username": string, // 用户名
  "password": string, // 密码
}
```

响应体
```json
{
  "success": boolean, // 是否成功
  "access_token": string, // 访问令牌
  "permissions": string, // 用户权限，可能的值为 guest 和 manager
  "message": string, // 错误信息
}
```

# 查询书籍

## 查询所有书籍

> *需要鉴权(普通用户)*

查询所有书籍，返回书籍列表

书籍量可能很多，避免频繁查询，应该本地缓存
```
GET /api/books?page=number
```
响应体
```json
{
  "success": boolean, // 是否成功
  "books": [
    {
      "id": number, // 书籍ID
      "title": string, // 书籍标题
      "author": string, // 作者
      "category": [string], // 分类
      "description": string?, // 描述
      "published_date": string?, // 出版日期
      "borrowed": boolean, // 是否已借出
      "expire_date": string?, // 到期日期
    }
  ],
  "message": string, // 错误信息
}
```

## 搜索书籍

> *需要鉴权(普通用户)*

根据关键字查询书籍，返回匹配的列表

`title`为书籍关键字

`author`为作者关键字

`category`为分类关键字

```
GET /api/books/search?title=string
GET /api/books/search?author=string
GET /api/books/search?category=string
```
响应体
```json
{
  "success": boolean, // 是否成功
  "books": [
    {
      "id": number, // 书籍ID
      "title": string, // 书籍标题
      "author": string, // 作者
      "category": [string], // 分类
      "description": string?, // 描述
      "published_date": string?, // 出版日期
      "borrowed": boolean, // 是否已借出
      "expire_date": string?, // 到期日期
    }
  ],
  "message": string, // 错误信息
}
```

# 书籍管理

## 添加书籍

> *需要鉴权(管理员)*

```
POST /api/books
```

请求体
```json
{
  "title": string, // 书籍标题
  "author": string, // 作者
  "category": [string], // 分类
  "description": string?, // 描述
  "published_date": string?, // 出版日期
}
```

响应体
```json
{
  "success": boolean, // 是否成功
  "message": string, // 错误信息
}
```

## 删除书籍

> *需要鉴权(管理员)*

```
DELETE /api/books/{id}
```

响应体
```json
{
  "success": boolean, // 是否成功
  "message": string, // 错误信息
}
```

# 用户管理

## 当前用户信息
> *需要鉴权(普通用户)*

```
GET /api/users/me
```
响应体
```json
{
  "success": boolean, // 是否成功
  "user": {
    "id": number, // 用户ID
    "username": string, // 用户名
    "phone": string?, // 手机号
    "email": string?, // 邮箱
    "borrowed_books": number, // 已借阅数量
  },
  "message": string, // 错误信息
}
```

## 查询所有用户

> *需要鉴权(管理员)*

书籍量可能很多，避免频繁查询，应该本地缓存

```
GET /api/users
```
响应体
```json
{
  "success": boolean, // 是否成功
  "users": [
    {
      "id": number, // 用户ID
      "username": string, // 用户名
      "phone": string?, // 手机号
      "email": string?, // 邮箱
      "borrowed_books": number, // 已借阅数量
    }
  ],
  "message": string, // 错误信息
}
```

## 搜索用户

> *需要鉴权(管理员)*

`keyword`为用户名

```
GET /api/users/{keyword}
```
响应体
```json
{
  "success": boolean, // 是否成功
  "users": [
    {
      "id": number, // 用户ID
      "username": string, // 用户名
      "phone": string?, // 手机号
      "email": string?, // 邮箱
      "borrowed_books": number, // 已借阅数量
    }
  ],
  "message": string, // 错误信息
}
```

## 删除用户

> *需要鉴权(管理员)*

```
DELETE /api/users/{id}
```

响应体
```json
{
  "success": boolean, // 是否成功
  "message": string, // 错误信息
}
```

## 修改密码

> *需要鉴权(管理员)*

若使用me作为id，代表修改当前用户的密码，仅需要当前用户鉴权

```
PUT /api/users/{id}/password
```

请求体
```json
{
  "password": string, // 新密码
}
```

响应体
```json
{
  "success": boolean, // 是否成功
  "message": string, // 错误信息
}
```

## 修改用户信息

> *需要鉴权(普通用户)*

```
PATCH /api/users/{id}
```

请求体
```json
{
  "phone": string, // 手机号
  "email": string, // 邮箱
}
```

响应体
```json
{
  "success": boolean, // 是否成功
  "message": string, // 错误信息
}
```


# 书籍借阅

## 借出书籍

> *需要鉴权(管理员)*

```
POST /api/books/{id}/borrow
```

请求体
```json
{
  "user_id": number, // 借阅用户ID
  "expire_date": string, // 到期日期
}
```

响应体
```json
{
  "success": boolean, // 是否成功
  "message": string, // 错误信息
}
```

## 归还书籍

> *需要鉴权(管理员)*

```
POST /api/books/{id}/return
```

请求体
```json
{
  "user_id": number, // 归还用户ID
}
```

响应体
```json
{
  "success": boolean, // 是否成功
  "message": string, // 错误信息
}
```

## 归还书籍

*需要鉴权(管理员)*

```
POST /api/books/{id}/return
```

请求体
```json
{
  "user_id": number, // 归还用户ID
}
```

响应体
```json
{
  "success": boolean, // 是否成功
  "message": string, // 错误信息
}
```

## 借阅记录

## 查询书籍借阅记录

> *需要鉴权(管理员)*

```
GET /api/books/{id}/borrow
```

响应体
```json
{
  "success": boolean, // 是否成功
  "borrow_records": [
    {
      "id": number, // 记录ID
      "user_id": number, // 用户ID
      "book_id": number, // 书籍ID
      "borrow_date": string, // 借阅日期
      "expire_date": string, // 到期日期
      "return_date": string?, // 归还日期
    }
  ],
  "message": string, // 错误信息
}
```

## 查询当前用户借阅记录

> *需要鉴权(普通用户)*

```
GET /api/users/me/borrow
```

响应体
```json
{
  "success": boolean, // 是否成功
  "borrow_records": [
    {
      "id": number, // 记录ID
      "user_id": number, // 用户ID
      "book_id": number, // 书籍ID
      "borrow_date": string, // 借阅日期
      "expire_date": string, // 到期日期
      "return_date": string?, // 归还日期
    }
  ],
  "message": string, // 错误信息
}
```

## 查询指定用户借阅记录

> *需要鉴权(管理员)*

```
GET /api/users/{id}/borrow
```

响应体
```json
{
  "success": boolean, // 是否成功
  "borrow_records": [
    {
      "id": number, // 记录ID
      "user_id": number, // 用户ID
      "book_id": number, // 书籍ID
      "borrow_date": string, // 借阅日期
      "expire_date": string, // 到期日期
      "return_date": string?, // 归还日期
    }
  ],
  "message": string, // 错误信息
}
```

## 查询过期借阅记录

> *需要鉴权(管理员)*

默认查询当前已过期的记录

若指定了`expired_after`参数，则查询指定日期之后已过期的记录

```
GET /api/books/borrow/expired?expired_after=YYYY-MM-DD
```

响应体
```json
{
  "success": boolean, // 是否成功
  "borrow_records": [
    {
      "id": number, // 记录ID
      "user_id": number, // 用户ID
      "book_id": number, // 书籍ID
      "borrow_date": string, // 借阅日期
      "expire_date": string, // 到期日期
      "return_date": string?, // 归还日期
    }
  ],
  "message": string, // 错误信息
}
```

## 查询所有借阅记录

> *需要鉴权(管理员)*

借阅记录很多，避免频繁查询

```
GET /api/books/borrow
```

响应体
```json
{
  "success": boolean, // 是否成功
  "borrow_records": [
    {
      "id": number, // 记录ID
      "user_id": number, // 用户ID
      "book_id": number, // 书籍ID
      "borrow_date": string, // 借阅日期
      "expire_date": string, // 到期日期
      "return_date": string?, // 归还日期
    }
  ],
  "message": string, // 错误信息
}
```

# 续借申请

> *需要鉴权(普通用户)*

## 提交申请

```
POST /api/books/borrow/{id}/renew
```

请求体
```json
{
  "expired_after": string, // 续借后的新到期日期
}
```

响应体
```json
{
  "success": boolean, // 是否成功
  "message": string, // 错误信息
}
```

## 列出申请

> *需要鉴权*
> 若管理员则为所有申请，若普通用户则为当前用户待审核申请

```
GET /api/books/borrow/renewals
```

响应体
```json
{
  "success": boolean, // 是否成功
  "renewals": [
    {
      "id": number, // 申请ID
      "user_id": number, // 用户ID
      "book_id": number, // 书籍ID
      "request_date": string, // 申请日期
      "expired_after": string, // 续借后的新到期日期
      "status": string, // 审批状态，仅在用户查询时存在，管理员仅显示待审核申请，`pending` 表示待审核，`approved` 表示通过，`rejected` 表示拒绝
    }
  ],
  "message": string, // 错误信息
}
```

## 审批申请

> *需要鉴权(管理员)*

`approved`: 审批结果，`true` 表示通过，`false` 表示拒绝

```
POST /api/books/borrow/renewals/{id}/approve
```

请求体
```json
{
  "approved": boolean, // 审批结果，true 表示通过，false 表示拒绝
}
```

响应体
```json
{
  "success": boolean, // 是否成功
  "message": string, // 错误信息
}
```
