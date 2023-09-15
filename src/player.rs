use std::fmt;
use std::fmt::Formatter;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use crate::position::Position;

const BUFFER_SIZE: usize = 1024;
const MOVEMENT_SPEED: f32 = 0.05;
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
    pub position: [f32; 2],
    color: Colors,
    stream: TcpStream,
    socket_addr: SocketAddr,
    pub player_id: u16,
}

impl Player {
    pub fn new(stream: TcpStream, socket_addr: SocketAddr, player_id: u16) -> Self {
        Player{
            position: [0.0, 0.0],
            color: Colors::Red,
            stream,
            socket_addr,
            player_id
        }
    }
    pub fn get_input(&mut self) -> anyhow::Result<()> {
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let size = match self.stream.read(&mut buffer){
            Ok(size) => {
                //println!("size: {size}");
                size
            }
            Err(err) => {
                return Err(err.into());
            }
        }; // this will crash the program if someone disconnects
        // it doesnt anymore because I am the greatest programmer ever
        // this function crashes the computer directly
        let buffer = &buffer[0..size];
        Self::update_position(self, buffer);
        Ok(())
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
    pub fn send_position(&mut self, player_position: &[Position]){
        for position in player_position {
            if position.player_id != self.player_id {
                match self.stream.write(&position.position[0].to_be_bytes()) {
                    Ok(_) => {
                        self.stream.write(&position.position[1].to_be_bytes()).unwrap();
                    }
                    Err(_) => {}
                }
            }
        }
    }
}
impl fmt::Display for Player{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "addr: {:?}", self.socket_addr.ip())
    }
}