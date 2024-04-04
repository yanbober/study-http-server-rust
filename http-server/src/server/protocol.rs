use std::{collections::HashMap, io::Write, net::TcpStream};

pub enum HttpMethod {
    GET,
    POST
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            _ => {
                eprintln!("HttpMethod {} not support, please impl it, default to GET.", value);
                HttpMethod::GET
            }
        }
    }
}

pub enum ContentType {
    Plain,
    Json,
    OctetStream,
}

impl From<String> for ContentType {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "text/plain" => ContentType::Plain,
            "text/json" => ContentType::Json,
            "application/octet-stream" => ContentType::OctetStream,
            _ => {
                eprintln!("ContentType {} not support, default to text/plain.", value);
                ContentType::Plain
            }
        }
    }
}

// HTTP/1.1 请求协议实现
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl TryFrom<String> for HttpRequest {
    type Error = HttpError;

    fn try_from(request: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = request.split("\r\n\r\n").collect();
        if parts.len() < 2 {
            return Err(HttpError::InvalidFormat(request));
        }

        let head_parts = parts[0];
        let body_parts = parts[1];

        let head_lines: Vec<&str> = head_parts.split("\r\n").collect();
        let request_tokens: Vec<&str> = head_lines.first().unwrap_or(&"").split(" ").collect();
        let mut headers_map: HashMap<String, String> = HashMap::new();
        for kv_line in head_lines.iter().skip(1) {
            if let Some((k, v)) = kv_line.split_once(":") {
                headers_map.insert(k.trim().to_string(), v.trim().to_string());
            }
        }

        Ok(HttpRequest {
            method: HttpMethod::from(request_tokens[0]),
            path: request_tokens[1].to_string(),
            headers: headers_map,
            body: body_parts.to_string(),
        })
    }
}

// HTTP/1.1 响应协议实现
pub struct HttpResponse {
    pub status: u16,
    pub content: String,
    pub content_type: ContentType,
    pub content_length: usize,
}

impl HttpResponse {
    pub fn new(status: u16, content: &str, content_type: ContentType) -> Self {
        HttpResponse { status, content: content.to_string(), content_type, content_length: content.len() }
    }

    pub fn response(res: HttpResponse, stream: &mut TcpStream) -> Result<String, HttpError> {
        let mut str_buf = String::new();
        str_buf.push_str("HTTP/1.1 ");

        match res.status {
            200 => str_buf.push_str("200 OK\r\n"),
            201 => str_buf.push_str("201 OK\r\n"),
            404 => str_buf.push_str("404 Not Found\r\n"),
            _ => return Err(HttpError::UnsupportStatus(res.status.to_string())),
        }

        match res.content_type {
            ContentType::Plain => str_buf.push_str("Content-Type: text/plain\r\n"),
            ContentType::Json => str_buf.push_str("Content-Type: text/json\r\n"),
            ContentType::OctetStream => {
                str_buf.push_str("Content-Type: application/octet-stream\r\n")
            }
        }

        str_buf.push_str(format!("Content-Length: {}\r\n\r\n", res.content_length).as_str());

        str_buf.push_str(&res.content);

        stream.write_all(str_buf.as_bytes())?;
        Ok(str_buf)
    }
}

// 错误类型定义
#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("Invalid request format: {0}.")]
    InvalidFormat(String),
    #[error("Can't handle status code: {0}.")]
    UnsupportStatus(String),
    #[error("Can't handle content type: {0}.")]
    UnsupportContentType(String),
    #[error("io exception.")]
    IoError(#[from] std::io::Error),
    #[error("FromUtf8Error exception.")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Handle http: {0}.")]
    HttpHandleError(String),
}
