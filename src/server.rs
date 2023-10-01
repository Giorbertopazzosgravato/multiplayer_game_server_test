use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::player::Player;
use crate::thread_pool::ThreadPool;
use crate::position::Position;

const BUFFER_SIZE: usize = 8;

pub struct Server {
    players: Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<Player>>>>>,
    listener: Arc<Mutex<UdpSocket>>,
    //thread_pool: ThreadPool,
    positions: Arc<Mutex<Box<Vec<Position>>>>,
}
impl Server {
    pub fn new(port: &str) -> Self {
        let listener = UdpSocket::bind(port).unwrap();
        Self{
            players: Arc::new(Mutex::new(HashMap::new())),
            listener: Arc::new(Mutex::new(listener)),
            // thread_pool: ThreadPool::new(10),
            positions: Arc::new(Mutex::new(Box::new(vec![])))
        }
    }
    pub fn handle_connections(&mut self) {
        let socket = Arc::clone(&self.listener);
        let players = Arc::clone(&self.players);
        let handle = thread::spawn(move ||{
            loop {
                let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
                let mut src: SocketAddr = SocketAddr::new(IpAddr::from_str("192.168.0.0").unwrap(), 7878);
                {
                    let socket = socket.lock().unwrap();
                    (_, src) = socket.recv_from(&mut buf).unwrap();
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
        handle.join().unwrap();
    }
    fn send_updated_positions(&mut self){
        let threadPool = ThreadPool::new(10);
        let hashmap = self.players.lock().unwrap();
        let sources = hashmap.keys();
        for source in sources {
            threadPool.execute(||{})
        }
    }
}