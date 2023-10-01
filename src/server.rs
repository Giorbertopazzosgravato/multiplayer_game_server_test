use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::player::Player;
use crate::thread_pool::ThreadPool;
use crate::position::Position;

const BUFFER_SIZE: usize = 1024;

pub struct Server {
    players: Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<Player>>>>>,
    listener: Arc<Mutex<UdpSocket>>,
    sender: Arc<Mutex<UdpSocket>>,
    //thread_pool: ThreadPool,
    positions: Arc<Mutex<Box<Vec<Position>>>>,
}
impl Server {
    pub fn new() -> Self {
        let listener = UdpSocket::bind("0.0.0.0:7878").unwrap();
        let sender = UdpSocket::bind("0.0.0.0:8800").unwrap();
        Self{
            players: Arc::new(Mutex::new(HashMap::new())),
            listener: Arc::new(Mutex::new(listener)),
            sender: Arc::new(Mutex::new(sender)),
            positions: Arc::new(Mutex::new(Box::new(vec![]))),
            // thread_pool: ThreadPool::new(10),
        }
    }
    pub fn handle_connections(&mut self) {
        let socket = Arc::clone(&self.listener);
        let players = Arc::clone(&self.players);
        thread::spawn(move || {
            loop {
                let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
                let mut src: SocketAddr = SocketAddr::new(IpAddr::from_str("192.168.0.0").unwrap(), 5768);
                {
                    let socket = socket.lock().unwrap();
                     let (size, src) = match socket.recv_from(&mut buf){
                        Ok((size, src )) => {
                            (size, src)
                        }
                        Err(error) => {
                            println!("{}", error.to_string());
                            panic!("dio cane");
                        }
                    };
                }
                {
                    let mut players = players.lock().unwrap();
                    match players.get(&src) {
                        None => {
                            println!("added player {src}");
                            players.insert(src, Arc::new(Mutex::new(Player::new(src))));
                        }
                        Some(player) => {
                            let mut player = player.lock().unwrap();
                            player.update_position(&buf);
                            println!("updated player position {src}; {}", player);
                        }
                    };
                }
            }
        });
    }
    pub fn send_updated_positions(&mut self){
        let thread_pool = ThreadPool::new(10);
        loop {
            let hashmap = self.players.lock().unwrap();
            let sources = hashmap.keys().cloned().collect::<Vec<_>>();
            for source in sources {
                let sender = Arc::clone(&self.sender);
                let source = source;
                thread_pool.execute(move || {
                    let source = source;
                    let sender = sender;
                    let sender = sender.lock().unwrap();
                    let dest = SocketAddr::new(source.ip(), 9045);
                    println!("destination: {:?}", dest.to_string());
                    println!("sender: {:?}", sender);
                    sender.send_to(&[0u8, 0u8, 0u8], dest).unwrap();
                    println!("sent buffer")
                })
            }
        }
    }
}