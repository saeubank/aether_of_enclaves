
const SHIP_TILES: [[i32]] =
[
[0,0,0,0,1,1,1,0,0,0,0],
[0,0,0,1,1,1,1,1,0,0,0],
[0,0,0,1,1,1,1,1,0,0,0],
[0,0,1,1,1,1,1,1,1,0,0],
[0,1,1,1,1,1,1,1,1,1,0],
[1,1,1,1,1,1,1,1,1,1,1],
[1,1,1,1,1,1,1,1,1,1,1],
[1,0,1,1,1,1,1,1,1,0,1],
[0,0,1,1,1,1,1,1,1,0,0],
[0,0,0,0,1,1,1,0,0,0,0],
];

struct Ship {
    // Vec of tiles
    x: f64,
    y: f64,
    self_vel_x: f64,
    self_vel_y: f64,
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
        this.self_vel_x += dx;
        this.self_vel_y += dy;
    }
    fn change_health(&mut self, change: i32) { this.health += change; }
    fn update_tile() {}
    // figure out how to do chest storage and inventory stuff
}
