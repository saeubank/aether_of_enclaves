struct Ship {
    // Vec of tiles
    vel_x: f64,
    vel_y: f64,
    health: i32,
    // have cannons and sub rooms here?
}

impl Ship {
    fn new() -> Self {
        Ship {
            10,
            10
        }
    }
    fn update_position() {}
    fn change_velocity(&mut self, dx: f64, dy: f64) {
        this.vel_x += dx;
        this.vel_y += dy;
    }
    fn change_health(&mut self, change: i32) { this.health += change; }
    fn update_tile() {}
    // figure out how to do chest storage and inventory stuff
}
