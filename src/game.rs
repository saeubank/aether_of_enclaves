use user_interface;


/**
	Implementation of the Game object.

	@field u_i The user interface.
*/
pub struct Game {
	u_i: user_interface::Interface,
}

impl Game {
	// Constructor of the Game.
	pub fn new() -> Self {
		Game {
			u_i: user_interface::Interface::new(),
		}
	}

	// The game loop.
	pub fn run(&mut self) {
		while self.u_i.update() {

		}
	}
}