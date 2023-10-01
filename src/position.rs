use std::net::{SocketAddr, SocketAddrV4};

pub struct Position{
    pub player_socket: SocketAddr,
    pub position: [f32; 2],
}
impl Position{
    pub fn new(position: [f32; 2], player_socket: SocketAddr) -> Self {
        Self{
            position,
            player_socket,
        }
    }
}