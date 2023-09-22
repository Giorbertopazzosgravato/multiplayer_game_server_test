use std::net::SocketAddr;

pub struct Position{
    pub player_socket: SocketAddr,
    pub position: [f32; 2],
}
impl Position{
    pub fn new(player_id: u16, position: [f32; 2]) -> Self {
        Self{
            player_id,
            position,
        }
    }
}