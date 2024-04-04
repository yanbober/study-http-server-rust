use std::path::Path;
use std::io::Read;
use std::io::Write;
use http_server::{server::protocol::{ContentType, HttpMethod, HttpRequest, HttpResponse}, HttpError};

// 一个请求整包数据小于 1024 字节的模拟 HTTP/1.1 协议服务实现
// 此案例用来巩固 Rust 学习，基本未使用标准库外其他库，结合理解计算机传统网络协议核心原理
// 加微信：bitdev 进行交流
fn main() {
    http_server::start("127.0.0.1", 4221, router_to_handle_request);
}

// 模拟基于 http server 的业务路由处理实现
fn router_to_handle_request(request: &HttpRequest) -> Result<HttpResponse, HttpError> {
    if request.path.starts_with("/files/") {
        let file_name = request.path.strip_prefix("/files/").unwrap_or_default();
        let static_res_dir = "./static_res_dir";
        let file_path = Path::new(static_res_dir).join(file_name);
        match request.method {
            HttpMethod::GET => {
                if !Path::new(static_res_dir).exists() {
                    return Ok(HttpResponse::new(404, "dir not found", ContentType::Json));
                }

                if !file_path.exists() {
                    return Ok(HttpResponse::new(404, "not found", ContentType::Json));
                } else {
                    let file = std::fs::File::open(file_path);
                    match file {
                        Ok(mut file) => {
                            let mut content = String::new();
                            file.read_to_string(&mut content)?;
                            return Ok(HttpResponse::new(200, content.as_str(), ContentType::OctetStream));
                        }
                        Err(_) => {
                            return Ok(HttpResponse::new(404, "not found", ContentType::Json));
                        }
                    }
                }
            }
            HttpMethod::POST => {
                let mut f = std::fs::File::create(file_path)?;
                f.write_all(request.body.as_bytes())?;
                return Ok(HttpResponse::new(201, "created", ContentType::Plain));
            }
        }
    } else if request.path.starts_with("/echo/") {
        let end_str = request.path.strip_prefix("/echo/").unwrap_or_default();
        return Ok(HttpResponse::new(200, end_str, ContentType::Json));
    } else if request.path == "/" {
        return Ok(HttpResponse::new(200, "加微信：bitdev 进行交流.", ContentType::Plain));
    } else if request.path == "/user-agent" {
        let default = &"unknown".to_string();
        let ua: &str = request.headers.get("User-Agent").unwrap_or(default);
        return Ok(HttpResponse::new(200, ua, ContentType::Json));
    }

    return Ok(HttpResponse::new(404, "unsupport", ContentType::Json));
}
