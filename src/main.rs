use crate::server::Server;

mod player;
mod server;
mod thread_pool;
mod position;

fn main() {

    let mut server = Server::new("0.0.0.0:7878");
    server.accept_connections();
    server.handle_connections();
}
