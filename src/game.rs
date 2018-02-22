use user_interface::{Interface};
use player::{Player};

pub enum GameState { Title, Main, InMenu }
use self::GameState::*;

// Global Game State variable.
pub static mut GAME_STATE: GameState = Title;
pub static mut PLAYER: Player = Player { x: 0.0, y: 0.0 };

/**
	Implementation of the Game object.

	@field u_i The user interface.
*/
pub struct Game { u_i: Interface, }

impl Game {
	// Constructor of the Game.
	pub fn new() -> Self {
		Game { u_i: Interface::new(), }
	}

	// The game loop.
	pub fn run(&mut self) {
		while self.u_i.update() { }
	}
}