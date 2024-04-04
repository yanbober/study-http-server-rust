pub mod protocol;

use std::{io::Read, net::{TcpListener, TcpStream}, thread};
use crate::{HttpError, HttpRequest, HttpResponse};

pub type HandleHttp = fn(&HttpRequest) -> Result<HttpResponse, HttpError>;

// 启动 Http 服务，基于 TCP 实现
pub fn start_http_server(address: &str, port: u16, handle_http: HandleHttp) {
    let listener = TcpListener::bind(format!("{}:{}", address, port));
    match listener {
        Ok(listener) => {
            println!("Http Server Started at {}:{}", address, port);
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        let handle_http = handle_http.clone();
                        // 多线程并发处理，可以同时处理多个 http 请求
                        let _ = thread::spawn(move || {
                            handle_stream_connect_default(&mut stream, handle_http);
                        });
                    }
                    Err(e) => {
                        eprintln!("Accept new connection error:{}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Start Http Server Error:{}", e);
        }
    }
}

// 处理一个 http 连接请求，兜底容错处理
fn handle_stream_connect_default(tcp_stream: &mut TcpStream, handle_http: HandleHttp) {
    let result = handle_stream_connect(tcp_stream, handle_http);
    if result.is_err() {
        eprintln!("handle_stream_connect error:{}", result.err().unwrap());
    }
}

// 处理一个 http 连接请求
fn handle_stream_connect(
    tcp_stream: &mut TcpStream,
    handle_http: HandleHttp
) -> Result<(), HttpError> {
    println!(
        "Accepted a new connection from {}.",
        tcp_stream.peer_addr()?
    );
    // 读取整个 http 请求内容放入 buf
    let mut buf = [0; 1024];
    tcp_stream.read(&mut buf)?;

    let null_index = buf.iter().position(|&c| c == b'\0').unwrap_or(buf.len());
    let raw_string: String = String::from_utf8(buf[0..null_index].to_vec())?;
    // 把整个用户请求体按照 http 协议约定解析成 HttpRequest 对象
    let request = HttpRequest::try_from(raw_string)?;
    //Http Server 对外给用户自定义的路由实现层
    let response = handle_http(&request)?;
    // 把 HttpResponse 对象按照 http 协议约定包装成返回信息返回
    HttpResponse::response(response, tcp_stream)?;
    Ok(())
}
