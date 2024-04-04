## study-http-server-rust

基于学习 Rust 实现最基本 HTTP 1.1 协议功能的 Http Server，代码量很少且非常适合 Rust 语言学习时用来实践。

项目工程采用最佳实践组织，代码结构如下：
```bash
| - study-http-server-rust 工程目录
|   | - http_server 抽象服务 crate
|   |   | - src/lib.rs  模块对外 API
|   |   | - src/server.rs  HTTP 1.1 服务管理
|   |   | - src/server/protocol.rs 基于 TCP 的 HTTP 1.1 协议实现
|   | - demo-app 业务模拟 crate
|   |   | - src/main.rs 模拟服务承载的业务路由实现
```

## 使用方法

### HTTP 1.1 服务端启动
1. github 拉取代码
2. 根目录运行`cargo run`或`cargo run --release`
3. 控制台看见`Http Server Started at 127.0.0.1:4221`日志表示服务启动成功

### 客户端访问服务端基于此服务实现的业务

    说明：你可以使用浏览器或 Postman 等工具，这里推荐使用 curl 命令行工具进行调试。

```bash
# 场景1：GET/POST 请求根服务进入欢迎词
curl http://127.0.0.1:4221 -v

# 场景2：GET/POST 请求 /user-agent 返回客户端的 User-Agent 信息
curl http://127.0.0.1:4221/user-agent -v

# 场景3：GET/POST 请求 /echo/YOU_DEFINED 返回请求链接的 YOU_DEFINED
curl http://127.0.0.1:4221/echo/bitdev -v

# 场景4：GET 请求 /files/readme.txt 返回服务端静态资源 readme.txt 内容
curl http://127.0.0.1:4221/files/readme.txt -v

# 场景5：POST 请求 /files/upload.txt 将 POST Boday 内容写入到服务端静态资源 /files/upload.txt 里面
curl http://127.0.0.1:4221/files/aaa -v -d dsfsfsfsgsg
```

## 交流讨论

添加微信：bitdev，一起成长。

<img src="static_res_dir/weixin.png" width="250" />
