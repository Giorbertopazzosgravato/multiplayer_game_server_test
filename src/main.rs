use crate::server::Server;

mod player;
mod server;
mod thread_pool;
mod position;

fn main() {

    let mut server = Server::new();
    server.handle_connections();
    server.send_updated_positions();
}
