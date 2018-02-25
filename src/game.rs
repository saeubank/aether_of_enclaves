// use user_interface::{Interface};
use piston_window::*;
use player::Player;
use input_handler::InputHandler;

pub enum GameState {
    Title,
    Main,
    InMenu,
}
use self::GameState::*;

/**
	Implementation of the Game object.

	@field u_i The user interface.
*/
pub struct Game {
    input_hnd: InputHandler,
    player: Player,
    game_state: GameState,
}

impl Game {
    // Constructor of the Game.
    pub fn new() -> Self {
        Game {
            input_hnd: InputHandler::new(),
            player: Player::new(),
            game_state: GameState::Title,
        }
    }

    // The game loop.
    pub fn run(&mut self, window: &mut PistonWindow) {
        // while self.ui.update() {}

        while let Some(e) = window.next() {
            match e {
                Event::Input(Input::Button(args)) => {
                    self.input_hnd.handle_input(
                        args.button,
                        &mut self.player,
                        &mut self.game_state,
                    );
                }

                Event::Loop(Loop::Update(args)) => {
                    //Update Events
                }

                Event::Loop(Loop::Render(args)) => {
                    window.draw_2d(&e, |context, graphics| {
                        clear([0.0, 0.0, 0.0, 1.0], graphics); // Clears screen.
                        match self.game_state {
                            Main => {
                                // Draw Player.
                                let red = [1.0, 0.0, 0.0, 1.0];
                                let player_image = [self.player.x, self.player.y, 15.0, 15.0];
                                rectangle(red, player_image, context.transform, graphics);
                            }
                            Title => {}
                            _ => {}
                        }
                    });
                }
                _ => {}
            }
        }
    }
}
