use std::fmt;
use std::fmt::Formatter;
use std::io::{Read, Write};
use std::net::{SocketAddr};
use crate::position::Position;

const BUFFER_SIZE: usize = 1024;
const MOVEMENT_SPEED: f64 = 0.05;
enum Colors{
    Red,
    Green,
    Blue,
}
#[repr(u8)]
enum PossibleMovements {
    NoInput = 0,

    Forward = 1,
    Backwards = 2,
    Left = 3,
    Right = 4,
}
pub struct Player {
    pub position: [f64; 2],
    color: Colors,
    socket_addr: SocketAddr,
}

impl Player {
    pub fn new(socket_addr: SocketAddr) -> Self {
        Player{
            position: [0.0, 0.0],
            color: Colors::Red,
            socket_addr,
        }
    }
    pub fn update_position(&mut self, buffer: &[u8]){
        for element in buffer {
            match element {
                1 => {
                    self.position[1] += MOVEMENT_SPEED;
                }
                2 => {
                    self.position[1] -= MOVEMENT_SPEED;
                }
                3 => {
                    self.position[0] -= MOVEMENT_SPEED;
                }
                4 => {
                    self.position[0] += MOVEMENT_SPEED;
                }
                _ => {}
            }
        }
    }
}
impl fmt::Display for Player{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "position: {:?}", self.position)
    }
}