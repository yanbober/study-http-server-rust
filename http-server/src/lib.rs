pub mod server;
pub use server::protocol::{HttpError, HttpRequest, HttpResponse};


pub fn start(address: &str, port: u16, handle_http: server::HandleHttp) {
    server::start_http_server(address, port, handle_http);
}
