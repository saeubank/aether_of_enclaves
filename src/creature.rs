
pub enum CreatureType {
    Player,
    // Fighter,
    // Cook,
    // Carpenter,
    // Monster,
}

/**
	Implementation of the Creature object.

	@field x Creature's horizontal position on screen.
	@field y Creature's vertical position on screen.
    @field self_vel_x Creature's horizontal velocity.
    @field self_vel_y Creature's vertical velocity.
    @field health Creature's health.
*/
pub struct Creature {
    pub creature_type: CreatureType,
    pub x: f64,
    pub y: f64,
    pub self_vel_x: f64,
    pub self_vel_y: f64,
    pub other_vel_x: f64,
    pub other_vel_y: f64,
    pub speed: f64,
    pub health: i32, // array size 3 for inventory can only use/drop top item
}

impl Creature {
    pub fn new(c_type: CreatureType) -> Creature {
        Creature {
            creature_type: c_type,
            x: 0.0,
            y: 0.0,
            self_vel_x: 0.0,
            self_vel_y: 0.0,
            other_vel_x: 0.0,
            other_vel_y: 0.0,
            speed: 5.0,
            health: 1,
        }
    }

    // Updates the player's position.
    // @param dir The direction player will move.
    // @param dist The distance in pixels to move.
    // pub fn update_position(&mut self, dir: Option<Direction>, dist: f64) {
    //     match dir {
    //         Some(Direction::Up) => self.y -= dist,
    //         Some(Direction::Down) => self.y += dist,
    //         Some(Direction::Left) => self.x -= dist,
    //         Some(Direction::Right) => self.x += dist,
    //         _ => {}
    //     }
    // }

    pub fn update_position(&mut self) {
        self.x += self.self_vel_x;
        self.y += self.self_vel_y;
    }

    // velocity should be based on both what the creature is on and the actual movement of the creature
    pub fn change_self_velocity(&mut self, dx: f64, dy: f64) {
        self.self_vel_x += dx;
        self.self_vel_y += dy;

        if self.self_vel_x > self.speed {
            self.self_vel_x = self.speed;
        }
        if self.self_vel_y > self.speed {
            self.self_vel_y = self.speed;
        }
        if self.self_vel_x < -self.speed {
            self.self_vel_x = -self.speed;
        }
        if self.self_vel_y < -self.speed {
            self.self_vel_y = -self.speed;
        }
    }

    pub fn reset_self_velocity_x(&mut self) {
        self.self_vel_x = 0.0;
    }
    pub fn reset_self_velocity_y(&mut self) {
        self.self_vel_y = 0.0;
    }    
    // fn interact()
}

// Maybe make so some types of creatures hate some other type
// Fighting within same species of creatures?
