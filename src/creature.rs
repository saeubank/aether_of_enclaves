use input_handler::Direction;

enum CreatureType {
    Player,
    Fighter,
    Cook,
    Carpenter,
    Monster,
}

/**
	Implementation of the Player object.

	@field x Player's horizontal position on screen.
	@field y Player's vertical position on screen.
*/
pub struct Creature {
    pub x: f64,
    pub y: f64,
    // array size 3 for inventory can only use/drop top item
}

impl Creature {
    pub fn new() -> Creature {
        Creature { x: 0.0, y: 0.0 }
    }

    // Updates the player's position.
    // @param dir The direction player will move.
    // @param dist The distance in pixels to move.
    pub fn update_position(&mut self, dir: Option<Direction>, dist: f64) {
        match dir {
            Some(Direction::Up) => self.y -= dist,
            Some(Direction::Down) => self.y += dist,
            Some(Direction::Left) => self.x -= dist,
            Some(Direction::Right) => self.x += dist,
            _ => {}
        }
    }
    // fn interact()
}

// Maybe make so some types of creatures hate some other type
// Fighting within same species of creatures?
