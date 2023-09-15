pub struct Position{
    pub player_id: u16,
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