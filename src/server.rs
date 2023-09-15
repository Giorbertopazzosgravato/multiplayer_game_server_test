use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::player::Player;
use crate::thread_pool::ThreadPool;
use crate::position::Position;

pub struct Server {
    players: Arc<Mutex<Vec<Arc<Mutex<Player>>>>>,
    listener: Arc<Mutex<TcpListener>>,
    thread_pool: ThreadPool,
    positions: Arc<Mutex<Box<Vec<Position>>>>,
}
impl Server {
    pub fn new(port: &str) -> Self {
        let listener = TcpListener::bind(port).unwrap();
        Self{
            players: Arc::new(Mutex::new(vec![])),
            listener: Arc::new(Mutex::new(listener)),
            thread_pool: ThreadPool::new(10),
            positions: Arc::new(Mutex::new(Box::new(vec![])))
        }
    }
    pub fn accept_connections(&mut self) {
        let player_clone = Arc::clone(&self.players);
        let listener_clone = Arc::clone(&self.listener);
        println!("help");
        thread::spawn(move ||{
            println!("accepting shit");
            let mut player_id: u16 = 0;
            while let Ok((stream, socket_addr)) = listener_clone.lock().unwrap().accept(){
                println!("accepted connection: {:?}", socket_addr.ip());
                player_clone.lock().unwrap().push(Arc::new(Mutex::new(Player::new(stream, socket_addr, player_id))));
                player_id += 1;
            }
        });
    }
    pub fn handle_connections(&mut self) {
        loop {
            let clone = Arc::clone(&self.players);
            let mut new_positions = Box::new(vec![]);
            {
                let mut clone = clone.lock().unwrap();
                let mut indices_to_remove = vec![];
                let mut index = 0;
                for player in clone.iter_mut() {
                    {
                        let mut player = player.lock().unwrap();
                        match player.get_input() {
                            Ok(_) => {
                                println!("player position: {:?}", player.position);
                                new_positions.push(Position::new(player.player_id, player.position));
                            }
                            Err(_) => { indices_to_remove.push(index) }
                        };
                    }
                    index += 1;
                }
                if indices_to_remove.len() > 0 {
                    for indices in indices_to_remove.iter().rev() {
                        println!("closed connection {}", clone.iter().nth(*indices as usize).unwrap().lock().unwrap());
                        clone.remove(*indices as usize);
                    }
                }
            }
            self.positions = Arc::new(Mutex::new(new_positions));
            Self::send_update_position(self);
        }
    }
    fn send_update_position(&mut self){
        let players = Arc::clone(&self.players);
        let mut players = players.lock().unwrap();
        for player in players.iter_mut() {
            let clone = Arc::clone(&self.positions);
            let player_clone = Arc::clone(&player);
            self.thread_pool.execute( move || {
                println!("sending updated position");
                let mut player = player_clone.lock().unwrap();
                let positions = clone.lock().unwrap();
                player.send_position(positions.as_slice());
            });
        }
    }
}